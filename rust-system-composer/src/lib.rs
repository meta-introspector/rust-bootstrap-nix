use anyhow::Context;
use std::path::PathBuf;
use tokio::process::Command; // Temporarily keep for cargo metadata/vendor until replaced
use expanded_code_collector::collect_expanded_code;
use std::collections::HashMap;
use walkdir::WalkDir;
use prelude_generator::types::CollectedAnalysisData;
use code_graph_flattener::CodeGraph;
use clap::Parser; // Add this import

mod cli;
use cli::{CliArgs}; // Correctly import Commands


mod config;
// use config::Config; // Remove this line

mod layered_crate_organizer;
 // Removed `self`

mod system_config;
use system_config::{SystemConfig, ProjectInfo, GeneratedProject};

mod traits; // Add this line

pub async fn run_self_composition_workflow_lib(config: &crate::config::Config, args: &CliArgs) -> anyhow::Result<()> {
    let project_root = std::env::current_dir()?;
    let metadata_file = project_root.join("rust-bootstrap-core/full_metadata.json");
    let expanded_dir = project_root.join("expanded");

    let main_project_root = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let canonical_output_root = main_project_root.join("generated");
    tokio::fs::create_dir_all(&canonical_output_root)
        .await
        .context(format!("Failed to create canonical output root directory: {}", canonical_output_root.display()))?;

    // 1. Run cargo metadata
    println!("Collecting full workspace metadata using cargo metadata...");
    std::fs::create_dir_all(metadata_file.parent().unwrap())?;

    // Correct way to handle output redirection for cargo metadata
    let output = Command::new(&config.rust.cargo)
        .args(&["metadata", "--format-version", "1"])
        .output().await?;

    if output.status.success() {
        std::fs::write(&metadata_file, &output.stdout)
            .context(format!("Failed to write metadata to {}", metadata_file.display()))?;
        println!("Metadata collected to {}.", metadata_file.display());
    } else {
        eprintln!("cargo metadata failed.");
        eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("cargo metadata failed"));
    }

    // 2. Run expanded-code-collector
    collect_expanded_code(
        &metadata_file,
        &expanded_dir,
        &serde_json::json!({}), // flake_lock_json
        Some(0), // layer
        None,    // package_filter
        false,   // dry_run
        false,   // force
        config.rust.rustc_version.clone(),
        config.rust.rustc_host.clone(),
    ).await?;

    // 3. Run split-expanded-bin
    println!("Running split-expanded-bin...");
    let expanded_manifest_path = expanded_dir.join("expanded_manifest.json");
    let rustc_info = split_expanded_lib::RustcInfo {
        version: config.rust.rustc_version.clone(),
        host: config.rust.rustc_host.clone(),
    };
    let warnings_from_split_expanded_lib = split_expanded_lib::process_expanded_manifest(
        split_expanded_lib::ProcessExpandedManifestInputs {
            expanded_manifest_path: &expanded_manifest_path,
            project_root: &project_root,
            rustc_info: &rustc_info,
            verbosity: args.verbosity,
            layer: Some(0),
            canonical_output_root: &canonical_output_root,
            package_filter: args.package_filter.clone(), // Pass the package filter here
        }
    ).await?;

    // 4. Generate system.toml
    println!("Generating system.toml after self-composition...");
    run_update_system_toml_workflow_lib(config, Some(warnings_from_split_expanded_lib)).await?;

    Ok(())
}

pub async fn run_layered_composition_workflow_lib(config: &crate::config::Config, args: &CliArgs, layered_compose_args: &cli::LayeredComposeArgs) -> anyhow::Result<()> {
    println!("Debug: layered_compose_args.dry_run = {}", layered_compose_args.dry_run);
    println!("Running layered composition workflow...");
    println!("Config: {:?}", config);
    println!("Args: {:?}", args);
    println!("LayeredComposeArgs: {:?}", layered_compose_args);

    let project_root = std::env::current_dir()?; // Get the actual project root
    let generated_decls_root = config.paths.generated_declarations_root.clone(); // Use configurable path
    let exclude_paths = config.paths.exclude_paths.clone().unwrap_or_default(); // Use configurable exclusion paths

    if layered_compose_args.dry_run {
        println!("DRY RUN: Layered composition workflow will simulate actions without execution.");
        println!("DRY RUN: Would call prelude-generator::collect_prelude_info to extract constants from: {}", project_root.display());
        println!("DRY RUN: Would call prelude-generator::type_usage_analyzer::analyze_type_usage for type analysis.");
        println!("DRY RUN: Would ensure output directory for generated declarations exists: {}", generated_decls_root.display());
        println!("DRY RUN: Would capture CollectedAnalysisData.");
        println!("DRY RUN: Would flatten CollectedAnalysisData into a CodeGraph.");
        println!("DRY RUN: Would serialize and write CodeGraph to: {}", layered_compose_args.code_graph_output_path.clone().unwrap_or_else(|| project_root.join(".gemini").join("generated").join("code_graph.json")).display());
        println!("DRY RUN: Would organize layered declarations into crates using CollectedAnalysisData.");
        return Ok(());
    }

    // Call prelude-generator's collect_prelude_info to extract constants...
    println!("Calling prelude-generator::collect_prelude_info to extract constants...");

    let prelude_generator_args_for_collect_prelude = prelude_generator::Args {
        path: project_root.clone(), // Search the entire project
        exclude_paths: exclude_paths.clone(), // Use configurable exclusion paths
        verbose: args.verbosity,
        dry_run: layered_compose_args.dry_run,
        ..Default::default()
    };

    prelude_generator::collect_prelude_info::collect_prelude_info(
        &project_root, // Pass project root as workspace_path
        &prelude_generator_args_for_collect_prelude, // Pass the args with exclusion
    ).await?;

    println!("prelude-generator::collect_prelude_info for constant extraction completed successfully.");

    // Call prelude-generator's type_usage_analyzer::analyze_type_usage directly
    println!("Calling prelude-generator::type_usage_analyzer::analyze_type_usage...");

    // Ensure the output directory for generated declarations exists
    tokio::fs::create_dir_all(&generated_decls_root)
        .await
        .context(format!("Failed to create generated declarations root directory: {}", generated_decls_root.display()))?;

    // Create Args for type usage analysis
    let type_analysis_args = prelude_generator::Args {
        path: project_root.clone(), // Search the entire project
        analyze_type_usage: true,
        max_expression_depth: Some(8), // Hardcode for now, or make configurable
        output_type_usage_report: Some(generated_decls_root.join("type_usage_report.toml")),
        output_toml_report: Some(generated_decls_root.join("type_usage_report.toml")),
        exclude_paths: exclude_paths.clone(), // Use configurable exclusion paths
        dry_run: layered_compose_args.dry_run,
        verbose: args.verbosity,
        ..Default::default()
    };

    // Capture the returned CollectedAnalysisData
    let collected_analysis_data = prelude_generator::type_usage_analyzer::analyze_type_usage(&type_analysis_args).await?;

    println!("prelude-generator::type_usage_analyzer::analyze_type_usage completed successfully.");
    // println!("Successfully obtained CollectedAnalysisData directly: {:?}", collected_analysis_data);
    // println!("Debug: collected_analysis_data before flattening: {:?}", collected_analysis_data);

    println!("Flattening CollectedAnalysisData into a CodeGraph...");
    let code_graph = code_graph_flattener::flatten_analysis_data_to_graph(
        collected_analysis_data.clone() // Clone because collected_analysis_data is moved into organize_inputs
    ).context("Failed to flatten analysis data into a code graph")?;

    println!("Successfully flattened CollectedAnalysisData into a CodeGraph with {} nodes and {} edges.",
             code_graph.nodes.len(), code_graph.edges.len());

    // Determine CodeGraph output path and serialize
    let code_graph_output_path = layered_compose_args.code_graph_output_path.clone().unwrap_or_else(|| {
        let default_path = project_root.join(".gemini").join("generated").join("code_graph.json");
        println!("No --code-graph-output-path provided, using default: {}", default_path.display());
        default_path
    });

    let serialized_graph = serde_json::to_string_pretty(&code_graph)
        .context("Failed to serialize CodeGraph to JSON")?;
    tokio::fs::create_dir_all(code_graph_output_path.parent().unwrap())
        .await
        .context(format!("Failed to create directory for CodeGraph output: {}", code_graph_output_path.display()))?;
    tokio::fs::write(&code_graph_output_path, serialized_graph)
        .await
        .context(format!("Failed to write CodeGraph to {}", code_graph_output_path.display()))?;
    println!("CodeGraph successfully written to {}", code_graph_output_path.display());

    // 4. Organize layered declarations into crates using the collected analysis data
    println!("Organizing layered declarations into crates using CollectedAnalysisData...");
    let top_level_cargo_toml_path = project_root.join("Cargo.toml");
    let organize_inputs = layered_crate_organizer::OrganizeLayeredDeclarationsInputs {
        project_root: &project_root,
        verbosity: args.verbosity,
        compile_flag: args.compile,
        canonical_output_root: &generated_decls_root, // Use configurable generated root
        top_level_cargo_toml_path: &top_level_cargo_toml_path,
        collected_analysis_data, // Pass the collected analysis data
        code_graph, // Pass the code graph
        topological_sort_output_path: layered_compose_args.topological_sort_output_path.clone(),
        per_file_report_dir: layered_compose_args.per_file_report_dir.clone(),
    };
    let summaries = layered_crate_organizer::organize_layered_declarations(organize_inputs).await?;

    println!("\n--- Layered Composition Summary ---");
    for summary in summaries {
        print!("Crate: {}, Status: {}", summary.crate_name, summary.status);
        if let Some(report_file) = summary.report_file {
            print!(", Report: {}", report_file.display());
        }
        if let Some(error_message) = summary.error_message {
            print!(", Error: {}", error_message);
        }
        println!();
    }
    println!("-----------------------------------\n");

    Ok(())
}

pub async fn run_standalonex_composition_workflow_lib(config: &crate::config::Config, args: &CliArgs) -> anyhow::Result<()> {
    let project_root = std::env::current_dir()?.join("standalonex");
    let metadata_file = project_root.join("rust-bootstrap-core/full_metadata.json");
    let expanded_dir = project_root.join("expanded");

    let main_project_root = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let canonical_output_root = main_project_root.join("generated");
    tokio::fs::create_dir_all(&canonical_output_root)
        .await
        .context(format!("Failed to create canonical output root directory: {}", canonical_output_root.display()))?;

    // 1. Run cargo metadata for standalonex
    println!("Collecting full workspace metadata for standalonex using cargo metadata...");
    std::fs::create_dir_all(metadata_file.parent().unwrap())?;
    let output = Command::new(&config.rust.cargo)
        .args(&["metadata", "--format-version", "1"])
        .current_dir(&project_root) // Run cargo metadata in the standalonex project root
        .output().await?;

    if output.status.success() {
        std::fs::write(&metadata_file, &output.stdout)
            .context(format!("Failed to write metadata to {}", metadata_file.display()))?;
        println!("Standalonex metadata collected to {}.", metadata_file.display());
    } else {
        eprintln!("cargo metadata for standalonex failed.");
        eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("cargo metadata for standalonex failed"));
    }

    // 2. Run expanded-code-collector for standalonex
    println!("Running expanded-code-collector for standalonex...");
    collect_expanded_code(
        &metadata_file,
        &expanded_dir,
        &serde_json::json!({}), // flake_lock_json
        Some(0), // layer
        None,    // package_filter
        false,   // dry_run
        false,   // force
        config.rust.rustc_version.clone(),
        config.rust.rustc_host.clone(),
    ).await?;

    // 3. Run split-expanded-bin for standalonex
    println!("Running split-expanded-bin for standalonex...");
    let expanded_manifest_path = expanded_dir.join("expanded_manifest.json");
    let rustc_info = split_expanded_lib::RustcInfo {
        version: config.rust.rustc_version.clone(),
        host: config.rust.rustc_host.clone(),
    };
    let warnings_from_split_expanded_lib = split_expanded_lib::process_expanded_manifest(
        split_expanded_lib::ProcessExpandedManifestInputs {
            expanded_manifest_path: &expanded_manifest_path,
            project_root: &project_root,
            rustc_info: &rustc_info,
            verbosity: args.verbosity,
            layer: Some(0),
            canonical_output_root: &canonical_output_root,
            package_filter: args.package_filter.clone(), // Pass the package filter here
        }
    ).await?;

    // 4. Organize layered declarations into crates
    println!("Organizing layered declarations into crates...");
    let top_level_cargo_toml_path = main_project_root.join("Cargo.toml");
    let organize_inputs = layered_crate_organizer::OrganizeLayeredDeclarationsInputs {
        project_root: &project_root,
        verbosity: args.verbosity,
        compile_flag: args.compile,
        canonical_output_root: &canonical_output_root,
        top_level_cargo_toml_path: &top_level_cargo_toml_path,
        collected_analysis_data: CollectedAnalysisData::default(),
        code_graph: CodeGraph::default(),
        topological_sort_output_path: None,
        per_file_report_dir: None,
    };
    let _summaries = layered_crate_organizer::organize_layered_declarations(organize_inputs).await?;

    // 5. Generate system.toml
    println!("Generating system.toml after standalonex composition...");
    run_update_system_toml_workflow_lib(config, Some(warnings_from_split_expanded_lib)).await?;

    Ok(())
}

pub async fn run_rustc_composition_workflow_lib(config: &crate::config::Config, args: &CliArgs) -> anyhow::Result<()> {
    let rustc_project_root = PathBuf::from(&config.rust.rustc_source).join("vendor/rust/rust-bootstrap-nix");
    let metadata_file = rustc_project_root.join("rust-bootstrap-core/full_metadata.json");
    let expanded_dir = rustc_project_root.join("expanded");

    let main_project_root = std::env::current_dir()?.parent().unwrap().to_path_buf();
    let canonical_output_root = main_project_root.join("generated");
    tokio::fs::create_dir_all(&canonical_output_root)
        .await
        .context(format!("Failed to create canonical output root directory: {}", canonical_output_root.display()))?;

    // 1. Run cargo metadata for rustc
    println!("Collecting full workspace metadata for rustc using cargo metadata...");
    std::fs::create_dir_all(metadata_file.parent().unwrap())?;
    let output = Command::new(&config.rust.cargo)
        .args(&["metadata", "--format-version", "1"])
        .current_dir(&rustc_project_root) // Run cargo metadata in the rustc project root
        .output().await?;

    if output.status.success() {
        std::fs::write(&metadata_file, &output.stdout)
            .context(format!("Failed to write metadata to {}", metadata_file.display()))?;
        println!("Rustc metadata collected to {}.", metadata_file.display());
    } else {
        eprintln!("cargo metadata for rustc failed.");
        eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("cargo metadata for rustc failed"));
    }

    // 2. Run expanded-code-collector for rustc
    println!("Running expanded-code-collector for rustc...");
    collect_expanded_code(
        &metadata_file,
        &expanded_dir,
        &serde_json::json!({}), // flake_lock_json
        Some(0), // layer
        None,    // package_filter
        false,   // dry_run
        false,   // force
        config.rust.rustc_version.clone(),
        config.rust.rustc_host.clone(),
    ).await?;

    // 3. Run split-expanded-bin for rustc
    println!("Running split-expanded-bin for rustc...");
    let expanded_manifest_path = expanded_dir.join("expanded_manifest.json");
    let rustc_info = split_expanded_lib::RustcInfo {
        version: config.rust.rustc_version.clone(),
        host: config.rust.rustc_host.clone(),
    };
    let warnings_from_split_expanded_lib = split_expanded_lib::process_expanded_manifest(
        split_expanded_lib::ProcessExpandedManifestInputs {
            expanded_manifest_path: &expanded_manifest_path,
            project_root: &rustc_project_root,
            rustc_info: &rustc_info,
            verbosity: args.verbosity,
            layer: Some(0),
            canonical_output_root: &canonical_output_root,
            package_filter: args.package_filter.clone(), // Pass the package filter here
        }
    ).await?;

    // 4. Organize layered declarations into crates
    println!("Organizing layered declarations into crates...");
    let top_level_cargo_toml_path = main_project_root.join("Cargo.toml");
    let organize_inputs = layered_crate_organizer::OrganizeLayeredDeclarationsInputs {
        project_root: &rustc_project_root,
        verbosity: args.verbosity,
                    compile_flag: args.compile,        canonical_output_root: &canonical_output_root,
        top_level_cargo_toml_path: &top_level_cargo_toml_path,
        collected_analysis_data: CollectedAnalysisData::default(),
        code_graph: CodeGraph::default(),
        topological_sort_output_path: None,
        per_file_report_dir: None,
    };
    let _summaries = layered_crate_organizer::organize_layered_declarations(organize_inputs).await?;

    // 5. Generate system.toml
    println!("Generating system.toml after rustc composition...");
    run_update_system_toml_workflow_lib(config, Some(warnings_from_split_expanded_lib)).await?;

    Ok(())
}

pub async fn run_update_system_toml_workflow_lib(config: &crate::config::Config, warnings: Option<Vec<String>>) -> anyhow::Result<()> {
    use tokio::fs;

    let args = CliArgs::parse(); // Re-parse args to get generated_declarations_root

    let rust_system_composer_root = std::env::current_dir()?;
    let main_project_root = rust_system_composer_root.parent().unwrap(); // Assuming rust-system-composer is directly under the main project root
    let system_toml_path = rust_system_composer_root.join("system.toml");
    let logs_dir = rust_system_composer_root.join("logs");
    let warnings_log_path = logs_dir.join("self_compose_warnings.log");

    // Create logs directory if it doesn't exist
    fs::create_dir_all(&logs_dir)
        .await
        .context(format!("Failed to create logs directory: {}", logs_dir.display()))?;

    // Write warnings to log file
    if let Some(ref warnings_vec) = warnings {
        if !warnings_vec.is_empty() {
            let warnings_content = warnings_vec.join("\n");
            fs::write(&warnings_log_path, warnings_content)
                .await
                .context(format!("Failed to write warnings to log file: {}", warnings_log_path.display()))?;
            println!("Warnings written to: {}", warnings_log_path.display());
        }
    }

    // Load config.toml content
    let config_toml_path = main_project_root.join("config.toml"); // config.toml is at the main project root
    let config_toml_content = fs::read_to_string(&config_toml_path)
        .await
        .context(format!("Failed to read config.toml at {}", config_toml_path.display()))?;
    let project_config: toml::Value = toml::from_str(&config_toml_content)
        .context("Failed to parse config.toml into toml::Value")?;

    // Create ProjectInfo
    let project_info = ProjectInfo {
        name: "rust-bootstrap-nix".to_string(),
        root_path: main_project_root.to_path_buf(),
    };

    // Determine the generated_declarations_root
    let generated_declarations_root = if let Some(ref path) = args.generated_declarations_root {
        path.clone()
    } else {
        config.paths.generated_declarations_root.clone()
    };

    // Collect generated projects
    let mut generated_projects: HashMap<String, GeneratedProject> = HashMap::new();

    if generated_declarations_root.exists() && generated_declarations_root.is_dir() {
        for entry in WalkDir::new(&generated_declarations_root)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                let project_dir = entry.path();
                let project_name = project_dir.file_name().unwrap().to_string_lossy().to_string();
                let src_dir = project_dir.join("src");

                let mut modules: Vec<PathBuf> = Vec::new();
                if src_dir.exists() && src_dir.is_dir() {
                    for rs_file_entry in WalkDir::new(&src_dir)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
                    {
                        modules.push(rs_file_entry.path().strip_prefix(&main_project_root)?.to_path_buf());
                    }
                }

                generated_projects.insert(
                    project_name.clone(),
                    GeneratedProject {
                        path: project_dir.strip_prefix(&main_project_root)?.to_path_buf(),
                        modules,
                        declarations: None, // This can be populated later if needed
                    },
                );
            }
        }
    }
    // Construct SystemConfig
    let system_config = SystemConfig {
        project_info,
        project_config,
        generated_projects,
        warnings, // Pass warnings to SystemConfig
    };

    // Serialize to TOML
    let toml_string = toml::to_string_pretty(&system_config)
        .context("Failed to serialize SystemConfig to TOML")?;

    // Write updated system.toml
    fs::write(&system_toml_path, toml_string)
        .await
        .context(format!("Failed to write updated system.toml to {}", system_toml_path.display()))?;

    println!("system.toml updated successfully.");
    println!("Debug: system.toml written to: {:?}", system_toml_path);
    println!("Debug: Does system.toml exist after write? {}", system_toml_path.exists());

    Ok(())
}

pub async fn run_vendorize_workflow_lib(config: &crate::config::Config, args: &CliArgs) -> anyhow::Result<()> {
    use tokio::fs;

    let project_path = if let Some(path) = args.project_path.as_ref() {
        path.clone()
    } else {
        PathBuf::from(&config.rust.rustc_source)
    };

    let output_vendor_dir = if let Some(path) = args.output_vendor_dir.as_ref() {
        path.clone()
    } else {
        config.paths.default_vendor_dir.clone().unwrap_or_else(|| PathBuf::from("vendor/rustc_deps"))
    };

    println!("Vendorizing dependencies for project: {}", project_path.display());
    println!("Output vendor directory: {}", output_vendor_dir.display());

    // Ensure the output vendor directory exists
    fs::create_dir_all(&output_vendor_dir).await?;

    // Construct and execute the cargo vendor command
    let output = Command::new(&config.rust.cargo)
        .args(&["vendor", output_vendor_dir.to_str().unwrap()])
        .current_dir(&project_path)
        .output()
        .await?;

    if output.status.success() {
        println!("Cargo vendor completed successfully.");
        println!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Cargo vendor failed.");
        eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("cargo vendor failed"));
    }

    // Generate or update .cargo/config.toml
    let cargo_config_dir = project_path.join(".cargo");
    fs::create_dir_all(&cargo_config_dir).await?;
    let cargo_config_path = cargo_config_dir.join("config.toml");

    let config_content = format!(
        r#"[source.crates-io]\nreplace-with = \"vendored-sources\"\n\n[source.vendored-sources]\ndirectory = \"{}\"\n"#,
        output_vendor_dir.display()
    );

    fs::write(&cargo_config_path, config_content.as_bytes()).await
        .context(format!("Failed to write .cargo/config.toml to {}", cargo_config_path.display()))?;

    println!("Updated .cargo/config.toml at {} to use vendored sources.", cargo_config_path.display());

    Ok(())
}
