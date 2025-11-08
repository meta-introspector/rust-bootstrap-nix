use clap::{Parser, Subcommand};
use anyhow::Context;
use std::path::PathBuf;
use tokio::process::Command;
use expanded_code_collector::collect_expanded_code;

mod config;
use config::Config;

mod layered_crate_organizer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file (config.toml).
    #[clap(short, long, value_parser, default_value = "config.toml")]
    config_file: PathBuf,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Composes the current project (rust-system-composer itself).
    SelfCompose {},
    /// Composes the rustc project.
    RustcCompose {},
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
            run_self_composition_workflow(&config).await?;
        }
        Commands::RustcCompose {} => {
            println!("Running rustc composition workflow...");
            run_rustc_composition_workflow(&config).await?;
        }
        Commands::UpdateSystemToml {} => {
            println!("Updating system.toml with project configuration...");
            run_update_system_toml_workflow(&config).await?;
        }
    }

    Ok(())
}

async fn run_update_system_toml_workflow(config: &Config) -> anyhow::Result<()> {
    use tokio::fs;
    use toml_edit::{DocumentMut, value};

    let project_root = std::env::current_dir()?;
    let system_toml_path = project_root.join("rust-system-composer/system.toml");

    // Read existing system.toml
    let system_toml_content = fs::read_to_string(&system_toml_path)
        .await
        .context(format!("Failed to read system.toml at {}", system_toml_path.display()))?;
    let mut system_toml_doc = system_toml_content.parse::<DocumentMut>()
        .context("Failed to parse system.toml")?;

    // Add project_info
    system_toml_doc["project_info"]["name"] = value("rust-bootstrap-nix");
    system_toml_doc["project_info"]["root_path"] = value(project_root.to_str().unwrap());

    // Add project_config (embedding config.toml content)
    let config_toml_path = project_root.join("config.toml");
    let config_toml_content = fs::read_to_string(&config_toml_path)
        .await
        .context(format!("Failed to read config.toml at {}", config_toml_path.display()))?;
    let config_toml_doc = config_toml_content.parse::<DocumentMut>()
        .context("Failed to parse config.toml")?;

    // If `system_toml_doc["project_config"]` doesn't exist, create it.
    if system_toml_doc["project_config"].is_none() {
        system_toml_doc["project_config"] = toml_edit::table();
    }

    // Iterate through config_toml_doc and add its items to system_toml_doc["project_config"]
    for (key, item) in config_toml_doc.iter() {
        system_toml_doc["project_config"][key] = item.clone();
    }

    // Write updated system.toml
    fs::write(&system_toml_path, system_toml_doc.to_string())
        .await
        .context(format!("Failed to write updated system.toml to {}", system_toml_path.display()))?;

    println!("system.toml updated successfully.");

    Ok(())
}

async fn run_self_composition_workflow(config: &Config) -> anyhow::Result<()> {
    let project_root = std::env::current_dir()?;
    let metadata_file = project_root.join("rust-bootstrap-core/full_metadata.json");
    let expanded_dir = project_root.join("expanded");

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
    split_expanded_lib::process_expanded_manifest(
        split_expanded_lib::ProcessManifestParams {
            expanded_manifest_path: &expanded_manifest_path,
            project_root: &project_root.parent().unwrap(),
            rustc_info: &rustc_info,
            verbosity: 3,
            layer: Some(0),
        },
    ).await?;

    Ok(())
}

async fn run_rustc_composition_workflow(config: &Config) -> anyhow::Result<()> {
    let rustc_project_root = PathBuf::from(&config.rust.rustc_source).join("vendor/rust/rust-bootstrap-nix");
    let metadata_file = rustc_project_root.join("rust-bootstrap-core/full_metadata.json");
    let expanded_dir = rustc_project_root.join("expanded");

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
    split_expanded_lib::process_expanded_manifest(
        split_expanded_lib::ProcessManifestParams {
            expanded_manifest_path: &expanded_manifest_path,
            project_root: &rustc_project_root,
            rustc_info: &rustc_info,
            verbosity: 3,
            layer: Some(0),
        },
    ).await?;

    // 4. Organize layered declarations into crates
    println!("Organizing layered declarations into crates...");
    layered_crate_organizer::organize_layered_declarations(
        &rustc_project_root,
        3, // verbosity
    ).await?;

    Ok(())
}
