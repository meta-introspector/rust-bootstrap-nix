use clap::Parser;
use std::path::{PathBuf};
use flake_orchestrator_lib::orchestrate_flake_generation;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set verbosity level (0 = silent, 1 = normal, 2 = detailed, 3 = debug).
    #[arg(short, long, default_value_t = 1)]
    pub verbosity: u8,

    /// Path to the cargo metadata JSON file.
    #[arg(long, required = true)]
    pub metadata_path: PathBuf,

    /// Path to the flake.lock file.
    #[arg(long, required = true)]
    pub flake_lock_path: PathBuf,

    /// Directory to output the expanded code.
    #[arg(long, required = true)]
    pub expanded_output_dir: PathBuf,

    /// Rustc version (e.g., "1.89.0").
    #[arg(long)]
    pub rustc_version: String,

    /// Rustc host triple (e.g., "aarch64-unknown-linux-gnu").
    #[arg(long)]
    pub rustc_host: String,

    /// Optional: Process only crates at a specific dependency layer.
    #[arg(long)]
    layer: Option<u32>,

    /// Optional: Filter by package name.
    #[arg(long)]
    package_filter: Option<String>,

    /// Path to the JSON summary file for declarations.
    #[arg(long, required = true)]
    json_summary_path: PathBuf,

    /// Directory for log output.
    #[arg(long, required = true)]
    log_output_dir: PathBuf,

    /// Output directory for the generated flake.
    #[arg(long, required = true)]
    pub flake_output_dir: PathBuf,

    /// Component for the branch name: e.g., solana-rust-1.83
    #[arg(long, required = true)]
    pub flake_component: String,

    /// Architecture for the branch name: e.g., aarch64
    #[arg(long, required = true)]
    pub flake_arch: String,

    /// Phase for the branch name: e.g., phase0
    #[arg(long, required = true)]
    pub flake_phase: String,

    /// Step for the branch name: e.g., step1
    #[arg(long, required = true)]
    pub flake_step: String,

    /// Enable rustc wrapper for logging calls
    #[arg(long, default_value_t = false)]
    pub use_rustc_wrapper: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    orchestrate_flake_generation(
        &args.metadata_path,
        &args.expanded_output_dir,
        &args.flake_lock_path,
        args.layer,
        args.package_filter,
        false, // dry_run - hardcoded for now
        false, // force - hardcoded for now
        args.rustc_version,
        args.rustc_host,
        &args.project_root,
        &args.json_summary_path,
        &args.log_output_dir,
        &args.flake_output_dir,
        args.flake_component,
        args.flake_arch,
        args.flake_phase,
        args.flake_step,
        args.use_rustc_wrapper,
    ).await?;

    Ok(())
}