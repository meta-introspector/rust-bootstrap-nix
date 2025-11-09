// layered_crate_organizer.rs
// This module will be responsible for organizing the layered declarations into actual Rust crates.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use std::collections::BTreeSet; // Use BTreeSet for sorted unique elements
use prelude_generator::types::CollectedAnalysisData;
use code_graph_flattener::CodeGraph; // Add this import
use code_graph_flattener::perform_topological_sort; // Add this import
use tokio::io::AsyncWriteExt; // For writing to files asynchronously

#[derive(Debug)]
pub struct CrateProcessingSummary {
    pub crate_name: String,
    pub status: String, // "Success", "Failure"
    pub report_file: Option<PathBuf>,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct OrganizeLayeredDeclarationsInputs<'a> {
    #[allow(dead_code)]
    pub project_root: &'a Path,
    pub verbosity: u8,
    pub compile_flag: bool,
    pub canonical_output_root: &'a Path,
    pub top_level_cargo_toml_path: &'a Path,
    pub collected_analysis_data: CollectedAnalysisData,
    pub code_graph: CodeGraph, // Add this field
    pub topological_sort_output_path: Option<PathBuf>,
    pub per_file_report_dir: Option<PathBuf>,
}

/// Organizes the layered declarations into new Rust crates.
///
/// This function performs the following steps:
/// 1. Iterates through the `rust-bootstrap-core/src/level_XX` directories.
/// 2. For each `level_XX` directory, creates a `Cargo.toml` and `lib.rs` file to define it as a crate.
/// 3. The `lib.rs` will re-export the modules within that layer.
/// 4. Updates the `rust-bootstrap-core/Cargo.toml` to include these new `level_XX` crates as members of its workspace.
pub async fn organize_layered_declarations(inputs: OrganizeLayeredDeclarationsInputs<'_>) -> Result<Vec<CrateProcessingSummary>> {
    if inputs.verbosity >= 1 {
        println!("Starting organization of layered declarations into crates.");
    }

    // Print information about the CodeGraph
    if inputs.verbosity >= 1 {
        println!("CodeGraph received: {} nodes, {} edges",
                 inputs.code_graph.nodes.len(),
                 inputs.code_graph.edges.len());
    }

    // Perform topological sort
    if inputs.verbosity >= 1 {
        println!("Performing topological sort on the CodeGraph...");
    }
    let sorted_node_ids = perform_topological_sort(&inputs.code_graph)
        .context("Failed to perform topological sort on the CodeGraph")?;

    if inputs.verbosity >= 1 {
        println!("Topologically sorted nodes (first 10): {:?}", &sorted_node_ids[..std::cmp::min(10, sorted_node_ids.len())]);
        println!("Total sorted nodes: {}", sorted_node_ids.len());
    }

    // Serialize topological sort results to file if path is provided
    if let Some(ref path) = inputs.topological_sort_output_path {
        let serialized_sort = serde_json::to_string_pretty(&sorted_node_ids)
            .context("Failed to serialize topological sort results to JSON")?;
        fs::write(path, serialized_sort)
            .await
            .context(format!("Failed to write topological sort results to {}", path.display()))?;
        println!("Topological sort results successfully written to {}", path.display());
    }

    let rust_bootstrap_core_path = inputs.project_root.parent().unwrap().join("rust-bootstrap-core");
    let rust_bootstrap_core_src_path = rust_bootstrap_core_path.join("src");

    // Ensure per_file_report_dir exists if provided
    if let Some(ref path) = inputs.per_file_report_dir {
        fs::create_dir_all(path)
            .await
            .context(format!("Failed to create per-file report directory: {}", path.display()))?;
    }

    // Step 1: Find all level_XX directories
    let mut level_dirs = Vec::new();
    let mut entries = fs::read_dir(&rust_bootstrap_core_src_path)
        .await
        .context(format!("Failed to read directory: {}", rust_bootstrap_core_src_path.display()))?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                if dir_name.starts_with("level_") && dir_name.len() == "level_XX".len() && dir_name[6..].parse::<u32>().is_ok() {
                    level_dirs.push(path);
                }
            }
        }
    }

    level_dirs.sort(); // Ensure consistent order

    let mut workspace_members = BTreeSet::new();
    let mut processing_summaries = Vec::new(); // Collect summaries here

    // Step 2 & 3: Create Cargo.toml and lib.rs for each layer crate
    for level_dir in &level_dirs {
        let level_name = level_dir.file_name().unwrap().to_str().unwrap(); // e.g., "level_00"
        let crate_name = format!("rust-bootstrap-core-{}", level_name.replace("_", "-")); // e.g., "rust-bootstrap-core-level-00"
        let relative_path = level_dir.strip_prefix(&rust_bootstrap_core_path)
            .context("Failed to get relative path for level_dir")?;

        workspace_members.insert(relative_path.to_str().unwrap().to_string());

        let cargo_toml_path = level_dir.join("Cargo.toml");
        let lib_rs_path = level_dir.join("src").join("lib.rs");
        let layer_src_path = level_dir.join("src");

        // Create src directory inside level_XX if it doesn't exist
        fs::create_dir_all(&layer_src_path)
            .await
            .context(format!("Failed to create src directory for layer: {}", layer_src_path.display()))?;

        // Generate Cargo.toml content
        let cargo_toml_content = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
            crate_name
        );
        fs::write(&cargo_toml_path, cargo_toml_content)
            .await
            .context(format!("Failed to write Cargo.toml for {}: {}", crate_name, cargo_toml_path.display()))?;
        if inputs.verbosity >= 2 {
            println!("  Created Cargo.toml for crate: {}", crate_name);
        }

        // Generate lib.rs content (just a placeholder for now)
        let lib_rs_content = format!("//! This crate contains declarations for {}\n\n// TODO: Implement proper module re-exporting based on CollectedAnalysisData\n", level_name);

        fs::write(&lib_rs_path, lib_rs_content)
            .await
            .context(format!("Failed to write lib.rs for {}: {}", crate_name, lib_rs_path.display()))?;
        if inputs.verbosity >= 2 {
            println!("  Created lib.rs for crate: {}", crate_name);
        }

        // Compile the generated crate
        let mut summary = CrateProcessingSummary {
            crate_name: crate_name.clone(),
            status: "Failure".to_string(),
            report_file: None,
            error_message: None,
        };

        let mut command = tokio::process::Command::new("cargo");
        command.arg("check").current_dir(level_dir);

        let report_file_path = if let Some(ref dir) = inputs.per_file_report_dir {
            let file_name = format!("{}_report.txt", crate_name);
            let path = dir.join(file_name);
            summary.report_file = Some(path.clone());
            Some(path)
        } else {
            None
        };

        let output = if let Some(path) = report_file_path {
            let file = fs::File::create(&path).await
                .context(format!("Failed to create report file for {}: {}", crate_name, path.display()))?;
            let mut writer = tokio::io::BufWriter::new(file);

            let output = command.output().await
                .context(format!("Failed to run cargo check for crate: {}", crate_name))?;

            writer.write_all(b"Stdout:\n").await?;
            writer.write_all(&output.stdout).await?;
            writer.write_all(b"\nStderr:\n").await?;
            writer.write_all(&output.stderr).await?;
            writer.flush().await?;

            output
        } else {
            // If no report file, print to stderr/stdout based on verbosity
            if inputs.verbosity >= 1 {
                println!("  Compiling crate: {}", crate_name);
            }
            command.output().await
                .context(format!("Failed to run cargo check for crate: {}", crate_name))?
        };

        if output.status.success() {
            summary.status = "Success".to_string();
            if inputs.verbosity >= 1 && summary.report_file.is_none() {
                println!("  Compilation successful for crate: {}", crate_name);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            summary.error_message = Some(format!("Compilation failed. Stderr: {}", stderr));
            if inputs.verbosity >= 1 && summary.report_file.is_none() {
                eprintln!("Compilation failed for crate: {}", crate_name);
                eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
                eprintln!("Stderr:\n{}", stderr);
            }

            if !output.status.success() && inputs.compile_flag {
                return Err(anyhow::anyhow!("Compilation failed for crate: {}", crate_name));
            }
        }
        processing_summaries.push(summary);
    }

    // Step 4: Update the top-level Cargo.toml
    let cargo_toml_content = fs::read_to_string(&inputs.top_level_cargo_toml_path)
        .await
        .context(format!("Failed to read top-level Cargo.toml: {}", inputs.top_level_cargo_toml_path.display()))?;

    let mut toml_doc = cargo_toml_content.parse::<toml_edit::DocumentMut>()
        .context("Failed to parse top-level Cargo.toml")?;

    let mut members_array = toml_edit::Array::new();
    for member in workspace_members.iter() {
        members_array.push(member);
    }

    if let Some(workspace) = toml_doc.get_mut("workspace") {
        workspace["members"] = toml_edit::value(members_array);
    } else {
        // If no workspace section, create one
        toml_doc["workspace"] = toml_edit::table();
        toml_doc["workspace"]["members"] = toml_edit::value(members_array);
    }

    fs::write(&inputs.top_level_cargo_toml_path, toml_doc.to_string())
        .await
        .context(format!("Failed to write updated top-level Cargo.toml: {}", inputs.top_level_cargo_toml_path.display()))?;
    if inputs.verbosity >= 1 {
        println!("Updated top-level Cargo.toml with new workspace members.");
    }

    if inputs.verbosity >= 1 {
        println!("Finished organizing layered declarations into crates.");
    }

    Ok(processing_summaries)
}
