use std::path::PathBuf;
use std::fs;

mod args;
mod config_parser;
mod flake_generator;
mod file_writer;
mod statix_checker;

use args::Args;
use clap::Parser;
use config_parser::parse_config;
use flake_generator::generate_flake_nix_content;
use file_writer::write_flake_and_config;
use statix_checker::run_statix_check;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let absolute_output_dir = repo_root.join(&args.output_dir);

    // Ensure output directory exists
    fs::create_dir_all(&absolute_output_dir)?;

    // Parse config.toml
    let config = parse_config(&args.config_path)?;

    // Extract nixpkgs_path from config.toml
    let nixpkgs_url = if config.nix.nixpkgs_path.is_empty() {
        // Fallback to the standard if not specified in config.toml
        "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify".to_string()
    } else {
        config.nix.nixpkgs_path
    };

    // Define the system architecture (can be made dynamic later)
    let system_arch = "aarch64-linux";

    // Generate flake.nix content
    let flake_nix_content = generate_flake_nix_content(&nixpkgs_url, &system_arch);

    // Read config.toml content for writing
    let config_content = fs::read_to_string(&args.config_path)?;

    // Write flake.nix and config.toml to output directory
    write_flake_and_config(&absolute_output_dir, &flake_nix_content, &config_content)?;

    // Run Statix check
    let output_flake_nix_path = absolute_output_dir.join("flake.nix");
    run_statix_check(&absolute_output_dir, &output_flake_nix_path)?;

    Ok(())
}
