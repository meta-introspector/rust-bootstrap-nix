use clap::Parser;
use anyhow::{Result, Context};
use std::path::PathBuf;
use expanded_code_collector::{collect_expanded_code};
use tokio::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the cargo metadata JSON file.
    #[clap(long, value_parser, required = true)]
    metadata_path: PathBuf,

    /// Output directory for expanded code and reports.
    #[clap(long, value_parser, required = true)]
    output_dir: PathBuf,

    /// Path to the flake.lock file.
    #[clap(long, value_parser, required = true)]
    flake_lock_path: PathBuf,

    /// Layer of expansion (e.g., 0 for initial expansion).
    #[clap(long, value_parser)]
    layer: Option<u32>,

    /// Optional: Collect expanded code only for a specific package.
    #[clap(long, value_parser)]
    package: Option<String>,

    /// If true, no files will be written to disk.
    #[clap(long)]
    dry_run: bool,

    /// If true, overwrite existing expanded code files.
    #[clap(long)]
    force: bool,
}

async fn get_rustc_info() -> Result<(String, String)> {
    let output = Command::new("rustc")
        .arg("--version")
        .arg("--verbose")
        .output()
        .await
        .context("Failed to execute rustc command")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut version = String::new();
    let mut host = String::new();

    for line in stdout.lines() {
        if line.starts_with("rustc ") {
            version = line.to_string();
        } else if line.starts_with("host: ") {
            host = line.replace("host: ", "").to_string();
        }
    }

    if version.is_empty() || host.is_empty() {
        anyhow::bail!("Could not parse rustc version or host from `rustc --version --verbose` output");
    }

    Ok((version, host))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let flake_lock_content = tokio::fs::read_to_string(&args.flake_lock_path)
        .await
        .context(format!("Failed to read flake.lock file: {}", args.flake_lock_path.display()))?;
    let flake_lock_json: serde_json::Value = serde_json::from_str(&flake_lock_content)
        .context(format!("Failed to parse flake.lock JSON from: {}", args.flake_lock_path.display()))?;

    let (rustc_version, rustc_host) = get_rustc_info().await?;

    collect_expanded_code(
        &args.metadata_path,
        &args.output_dir,
        &flake_lock_json,
        args.layer,
        args.package,
        args.dry_run,
        args.force,
        rustc_version,
        rustc_host,
    ).await?;

    Ok(())
}