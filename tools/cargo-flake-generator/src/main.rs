// src/main.rs for cargo-flake-generator

use std::path::PathBuf;
use anyhow::{Result, Context};
use clap::Parser;

use cargo_flake_generator::{generate_flake_for_crate, FlakeGeneratorConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Cargo project root (directory containing Cargo.toml)
    #[arg(short, long)]
    project_path: PathBuf,

    /// URL for nixpkgs input in the flake
    #[arg(long, default_value_t = FlakeGeneratorConfig::default().nixpkgs_url)]
    nixpkgs_url: String,

    /// URL for rust-overlay input in the flake
    #[arg(long, default_value_t = FlakeGeneratorConfig::default().rust_overlay_url)]
    rust_overlay_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = FlakeGeneratorConfig {
        nixpkgs_url: args.nixpkgs_url,
        rust_overlay_url: args.rust_overlay_url,
    };

    generate_flake_for_crate(&args.project_path, &config)
        .context(format!("Failed to generate flake for {}", args.project_path.display()))?;

    Ok(())
}
