use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    path::PathBuf,
};

pub mod utils; // Declare the utils module as public
mod preconditions; // Declare the preconditions module

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

    /// The flake reference for the rust-bootstrap-nix repository
    #[arg(long)]
    rust_bootstrap_nix_flake_ref: String,

    /// The flake reference for the rust source
    #[arg(long)]
    rust_src_flake_ref: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Run precondition checks
    preconditions::check_nix_command_available()?;
    preconditions::check_rust_toolchain_sysroot(
        &args.rust_bootstrap_nix_flake_ref,
        &args.system,
        // Assuming rust-overlay is an input to rust-bootstrap-nix flake
        // and its ref is the same as rust_bootstrap_nix_flake_ref for now.
        // This might need to be a separate argument if it varies.
        &args.rust_bootstrap_nix_flake_ref,
    )?;
    preconditions::check_rust_src_flake_exists(
        &args.rust_bootstrap_nix_flake_ref,
        &args.rust_src_flake_ref,
    )?;


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
        &args.rust_bootstrap_nix_flake_ref,
        &args.rust_src_flake_ref,
    );

    // 4. Write the output file
    fs::write(&args.output, config_content)
        .with_context(|| format!("Failed to write config to file: {:?}", args.output))?;

    println!("Successfully generated config file at: {:?}", args.output);

    Ok(())
}