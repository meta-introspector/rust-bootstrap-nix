use clap::Parser;
use std::path::PathBuf;

/// A tool to generate config.toml for the rust-bootstrap process by querying Nix flakes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The bootstrap stage number (e.g., 0, 1, 2)
    #[arg()]
    pub stage: String,

    /// The target triple for the build (e.g., aarch64-unknown-linux-gnu)
    #[arg()]
    pub target: String,

    /// The path to the project root (where the top-level flake.nix is located)
    #[arg(long)]
    pub project_root: PathBuf,

    /// The host system (e.g., aarch64-linux)
    #[arg(long)]
    pub system: String,

    /// Output file path
    #[arg(long, short, default_value = "config.toml")]
    pub output: PathBuf,

    /// The flake reference for the rust-bootstrap-nix repository
    #[arg(long)]
    pub rust_bootstrap_nix_flake_ref: String,

    /// The flake reference for the rust source
    #[arg(long)]
    pub rust_src_flake_ref: String,

    /// Path to the nixpkgs flake input
    #[arg(long)]
    pub nixpkgs_path: PathBuf,

    /// Path to the rust-overlay flake input
    #[arg(long)]
    pub rust_overlay_path: PathBuf,

    /// Path to the rustBootstrapNix flake input
    #[arg(long)]
    pub rust_bootstrap_nix_path: PathBuf,

    /// Path to the configurationNix flake input
    #[arg(long)]
    pub configuration_nix_path: PathBuf,

    /// Path to the rustSrcFlake input
    #[arg(long)]
    pub rust_src_flake_path: PathBuf,

    /// Perform a dry run, printing the generated config to stdout instead of writing to a file.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}
