use clap::{Parser, Subcommand};
use anyhow::Context;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use expanded_code_collector::collect_expanded_code;

mod config;
use config::Config;

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
    println!("Running expanded-code-collector...");
    collect_expanded_code(
        &metadata_file,
        &expanded_dir,
        &PathBuf::from(&config.rust.rustc),
        config.rust.rustc_version.clone(),
        config.rust.rustc_host.clone(),
        &project_root.parent().unwrap(),
        0,
        None,
        false,
    ).await?;

    // 3. Run split-expanded-bin
    println!("Running split-expanded-bin...");
    let expanded_manifest = expanded_dir.join("expanded_manifest.json");
    execute_command(
        &config.bins.split_expanded_bin,
        &[
            "--expanded-manifest",
            &expanded_manifest.display().to_string(),
            "--project-root",
            &project_root.parent().unwrap().display().to_string(),
            "--rustc-version",
            &config.rust.rustc_version,
            "--rustc-host",
            &config.rust.rustc_host,
            "--verbosity",
            "3",
            "--layer",
            "0",
        ],
        Some(&project_root.parent().unwrap()), // Set current_dir to project root
    ).await?;

    Ok(())
}

async fn run_rustc_composition_workflow(config: &Config) -> anyhow::Result<()> {
    let rustc_project_root = PathBuf::from(&config.rust.rustc_source);
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
        &PathBuf::from(&config.rust.rustc),
        config.rust.rustc_version.clone(),
        config.rust.rustc_host.clone(),
        &rustc_project_root,
        0,
        None,
        false,
    ).await?;

    // 3. Run split-expanded-bin for rustc
    println!("Running split-expanded-bin for rustc...");
    let expanded_manifest = expanded_dir.join("expanded_manifest.json");
    execute_command(
        &config.bins.split_expanded_bin,
        &[
            "--expanded-manifest",
            &expanded_manifest.display().to_string(),
            "--project-root",
            &rustc_project_root.display().to_string(),
            "--rustc-version",
            &config.rust.rustc_version,
            "--rustc-host",
            &config.rust.rustc_host,
            "--verbosity",
            "3",
            "--layer",
            "0",
        ],
        Some(&rustc_project_root), // Set current_dir to rustc project root
    ).await?;

    Ok(())
}

// Helper function to execute external commands
async fn execute_command(binary_path: &str, args: &[&str], current_dir: Option<&Path>) -> anyhow::Result<()> {
    println!("Executing: {} {}", binary_path, args.join(" "));
    let mut cmd = Command::new(binary_path);
    cmd.args(args);
    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }
    let output = cmd.output().await?;

    if output.status.success() {
        println!("Command successful.");
        println!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed.");
        eprintln!("Stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow::anyhow!("Command failed: {}", binary_path));
    }
    Ok(())
}