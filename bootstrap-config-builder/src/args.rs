use crate::prelude::*;


use clap::Parser;
use std::path::PathBuf;

/// A tool to generate config.toml for the rust-bootstrap process by querying Nix flakes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to a config.toml file to load configuration from.
    #[arg(long, short = 'c', value_name = "FILE")]
    pub config_file: Option<PathBuf>,

    /// The build stage to configure (e.g., "0", "1", "2").
    #[arg(long, short, value_name = "STAGE")]
    pub stage: Option<String>,

    /// The target triple for the build (e.g., "aarch64-unknown-linux-gnu").
    #[arg(long, short, value_name = "TARGET")]
    pub target: Option<String>,

    /// The root directory of the rust-bootstrap-nix project.
    #[arg(long, value_name = "PATH")]
    pub project_root: Option<PathBuf>,

    /// The system for which to build (e.g., "aarch64-linux").
    #[arg(long, value_name = "SYSTEM")]
    pub system: Option<String>,

    /// The output path for the generated config.toml.
    #[arg(long, short, default_value = "config.toml")]
    pub output: Option<PathBuf>,

    /// The flake reference for rust-bootstrap-nix.
    #[arg(long, value_name = "REF")]
    pub rust_bootstrap_nix_flake_ref: Option<String>,

    /// The flake reference for rust-src.
    #[arg(long, value_name = "REF")]
    pub rust_src_flake_ref: Option<String>,

    /// The path to nixpkgs.
    #[arg(long, value_name = "PATH")]
    pub nixpkgs_path: Option<PathBuf>,

    /// The path to rust-overlay.
    #[arg(long, value_name = "PATH")]
    pub rust_overlay_path: Option<PathBuf>,

    /// The path to rust-bootstrap-nix.
    #[arg(long, value_name = "PATH")]
    pub rust_bootstrap_nix_path: Option<PathBuf>,

    /// The path to configuration-nix.
    #[arg(long, value_name = "PATH")]
    pub configuration_nix_path: Option<PathBuf>,

    /// The path to rust-src flake.
    #[arg(long, value_name = "PATH")]
    pub rust_src_flake_path: Option<PathBuf>,

    /// Perform a dry run, printing the generated config to stdout.
    #[arg(long)]
    pub dry_run: bool,

    /// The path to the rustc executable.
    #[arg(long, value_name = "PATH")]
    pub rustc_path: Option<PathBuf>,

    /// The path to the cargo executable.
    #[arg(long, value_name = "PATH")]
    pub cargo_path: Option<PathBuf>,

    /// The Rust channel to use (e.g., "stable", "beta", "nightly").
    #[arg(long, value_name = "CHANNEL")]
    pub rust_channel: Option<String>,

    /// Whether to download rustc.
    #[arg(long)]
    pub rust_download_rustc: Option<bool>,

    /// Whether to enable parallel compilation.
    #[arg(long)]
    pub rust_parallel_compiler: Option<bool>,

    /// Whether to enable LLVM tools.
    #[arg(long)]
    pub rust_llvm_tools: Option<bool>,

    /// The debuginfo level for Rust compilation.
    #[arg(long, value_name = "LEVEL")]
    pub rust_debuginfo_level: Option<u8>,

    /// Whether to patch binaries for Nix.
    #[arg(long)]
    pub patch_binaries_for_nix: Option<bool>,

    /// Whether to enable vendoring.
    #[arg(long)]
    pub vendor: Option<bool>,

    /// The build directory.
    #[arg(long, value_name = "PATH")]
    pub build_dir: Option<PathBuf>,

    /// The number of build jobs.
    #[arg(long, value_name = "JOBS")]
    pub build_jobs: Option<u32>,

    /// The HOME directory for the build.
    #[arg(long, value_name = "PATH")]
    pub home_dir: Option<PathBuf>,

    /// The CARGO_HOME directory for the build.
    #[arg(long, value_name = "PATH")]
    pub cargo_home_dir: Option<PathBuf>,

    /// The installation prefix.
    #[arg(long, value_name = "PATH")]
    pub install_prefix: Option<PathBuf>,

    /// The system configuration directory.
    #[arg(long, value_name = "PATH")]
    pub install_sysconfdir: Option<PathBuf>,

    /// The folder for distribution signing.
    #[arg(long, value_name = "PATH")]
    pub dist_sign_folder: Option<PathBuf>,

    /// The upload address for distribution.
    #[arg(long, value_name = "ADDR")]
    pub dist_upload_addr: Option<String>,

    /// Whether to download CI LLVM.
    #[arg(long)]
    pub llvm_download_ci_llvm: Option<bool>,

    /// Whether to use Ninja for LLVM.
    #[arg(long)]
    pub llvm_ninja: Option<bool>,

    /// The change ID for tracking major changes.
    #[arg(long, value_name = "ID")]
    pub change_id: Option<String>,

    /// The rustc version to build with (e.g., "1.89.0", "1.92.0-nightly").
    /// This will drive the generation of the flake and config.
    #[arg(long, value_name = "VERSION")]
    pub build_rustc_version: Option<String>,

    /// The path to the Solana rustc executable (source rustc).
    #[arg(long, value_name = "PATH")]
    pub solana_rustc_path: Option<PathBuf>,

    /// The architecture for the build (e.g., "aarch64-linux").
    #[arg(long, value_name = "ARCH")]
    pub architecture: Option<String>,

    /// The step in the bootstrap process (e.g., "step1-configure").
    #[arg(long, value_name = "STEP")]
    pub step: Option<String>,
}
