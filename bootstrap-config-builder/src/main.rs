use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use log::{info, debug}; // Import log macros
use toml;
use crate::config::AppConfig;

pub mod utils; // Declare the utils module as public
mod preconditions; // Declare the preconditions module
pub mod args; // Declare the args module
pub mod config; // Declare the config module

use crate::utils::validate_project_root::validate_project_root;
use crate::utils::construct_config_content::construct_config_content;
use crate::args::Args;

fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger

    let args = Args::parse();
    debug!("Raw CLI Arguments: {:?}", args);

    let mut app_config = if let Some(config_file_path) = &args.config_file {
        info!("Loading configuration from file: {:?}", config_file_path);
        let config_content = fs::read_to_string(config_file_path)
            .context(format!("Failed to read config file: {:?}", config_file_path))?;
        toml::from_str(&config_content)
            .context(format!("Failed to parse config file: {:?}", config_file_path))?
    } else {
        config::AppConfig::default()
    };

    app_config.merge_with_args(&args);
    info!("Final merged configuration: {:?}", app_config);

    info!("Starting config generation for stage {:?} and target {:?}", app_config.stage, app_config.target);

    // Run precondition checks
    info!("Running precondition checks...");
    preconditions::check_nix_command_available()?;
    info!("Nix command available.");

    // 1. Validate the project root
    info!("Validating project root: {:?}", app_config.project_root);
    let project_root = validate_project_root(app_config.project_root.as_ref().context("Project root is required")?)?;
    let flake_path_str = project_root.to_str()
        .context("Project root path contains non-UTF8 characters")?;
    info!("Project root validated: {}", flake_path_str);

    // 2. Use provided flake input paths
    let rust_src_flake_path_lossy = app_config.rust_src_flake_path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    debug!("rust_src_flake_path: {:?}", rust_src_flake_path_lossy);

    preconditions::check_rust_toolchain_sysroot(
        &rust_src_flake_path_lossy,
    )?;
    info!("Rust toolchain sysroot check passed.");

    // 3. Construct the config.toml content
    info!("Constructing config.toml content...");
    let config_content = construct_config_content(
        app_config.system.as_deref().unwrap_or_default(),
        flake_path_str,
        app_config.nixpkgs_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
        app_config.rust_overlay_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
        app_config.rust_bootstrap_nix_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
        app_config.configuration_nix_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
        app_config.rust_src_flake_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
        app_config.stage.as_deref().unwrap_or_default(),
        app_config.target.as_deref().unwrap_or_default(),
        app_config.rust_bootstrap_nix_flake_ref.as_deref().unwrap_or_default(),
        app_config.rust_src_flake_ref.as_deref().unwrap_or_default(),
    );
    debug!("Generated config content:\n{}", config_content);

    // 4. Handle output based on dry_run flag
    if app_config.dry_run.unwrap_or(false) {
        info!("Dry run enabled. Generated config will be printed to stdout.");
        println!("{}", config_content);
    } else {
        let output_path = app_config.output.unwrap_or_else(|| "config.toml".into());
        info!("Writing generated config to file: {:?}", output_path);
        fs::write(&output_path, config_content)
            .context(format!("Failed to write config to file: {:?}", output_path))?;
        info!("Config successfully written to {:?}", output_path);
    }

    Ok(())
}
