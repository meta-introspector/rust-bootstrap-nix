use clap::Parser;
use std::path::PathBuf;

/// A tool to generate config.toml for the rust-bootstrap process by querying Nix flakes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The bootstrap stage number (e.g., 0, 1, 2)
    #[arg()]
    pub stage: Option<String>,

    /// The target triple for the build (e.g., aarch64-unknown-linux-gnu)
    #[arg()]
    pub target: Option<String>,

    /// The path to the project root (where the top-level flake.nix is located)
    #[arg(long)]
    pub project_root: Option<PathBuf>,

    /// The host system (e.g., aarch64-linux)
    #[arg(long)]
    pub system: Option<String>,

    /// Output file path
    #[arg(long, short, default_value = "config.toml")]
    pub output: Option<PathBuf>,

    /// The flake reference for the rust-bootstrap-nix repository
    #[arg(long)]
    pub rust_bootstrap_nix_flake_ref: Option<String>,

    /// The flake reference for the rust source
    #[arg(long)]
    pub rust_src_flake_ref: Option<String>,

    /// Path to the nixpkgs flake input
    #[arg(long)]
    pub nixpkgs_path: Option<PathBuf>,

    /// Path to the rust-overlay flake input
    #[arg(long)]
    pub rust_overlay_path: Option<PathBuf>,

    /// Path to the rustBootstrapNix flake input
    #[arg(long)]
    pub rust_bootstrap_nix_path: Option<PathBuf>,

    /// Path to the configurationNix flake input
    #[arg(long)]
    pub configuration_nix_path: Option<PathBuf>,

    /// Path to the rustSrcFlake input
    #[arg(long)]
    pub rust_src_flake_path: Option<PathBuf>,

    /// Perform a dry run, printing the generated config to stdout instead of writing to a file.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub dry_run: Option<bool>,

    /// Path to a config.toml file to load configuration from.
    #[arg(long, short)]
    pub config_file: Option<PathBuf>,
}
