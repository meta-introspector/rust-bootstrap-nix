use clap::Parser;
use anyhow::{Result};
use std::path::PathBuf;
use expanded_code_collector::{collect_expanded_code};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the cargo metadata JSON file.
    #[clap(long, value_parser, required = true)]
    metadata_path: PathBuf,

    /// Output directory for expanded code and reports.
    #[clap(long, value_parser, required = true)]
    output_dir: PathBuf,

    /// Layer of expansion (e.g., 0 for initial expansion).
    #[clap(long, value_parser)]
    layer: Option<u32>,

    /// Optional: Collect expanded code only for a specific package.
    #[clap(long, value_parser)]
    package: Option<String>,

    /// If true, no files will be written to disk.
    #[clap(long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    collect_expanded_code(
        &args.metadata_path,
        &args.output_dir,
        args.layer,
        args.package,
        args.dry_run,
    ).await?;

    Ok(())
}