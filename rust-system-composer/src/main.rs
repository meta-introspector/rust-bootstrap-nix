use clap::{Parser, Subcommand};
use anyhow::Context;
use std::path::PathBuf;
use tokio::process::Command;
use expanded_code_collector::collect_expanded_code;
use std::collections::HashMap;
use walkdir::WalkDir;

mod config;
use config::Config;

mod layered_crate_organizer;
mod system_config;
use system_config::{SystemConfig, ProjectInfo, GeneratedProject};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file (config.toml).
    #[clap(short, long, value_parser, default_value = "config.toml")]
    config_file: PathBuf,

    /// Root directory where generated declarations are located.
    #[clap(long, value_parser)]
    generated_declarations_root: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,

    /// If set to 'fail', the process will stop on the first compilation error for each generated crate.
    #[clap(long, value_parser, default_value = "continue")]
    compile: String,

    /// Verbosity level (0-3).
    #[clap(short, long, action = clap::ArgAction::Count, default_value_t = 0)]
    verbosity: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Composes the current project (rust-system-composer itself).
    SelfCompose {},
    /// Composes the rustc project.
    RustcCompose {},
    /// Composes the standalonex project.
    StandaloneXCompose {},
    /// Updates the system.toml file with project configuration.
    UpdateSystemToml {},
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = Config::load_from_file(&args.config_file)
        .context(format!("Failed to load configuration from {}", args.config_file.display()))?;

    match &args.command {
        Commands::SelfCompose {} => {
            println!("Running self-composition workflow...");
            run_self_composition_workflow(&config, &args).await?;
        }
        Commands::RustcCompose {} => {
            println!("Running rustc composition workflow...");
            run_rustc_composition_workflow(&config, &args).await?;
        }
        Commands::StandaloneXCompose {} => {
            println!("Running standalonex composition workflow...");
            run_standalonex_composition_workflow(&config, &args).await?;
        }
        Commands::UpdateSystemToml {} => {
            println!("Updating system.toml with project configuration...");
            run_update_system_toml_workflow(&config, None).await?;
        }
    }

    Ok(())
}

async fn run_update_system_toml_workflow(config: &Config, warnings: Option<Vec<String>>) -> anyhow::Result<()> {
    use tokio::fs;

    let args = Args::parse(); // Re-parse args to get generated_declarations_root

    let rust_system_composer_root = std::env::current_dir()?;
    let main_project_root = rust_system_composer_root.parent().unwrap(); // Assuming rust-system-composer is directly under the main project root
    let system_toml_path = rust_system_composer_root.join("system.toml");

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
        warnings,
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

async fn run_self_composition_workflow(config: &Config, args: &Args) -> anyhow::Result<()> {
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
        &expanded_manifest_path,
        project_root.parent().unwrap(),
        &rustc_info,
        3, // verbosity
        Some(0), // layer
        &canonical_output_root,
    ).await?;

    // 4. Generate system.toml
    println!("Generating system.toml after self-composition...");
    run_update_system_toml_workflow(config, Some(warnings_from_split_expanded_lib)).await?;

    Ok(())
}

async fn run_standalonex_composition_workflow(config: &Config, args: &Args) -> anyhow::Result<()> {
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
        &expanded_manifest_path,
        &project_root,
        &rustc_info,
        3, // verbosity
        Some(0), // layer
        &canonical_output_root,
    ).await?;

    // 4. Organize layered declarations into crates
    println!("Organizing layered declarations into crates...");
    let top_level_cargo_toml_path = main_project_root.join("Cargo.toml");
    let organize_inputs = layered_crate_organizer::OrganizeLayeredDeclarationsInputs {
        project_root: &project_root,
        verbosity: args.verbosity,
        compile_flag: &args.compile,
        canonical_output_root: &canonical_output_root,
        top_level_cargo_toml_path: &top_level_cargo_toml_path,
    };
    layered_crate_organizer::organize_layered_declarations(organize_inputs).await?;

    // 5. Generate system.toml
    println!("Generating system.toml after standalonex composition...");
    run_update_system_toml_workflow(config, Some(warnings_from_split_expanded_lib)).await?;

    Ok(())
}

async fn run_rustc_composition_workflow(config: &Config, args: &Args) -> anyhow::Result<()> {
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
        &expanded_manifest_path,
        &rustc_project_root,
        &rustc_info,
        3, // verbosity
        Some(0), // layer
        &canonical_output_root,
    ).await?;

    // 4. Organize layered declarations into crates
    println!("Organizing layered declarations into crates...");
    let top_level_cargo_toml_path = main_project_root.join("Cargo.toml");
    let organize_inputs = layered_crate_organizer::OrganizeLayeredDeclarationsInputs {
        project_root: &rustc_project_root,
        verbosity: args.verbosity,
        compile_flag: &args.compile,
        canonical_output_root: &canonical_output_root,
        top_level_cargo_toml_path: &top_level_cargo_toml_path,
    };
    layered_crate_organizer::organize_layered_declarations(organize_inputs).await?;

    // 5. Generate system.toml
    println!("Generating system.toml after rustc composition...");
    run_update_system_toml_workflow(config, Some(warnings_from_split_expanded_lib)).await?;

    Ok(())
}
