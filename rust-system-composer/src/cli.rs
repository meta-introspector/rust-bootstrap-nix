use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file.
    #[arg(long, default_value = "../config.toml")]
    pub config_file: PathBuf,

    /// Verbosity level (0-3).
    #[arg(short, long, default_value_t = 0)]
    pub verbosity: u8,

    /// Whether to compile the generated crates.
    #[arg(long, default_value_t = false)]
    pub compile: bool,

    /// Optional package filter for cargo metadata and split-expanded-bin.
    #[arg(long)]
    pub package_filter: Option<String>,

    /// Optional root directory for generated declarations.
    #[arg(long)]
    pub generated_declarations_root: Option<PathBuf>,

    /// Optional project path for vendorization.
    #[arg(long)]
    pub project_path: Option<PathBuf>,

    /// Optional output vendor directory for vendorization.
    #[arg(long)]
    pub output_vendor_dir: Option<PathBuf>,

    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the self-composition workflow.
    SelfCompose {},
    /// Run the rustc composition workflow.
    RustcCompose {},
    /// Run the standalonex composition workflow.
    StandaloneXCompose {},
    /// Update system.toml with project configuration.
    UpdateSystemToml {},
    /// Run the vendorization workflow.
    Vendorize {},
    /// Run the layered composition workflow.
    LayeredCompose(LayeredComposeArgs),
}

#[derive(Parser, Debug)]
pub struct LayeredComposeArgs {
    /// Path to output the code graph.
    #[arg(long)]
    pub code_graph_output_path: Option<PathBuf>,

    /// Path to output the topological sort result.
    #[arg(long)]
    pub topological_sort_output_path: Option<PathBuf>,

    /// Directory to output per-file reports.
    #[arg(long)]
    pub per_file_report_dir: Option<PathBuf>,

    /// Path to output the Command object usage report.
    #[arg(long)]
    pub command_report_output_path: Option<PathBuf>,

    /// Path to output the CollectedAnalysisData JSON file.
    #[arg(long)]
    pub output_analysis_data_json: Option<PathBuf>,
}
