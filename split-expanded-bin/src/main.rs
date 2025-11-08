use clap::Parser;
use anyhow::Context;
use std::path::{PathBuf};
use split_expanded_lib::process_expanded_manifest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set verbosity level (0 = silent, 1 = normal, 2 = detailed, 3 = debug).
    #[arg(short, long, default_value_t = 1)]
    pub verbosity: u8,

    /// Path to the expanded_manifest.json file.
    #[arg(long)]
    pub expanded_manifest: PathBuf,

    /// Directory to output the generated declaration files.
    #[clap(short, long, value_parser, required = true)]
    project_root: PathBuf,

    /// Rustc version (e.g., "1.89.0").
    #[arg(long)]
    pub rustc_version: String,

    /// Rustc host triple (e.g., "aarch64-unknown-linux-gnu").
    #[arg(long)]
    pub rustc_host: String,

    /// Optional: Process only crates at a specific dependency layer.
    #[arg(long)]
    layer: Option<u32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    process_expanded_manifest(
        &args.expanded_manifest,
        &args.project_root,
        args.rustc_version,
        args.rustc_host,
        args.verbosity,
        args.layer,
    ).await?;

    Ok(())
}