use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    path::PathBuf,
};

mod utils; // Declare the utils module

/// A tool to generate config.toml for the rust-bootstrap process by querying Nix flakes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The bootstrap stage number (e.g., 0, 1, 2)
    #[arg()]
    stage: String,

    /// The target triple for the build (e.g., aarch64-unknown-linux-gnu)
    #[arg()]
    target: String,

    /// The path to the project root (where the top-level flake.nix is located)
    #[arg(long)]
    project_root: PathBuf,

    /// The host system (e.g., aarch64-linux)
    #[arg(long)]
    system: String,

    /// Output file path
    #[arg(long, short, default_value = "config.toml")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Validate the project root
    let project_root = utils::validate_project_root(&args.project_root)?;
    let flake_path_str = project_root.to_str()
        .context("Project root path contains non-UTF8 characters")?;

    // 2. Query Nix for all required flake input paths
    let nixpkgs_path = utils::get_flake_input(flake_path_str, "nixpkgs")?;
    let rust_overlay_path = utils::get_flake_input(flake_path_str, "rust-overlay")?;
    // These inputs might not exist in every flake, so we handle potential errors.
    let rust_bootstrap_nix_path = utils::get_flake_input(flake_path_str, "rustBootstrapNix").unwrap_or_else(|_| "not-found".to_string());
    let configuration_nix_path = utils::get_flake_input(flake_path_str, "configurationNix").unwrap_or_else(|_| "not-found".to_string());
    let rust_src_flake_path = utils::get_flake_input(flake_path_str, "rustSrcFlake")?;


    // 3. Construct the config.toml content
    let config_content = utils::construct_config_content(
        &args.system,
        flake_path_str,
        &nixpkgs_path,
        &rust_overlay_path,
        &rust_bootstrap_nix_path,
        &configuration_nix_path,
        &rust_src_flake_path,
        &args.stage,
        &args.target,
    );

    // 4. Write the output file
    fs::write(&args.output, config_content)
        .with_context(|| format!("Failed to write config to file: {:?}", args.output))?;

    println!("Successfully generated config file at: {:?}", args.output);

    Ok(())
}