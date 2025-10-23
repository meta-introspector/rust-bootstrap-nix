use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use log::{info, debug}; // Import log macros

pub mod utils; // Declare the utils module as public
mod preconditions; // Declare the preconditions module
pub mod args; // Declare the args module

use crate::utils::validate_project_root::validate_project_root;
use crate::utils::construct_config_content::construct_config_content;
use crate::args::Args;

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

    // 2. Use provided flake input paths

    debug!("rust_src_flake_path: {:?}", args.rust_src_flake_path);

    preconditions::check_rust_toolchain_sysroot(
        &args.rust_src_flake_path.to_string_lossy(),
    )?;
    info!("Rust toolchain sysroot check passed.");



    // 3. Construct the config.toml content
    info!("Constructing config.toml content...");
    let config_content = construct_config_content(
        &args.system,
        flake_path_str,
        &args.nixpkgs_path.to_string_lossy(),
        &args.rust_overlay_path.to_string_lossy(),
        &args.rust_bootstrap_nix_path.to_string_lossy(),
        &args.configuration_nix_path.to_string_lossy(),
        &args.rust_src_flake_path.to_string_lossy(),
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
