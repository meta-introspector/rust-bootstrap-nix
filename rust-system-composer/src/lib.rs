use anyhow::Context;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use expanded_code_collector::collect_expanded_code;
use std::collections::HashMap;
use walkdir::WalkDir;
use prelude_generator::types::CollectedAnalysisData;
use code_graph_flattener::CodeGraph;
use clap::Parser;
use cli::{CliArgs};
use serde::{Serialize, Deserialize}; // Added for caching
use chrono::Utc; // Added for timestamps in ConfigLock
use sha2::{Sha256, Digest}; // Added for hashing config.toml

mod cli;
mod config;
mod layered_crate_organizer;
mod system_config;
use system_config::{SystemConfig, ProjectInfo, GeneratedProject};
mod traits;
mod config_lock; // Import the new module
use config_lock::{ConfigLock, StageLock, StageStatus}; // Import the structs
mod stages; // Declare the stages module

// Generic function to load data from a cache file
async fn load_from_cache<T: for<'de> Deserialize<'de>>(cache_path: &PathBuf) -> anyhow::Result<Option<T>> {
    if cache_path.exists() {
        let content = tokio::fs::read_to_string(cache_path).await?;
        let data: T = serde_json::from_str(&content)?;
        println!("Loaded from cache: {}", cache_path.display());
        Ok(Some(data))
    } else {
        Ok(None)
    }
}

// Generic function to save data to a cache file
async fn save_to_cache<T: Serialize>(cache_path: &PathBuf, data: &T) -> anyhow::Result<()> {
    tokio::fs::create_dir_all(cache_path.parent().unwrap()).await?;
    let serialized_data = serde_json::to_string_pretty(data)?;
    tokio::fs::write(cache_path, serialized_data).await?;
    println!("Saved to cache: {}", cache_path.display());
    Ok(())
}

// Helper function to calculate SHA256 hash of a file's content
async fn calculate_file_hash(file_path: &Path) -> anyhow::Result<String> {
    use tokio::io::AsyncReadExt; // Import AsyncReadExt for read_buf

    let mut file = tokio::fs::File::open(file_path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new(); // Use a Vec as a buffer
    file.read_to_end(&mut buffer).await?; // Read entire file into buffer
    hasher.update(&buffer); // Update hasher with the buffer content
    Ok(format!("{:x}", hasher.finalize()))
}

// Helper function to create a StageLock
fn create_stage_lock(
    stage_name: &str,
    layered_compose_args: &cli::LayeredComposeArgs,
    status: StageStatus,
) -> StageLock {
    let mut parameters = HashMap::new();
    // Add relevant parameters from layered_compose_args to the StageLock
    parameters.insert("dry_run".to_string(), layered_compose_args.dry_run.to_string());
    parameters.insert("skip_prelude_info".to_string(), layered_compose_args.skip_prelude_info.to_string());
    parameters.insert("force_prelude_info".to_string(), layered_compose_args.force_prelude_info.to_string());
    parameters.insert("skip_type_analysis".to_string(), layered_compose_args.skip_type_analysis.to_string());
    parameters.insert("force_type_analysis".to_string(), layered_compose_args.force_type_analysis.to_string());
    parameters.insert("skip_graph_flattening".to_string(), layered_compose_args.skip_graph_flattening.to_string());
    parameters.insert("force_graph_flattening".to_string(), layered_compose_args.force_graph_flattening.to_string());
    parameters.insert("skip_crate_organizer".to_string(), layered_compose_args.skip_crate_organizer.to_string());
    parameters.insert("force_crate_organizer".to_string(), layered_compose_args.force_crate_organizer.to_string());
    parameters.insert("skip_command_report".to_string(), layered_compose_args.skip_command_report.to_string());
    parameters.insert("force_command_report".to_string(), layered_compose_args.force_command_report.to_string());
    // Add other relevant parameters as needed

    StageLock {
        name: stage_name.to_string(), // Initialize the name field
        status,
        input_hashes: HashMap::new(),
        output_hashes: HashMap::new(),
        parameters,
        dependencies: Vec::new(), // Will be populated later
        timestamp: Utc::now(),
        log_path: None,
        report_path: None,
    }
}

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

use stages::orchestrator::StageOrchestrator; // Import StageOrchestrator

pub async fn run_layered_composition_workflow_lib(config: &crate::config::Config, args: &CliArgs, layered_compose_args: &cli::LayeredComposeArgs) -> anyhow::Result<()> {
    println!("Debug: layered_compose_args.dry_run = {}", layered_compose_args.dry_run);
    println!("Running layered composition workflow...");
    println!("Config: {:?}", config);
    println!("Args: {:?}", args);
    println!("LayeredComposeArgs: {:?}", layered_compose_args);

    // Summary of enabled/disabled stages
    println!("\n--- Layered Composition Workflow Stages ---");
    println!("  Prelude Info Collection: {}", if layered_compose_args.skip_prelude_info { "Skipped" } else { "Enabled" });
    println!("  Type Analysis: {}", if layered_compose_args.skip_type_analysis { "Skipped" } else { "Enabled" });
    println!("  Code Graph Flattening: {}", if layered_compose_args.skip_graph_flattening { "Skipped" } else { "Enabled" });
    println!("  Layered Crate Organizer: {}", if layered_compose_args.skip_crate_organizer { "Skipped" } else { "Enabled" });
    println!("  Command Report Generation: {}", if layered_compose_args.skip_command_report { "Skipped" } else { "Enabled" });
    println!("-------------------------------------------\n");

    let project_root = std::env::current_dir()?; // Get the actual project root
    let _generated_decls_root = config.paths.generated_declarations_root.clone(); // Use configurable path
    let _exclude_paths = config.paths.exclude_paths.clone().unwrap_or_default(); // Use configurable exclusion paths

    // Define cache directories for intermediate results
    let _cache_dir = project_root.join(".gemini").join("cache").join("layered_compose");
    tokio::fs::create_dir_all(&_cache_dir).await.context("Failed to create layered_compose cache directory")?;

    let _collected_analysis_data_cache_path = _cache_dir.join("collected_analysis_data.json");
    let _code_graph_cache_path = _cache_dir.join("code_graph.json");
    let _layered_crate_organizer_summaries_cache_path = _cache_dir.join("layered_crate_organizer_summaries.json");
    let _command_report_cache_path = _cache_dir.join("command_report.json");

    if layered_compose_args.dry_run && !layered_compose_args.generate_lock_only {
        println!("DRY RUN: Layered composition workflow will simulate actions without execution.");
        // The detailed DRY RUN messages for each stage will now be handled by their respective conditional blocks.
        return Ok(());
    }

    // Determine config_lock_path
    let config_lock_path = layered_compose_args.config_lock_path.clone().unwrap_or_else(|| {
        project_root.join(".gemini").join("generated").join("config.lock")
    });

    // Load or initialize ConfigLock
    let mut config_lock: ConfigLock = if config_lock_path.exists() {
        load_from_cache(&config_lock_path).await?.unwrap_or_default()
    } else {
        ConfigLock::default()
    };

    // Calculate hash of config.toml
    let config_toml_path = args.config_file.clone().context("Config file path is required")?;
    let config_toml_hash = calculate_file_hash(&config_toml_path).await?;
    config_lock.config_toml_hash = config_toml_hash;
    config_lock.generated_at = Utc::now();

    let orchestrator = StageOrchestrator::new();
    orchestrator.run_stages(
        &project_root,
        config,
        args, // Pass cli_args
        layered_compose_args,
        &mut config_lock,
    )?;

    // Save the updated ConfigLock
    config_lock.save(&config_lock_path).await?;
    println!("ConfigLock saved to: {:?}", config_lock_path);

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
