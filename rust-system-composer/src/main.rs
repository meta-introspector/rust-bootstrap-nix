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
    }

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
        &expanded_manifest_path,
        &project_root.parent().unwrap(), // project_root
        &rustc_info,
        3, // verbosity
        Some(0), // layer
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
        &expanded_manifest_path,
        &rustc_project_root, // project_root
        &rustc_info,         // rustc_info
        3,                   // verbosity
        Some(0),             // layer
    ).await?;

    // 4. Organize layered declarations into crates
    println!("Organizing layered declarations into crates...");
    layered_crate_organizer::organize_layered_declarations(
        &rustc_project_root,
        3, // verbosity
    ).await?;

    Ok(())
}
