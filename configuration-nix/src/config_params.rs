use crate::prelude::*;
/// Struct to hold all configuration parameters for generating config.toml.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ConfigParams {
    /// The bootstrap stage number (e.g., 0, 1, 2)
    #[arg()]
    pub stage: String,
    /// The target triple for the build (e.g., aarch64-unknown-linux-gnu)
    #[arg()]
    pub target: String,
    /// Path to the nixpkgs flake input
    #[arg(long)]
    pub nixpkgs_path: PathBuf,
    /// Path to the rust-overlay flake input
    #[arg(long)]
    pub rust_overlay_path: PathBuf,
    /// Path to the configurationNix flake input
    #[arg(long)]
    pub configuration_nix_path: PathBuf,
    /// Path to the rustSrcFlake input
    #[arg(long)]
    pub rust_src_flake_path: PathBuf,
    /// The flake reference for the rust-bootstrap-nix repository
    #[arg(long)]
    pub rust_bootstrap_nix_flake_ref: String,
    /// The flake reference for the rust source
    #[arg(long)]
    pub rust_src_flake_ref: String,
}
