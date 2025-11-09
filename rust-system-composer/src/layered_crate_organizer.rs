// layered_crate_organizer.rs
// This module will be responsible for organizing the layered declarations into actual Rust crates.

use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;
use std::collections::BTreeSet; // Use BTreeSet for sorted unique elements
use prelude_generator::types::CollectedAnalysisData;
use code_graph_flattener::CodeGraph; // Add this import

#[derive(Debug)]
pub struct OrganizeLayeredDeclarationsInputs<'a> {
    #[allow(dead_code)]
    pub project_root: &'a Path,
    pub verbosity: u8,
    pub compile_flag: &'a str,
    pub canonical_output_root: &'a Path,
    pub top_level_cargo_toml_path: &'a Path,
    pub collected_analysis_data: CollectedAnalysisData,
    pub code_graph: CodeGraph, // Add this field
}

/// Organizes the layered declarations into new Rust crates.
///
/// This function performs the following steps:
/// 1. Iterates through the `rust-bootstrap-core/src/level_XX` directories.
/// 2. For each `level_XX` directory, creates a `Cargo.toml` and `lib.rs` file to define it as a crate.
/// 3. The `lib.rs` will re-export the modules within that layer.
/// 4. Updates the `rust-bootstrap-core/Cargo.toml` to include these new `level_XX` crates as members of its workspace.
pub async fn organize_layered_declarations(inputs: OrganizeLayeredDeclarationsInputs<'_>) -> Result<()> {
    if inputs.verbosity >= 1 {
        println!("Starting organization of layered declarations into crates.");
    }

    // Print information about the CodeGraph
    if inputs.verbosity >= 1 {
        println!("CodeGraph received: {} nodes, {} edges",
                 inputs.code_graph.nodes.len(),
                 inputs.code_graph.edges.len());
    }

    let rust_bootstrap_core_path = inputs.canonical_output_root.join("rust-bootstrap-core");
    let rust_bootstrap_core_src_path = rust_bootstrap_core_path.join("src");

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

        // Generate lib.rs content (re-exporting modules)
        let mut lib_rs_content = String::new();
        lib_rs_content.push_str(&format!("//! This crate contains declarations for {}\n\n", level_name));

        // Find all declaration_type directories within this level_XX/src
        let mut declaration_type_dirs = Vec::new();
        let mut layer_src_entries = fs::read_dir(&layer_src_path)
            .await
            .context(format!("Failed to read layer src directory: {}", layer_src_path.display()))?;

        while let Some(entry) = layer_src_entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                    if dir_name.ends_with("_t") { // e.g., "const_t", "struct_t"
                        declaration_type_dirs.push(path);
                    }
                }
            }
        }
        declaration_type_dirs.sort();

        for decl_type_dir in &declaration_type_dirs {
            let decl_type_name = decl_type_dir.file_name().unwrap().to_str().unwrap(); // e.g., "const_t"
            lib_rs_content.push_str(&format!("pub mod {};\n", decl_type_name));

            // Find all crate_name directories within this level_XX/src/declaration_type
            let mut crate_name_dirs = Vec::new();
            let mut decl_type_entries = fs::read_dir(&decl_type_dir)
                .await
                .context(format!("Failed to read declaration type directory: {}", decl_type_dir.display()))?;

            while let Some(entry) = decl_type_entries.next_entry().await? {
                let path = entry.path();
                if path.is_dir() {
                if let Some(_dir_name) = path.file_name().and_then(|s| s.to_str()) {
                        crate_name_dirs.push(path);
                    }
                }
            }
            crate_name_dirs.sort();

            for crate_dir in &crate_name_dirs {
                let inner_crate_name = crate_dir.file_name().unwrap().to_str().unwrap();
                lib_rs_content.push_str(&format!("pub mod {};\n", inner_crate_name));

                // Find all .rs files within this level_XX/src/declaration_type/crate_name
                let mut rs_files = Vec::new();
                let mut crate_entries = fs::read_dir(&crate_dir)
                    .await
                    .context(format!("Failed to read inner crate directory: {}", crate_dir.display()))?;

                while let Some(entry) = crate_entries.next_entry().await? {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                            if file_name.ends_with(".rs") {
                                rs_files.push(path);
                            }
                        }
                    }
                }
                rs_files.sort();

                for rs_file in &rs_files {
                    let module_name = rs_file.file_stem().unwrap().to_str().unwrap();
                    lib_rs_content.push_str(&format!("pub mod {};\n", module_name));
                }
            }
        }

        fs::write(&lib_rs_path, lib_rs_content)
            .await
            .context(format!("Failed to write lib.rs for {}: {}", crate_name, lib_rs_path.display()))?;
        if inputs.verbosity >= 2 {
            println!("  Created lib.rs for crate: {}", crate_name);
        }

        // Compile the generated crate
        if inputs.verbosity >= 1 {
            println!("  Compiling crate: {}", crate_name);
        }
        let output = tokio::process::Command::new("cargo")
            .arg("check") // Or "build"
            .current_dir(level_dir)
            .output()
            .await
            .context(format!("Failed to run cargo check for crate: {}", crate_name))?;

        if !output.status.success() {
            eprintln!("Compilation failed for crate: {}", crate_name);
            eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));

            if inputs.compile_flag == "fail" {
                return Err(anyhow::anyhow!("Compilation failed for crate: {}", crate_name));
            }
        } else if inputs.verbosity >= 1 {
            println!("  Compilation successful for crate: {}", crate_name);
        }
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

    Ok(())
}
