use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use super::traits::is_runnable::IsRunnable;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to the configuration file.
    #[arg(long)]
    pub config_file: Option<PathBuf>,

    /// Optional project root path for analysis.
    #[arg(long)]
    pub project_root: Option<PathBuf>,

    /// Verbosity level (0-3).
    #[arg(short, long, default_value = "0")]
    pub verbosity: u8,

    /// Compile generated crates.
    #[arg(long, default_value = "false")]
    pub compile: bool,

    /// Filter packages to process by name.
    #[arg(long)]
    pub package_filter: Option<String>,

    /// Root directory for generated declarations.
    #[arg(long)]
    pub generated_declarations_root: Option<PathBuf>,

    /// Path to the project for vendorization.
    #[arg(long)]
    pub project_path: Option<PathBuf>,

    /// Output directory for vendored dependencies.
    #[arg(long)]
    pub output_vendor_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Composes layered Rust projects based on configuration.
    LayeredCompose(LayeredComposeArgs),
    /// Reports on `std::process::Command` object usage.
    CommandReport {
        /// Path to output the command report JSON.
        #[arg(long)]
        output_path: PathBuf,
    },
    /// Runs the self-composition workflow.
    SelfCompose,
    /// Runs the rustc composition workflow.
    RustcCompose,
    /// Runs the standalonex composition workflow.
    StandaloneXCompose,
    /// Updates the system.toml with project configuration.
    UpdateSystemToml,
    /// Runs the vendorization workflow.
    Vendorize,
}

#[derive(Args, Debug)]
pub struct LayeredComposeArgs {
    /// Path to output the collected analysis data JSON.
    #[arg(long)]
    pub output_analysis_data_json: PathBuf,

    /// Path to output the CodeGraph JSON.
    #[arg(long)]
    pub code_graph_output_path: Option<PathBuf>,

    /// Path to output the command report JSON.
    #[arg(long)]
    pub command_report_output_path: Option<PathBuf>,

    /// Path to output the topological sort results JSON.
    #[arg(long)]
    pub topological_sort_output_path: Option<PathBuf>,

    /// Directory to output per-file compilation reports.
    #[arg(long)]
    pub per_file_report_dir: Option<PathBuf>,

    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub generate_lock_only: bool, // New flag
    #[arg(long)]
    pub skip_prelude_info: bool,
    #[arg(long)]
    pub force_prelude_info: bool,

    /// Skip the type analysis stage.
    #[arg(long)]
    pub skip_type_analysis: bool,
    /// Force re-run the type analysis stage, ignoring cache.
    #[arg(long)]
    pub force_type_analysis: bool,

    /// Skip the code graph flattening stage.
    #[arg(long)]
    pub skip_graph_flattening: bool,
    /// Force re-run the code graph flattening stage, ignoring cache.
    #[arg(long)]
    pub force_graph_flattening: bool,

    /// Skip the layered crate organizer stage.
    #[arg(long)]
    pub skip_crate_organizer: bool,
    /// Force re-run the layered crate organizer stage, ignoring cache.
    #[arg(long)]
    pub force_crate_organizer: bool,

    /// Skip the command report generation stage.
    #[arg(long)]
    pub skip_command_report: bool,
    /// Force re-run the command report generation stage, ignoring cache.
    #[arg(long)]
    pub force_command_report: bool,

    /// Path to the config.lock file for caching and reproducibility.
    #[arg(long)]
    pub config_lock_path: Option<PathBuf>,
    
    


}

impl IsRunnable for LayeredComposeArgs {
    fn is_dry_run(&self) -> bool {
        self.dry_run
    }

    fn set_dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }
}
