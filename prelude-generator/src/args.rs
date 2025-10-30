use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments for the prelude generator.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long)]
    pub dry_run: bool,
    /// The path to the workspace root.
    #[arg(long, default_value = ".")]
    pub path: PathBuf,
    /// Comma-separated list of crate names to exclude from processing.
    #[arg(long, value_delimiter = ',')]
    pub exclude_crates: Vec<String>,
    /// Generate a summary report of the prelude generation process.
    #[arg(long, default_value_t = false)]
    pub report: bool,
    /// Path to a file to save/load processing results.
    #[arg(long, default_value = "prelude_processing_results.json")]
    pub results_file: PathBuf,
    /// Generate a report on the prelude cache.
    #[arg(long, default_value_t = false)]
    pub cache_report: bool,
    /// Timeout in seconds for the prelude generation process.
    #[arg(long)]
    pub timeout: Option<u64>,
    /// Force overwriting of files even if they exist.
    #[arg(long, default_value_t = false)]
    pub force: bool,
    /// Generate a JSON report of all unique test cases found in the repository.
    #[arg(long, default_value_t = false)]
    pub generate_test_report: bool,
    /// Path to the output file for the JSON test report. Only used if `generate_test_report` is true.
    #[arg(long)]
    pub test_report_output_file: Option<PathBuf>,
    /// Generate a test verification script and report from a JSON test report.
    #[arg(long, default_value_t = false)]
    pub compile_tests: bool,
    /// Path to the JSON test report input file. Required if `compile_tests` is true.
    #[arg(long)]
    pub test_report_input_file: Option<PathBuf>,
    /// Path to the directory where the test verification script and report will be generated. Required if `compile_tests` is true.
    #[arg(long)]
    pub test_verification_output_dir: Option<PathBuf>,
    /// Extract unique use statements and generate test files for a use statement parser.
    #[arg(long, default_value_t = false)]
    pub extract_use_statements: bool,
    /// Path to the directory where generated use statement test files will be placed. Required if `extract_use_statements` is true.
    #[arg(long)]
    pub use_statements_output_dir: Option<PathBuf>,

    /// Collect and process use statements
    #[clap(long, default_value_t = false)]
    pub collect_and_process_use_statements: bool,

    /// Generate a single test file with all unique use statements
    #[clap(long, default_value_t = false)]
    pub generate_aggregated_test_file: bool,

    /// Run the use statement processing pipeline
    #[clap(long, default_value_t = false)]
    pub run_pipeline: bool,

    /// Specify the stage of the pipeline to run
    #[clap(long)]
    pub stage: Option<String>,

    /// Process files in batches of this size
    #[clap(long)]
    pub batch_size: Option<usize>,

    /// The maximum number of batches to run
    #[clap(long)]
    pub batch_limit: Option<usize>,

    /// Process a single file
    #[clap(long)]
    pub file: Option<String>,


    /// Stop after processing N statements
    #[clap(long, value_parser, default_value_t = 0)]
    pub stop_after: usize,

    /// Timeout in seconds for each processing step
    #[clap(long, value_parser, default_value_t = 0)]
    pub step_timeout: u64,

    /// Enable verbose output
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Path to the hf-validator executable.
    #[arg(long)]
    pub hf_validator_path: Option<PathBuf>,

    /// Path to the main configuration file (config.toml).
    #[arg(long)]
    pub config_file_path: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_default_values() {
        let args = Args::parse_from(&["prelude-generator"]);
        assert!(!args.dry_run);
        assert_eq!(args.path, PathBuf::from("."));
        assert!(args.exclude_crates.is_empty());
        assert!(!args.report);
        assert_eq!(args.results_file, PathBuf::from("prelude_processing_results.json"));
        assert!(!args.cache_report);
        assert!(args.timeout.is_none());
        assert!(!args.force);
    }

    #[test]
    fn test_args_custom_values() {
        let args = Args::parse_from(&[
            "prelude-generator",
            "--dry-run",
            "--path", "/tmp/my_project",
            "--exclude-crates", "crate1,crate2",
            "--report",
            "--results-file", "custom_results.json",
            "--cache-report",
            "--timeout", "60",
            "--force",
        ]);

        assert!(args.dry_run);
        assert_eq!(args.path, PathBuf::from("/tmp/my_project"));
        assert_eq!(args.exclude_crates, vec!["crate1".to_string(), "crate2".to_string()]);
        assert!(args.report);
        assert_eq!(args.results_file, PathBuf::from("custom_results.json"));
        assert!(args.cache_report);
        assert_eq!(args.timeout, Some(60));
        assert!(args.force);
    }
}
