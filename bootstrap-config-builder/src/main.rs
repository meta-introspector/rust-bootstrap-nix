use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs,
    path::PathBuf,
};
use log::{info, debug}; // Import log macros

pub mod utils; // Declare the utils module as public
mod preconditions; // Declare the preconditions module

use crate::utils::validate_project_root::validate_project_root;
use crate::utils::get_flake_input::get_flake_input;
use crate::utils::construct_config_content::construct_config_content;


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

    /// Perform a dry run, printing the generated config to stdout instead of writing to a file.
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger

    let args = Args::parse();

    info!("Starting config generation for stage {} and target {}", args.stage, args.target);
    debug!("Arguments: {:?}", args);

    // Run precondition checks
    info!("Running precondition checks...");
    preconditions::check_nix_command_available()?;
    info!("Nix command available.");

    // 1. Validate the project root
    info!("Validating project root: {:?}", args.project_root);
    let project_root = validate_project_root(&args.project_root)?;
    let flake_path_str = project_root.to_str()
        .context("Project root path contains non-UTF8 characters")?;
    info!("Project root validated: {}", flake_path_str);

    // 2. Query Nix for all required flake input paths
    info!("Querying Nix for flake input paths...");
    let nixpkgs_path = get_flake_input(flake_path_str, "nixpkgs")?;
    debug!("nixpkgs_path: {}", nixpkgs_path);
    let rust_overlay_path = get_flake_input(flake_path_str, "rust-overlay")?;
    debug!("rust_overlay_path: {}", rust_overlay_path);
    // These inputs might not exist in every flake, so we handle potential errors.
    let rust_bootstrap_nix_path = get_flake_input(flake_path_str, "rustBootstrapNix").unwrap_or_else(|_| {
        debug!("rustBootstrapNix input not found, using 'not-found'");
        "not-found".to_string()
    });
    debug!("rust_bootstrap_nix_path: {}", rust_bootstrap_nix_path);
    let configuration_nix_path = get_flake_input(flake_path_str, "configurationNix").unwrap_or_else(|_| {
        debug!("configurationNix input not found, using 'not-found'");
        "not-found".to_string()
    });
    debug!("configuration_nix_path: {}", configuration_nix_path);
    let rust_src_flake_path = get_flake_input(flake_path_str, "rustSrcFlake")?;
    debug!("rust_src_flake_path: {}", rust_src_flake_path);

    preconditions::check_rust_toolchain_sysroot(
        &rust_src_flake_path,
    )?;
    info!("Rust toolchain sysroot check passed.");



    // 3. Construct the config.toml content
    info!("Constructing config.toml content...");
    let config_content = construct_config_content(
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
    debug!("Generated config content:\n{}", config_content);

    // 4. Handle output based on dry_run flag
    if args.dry_run {
        info!("Dry run enabled. Generated config will be printed to stdout.");
        println!("{}", config_content);
    } else {
        info!("Writing generated config to file: {:?}", args.output);
        fs::write(&args.output, config_content)
            .context(format!("Failed to write config to file: {:?}", args.output))?;
        info!("Config successfully written to {:?}", args.output);
    }

    Ok(())
}
