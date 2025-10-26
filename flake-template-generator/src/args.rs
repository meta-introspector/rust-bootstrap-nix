use clap::Parser;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the generated config.toml
    #[arg(long)]
    pub config_path: PathBuf,

    /// Output directory for the new flake
    #[arg(long)]
    pub output_dir: PathBuf,

    /// Component for the branch name: e.g., solana-rust-1.83
    #[arg(long)]
    pub component: String,

    /// Architecture for the branch name: e.g., aarch64
    #[arg(long)]
    pub arch: String,

    /// Phase for the branch name: e.g., phase0
    #[arg(long)]
    pub phase: String,

    /// Step for the branch name: e.g., step1
    #[arg(long)]
    pub step: String,

    /// Perform a dry run without executing Git commands
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Show verbose output for Git operations
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
}
