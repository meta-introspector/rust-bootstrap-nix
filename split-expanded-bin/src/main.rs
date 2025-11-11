use clap::Parser;
use anyhow::Context;
use std::path::{PathBuf};
use split_expanded_lib::{process_expanded_manifest, ProcessExpandedManifestInputs, RustcInfo};

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

    /// Optional: Filter by package name.
    #[arg(long)]
    package_filter: Option<String>,

    /// Path to the JSON summary file for declarations.
    #[arg(long, required = true)]
    json_summary_path: PathBuf,

    /// Directory for log output.
    #[arg(long, required = true)]
    log_output_dir: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let rustc_info = RustcInfo {
        version: args.rustc_version,
        host: args.rustc_host,
    };

    let inputs = ProcessExpandedManifestInputs {
        expanded_manifest_path: &args.expanded_manifest,
        project_root: &args.project_root,
        rustc_info: &rustc_info,
        verbosity: args.verbosity,
        layer: args.layer,
        canonical_output_root: &args.project_root,
        package_filter: args.package_filter,
        json_summary_path: &args.json_summary_path,
        log_output_dir: &args.log_output_dir,
    };

    process_expanded_manifest(inputs).await?;

    Ok(())
}