use clap::Parser;
use std::path::PathBuf;
use anyhow::Context; // Add this line

/// Command-line arguments for the prelude generator.
#[derive(Parser, Debug, Clone, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long)]
    pub dry_run: bool,
    /// The path to the workspace root.
    #[arg(long)]
    pub path: PathBuf,
    /// Comma-separated list of crate names to exclude from processing.
    #[arg(long, value_delimiter = ',')]
    pub exclude_crates: Vec<String>,
    /// Generate a summary report of the prelude generation process.
    #[arg(long, default_value_t = false)]
    pub report: bool,
    /// Path to a file to save/load processing results.
    #[arg(long)]
    pub results_file: Option<PathBuf>,
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

    /// Verify the parsed configuration and exit.
    #[arg(long, default_value_t = false)]
    pub verify_config: bool,

    /// Analyze the AST of Rust files in a given path.
    #[arg(long, default_value_t = false)]
    pub analyze_ast: bool,
    /// The path to the Rust project or file to analyze AST for. Only used if `analyze_ast` is true.
    #[arg(long)]
    pub ast_analysis_path: Option<PathBuf>,

    /// Extract all Level 0 declarations (constants) from all modules and write to a global module.
    #[arg(long, default_value_t = false)]
    pub extract_global_level0_decls: bool,

    /// Path to the directory where individually generated Level 0 declaration files will be placed.
    /// Only used if `extract_global_level0_decls` is true.
    #[arg(long)]
    pub generated_decls_output_dir: Option<PathBuf>,

    /// Analyze the bag of words from all identifiers in the project.
    #[arg(long, default_value_t = false)]
    pub analyze_bag_of_words: bool,

    /// Path to the output TOML file for the bag of words report. Only used if `analyze_bag_of_words` is true.
    #[arg(long)]
    pub bag_of_words_output_file: Option<PathBuf>,

    /// Extract and organize numerical constants into a hierarchical directory structure.
    #[arg(long, default_value_t = false)]
    pub extract_numerical_constants: bool,

    /// Extract and organize string constants into a hierarchical directory structure.
    #[arg(long, default_value_t = false)]
    pub extract_string_constants: bool,

    /// Calculate and report the layer of each type in the project.
    #[arg(long, default_value_t = false)]
    pub calculate_layers: bool,

    /// Comma-separated list of file/module names to filter by during processing.
    #[arg(long, value_delimiter = ',')]
    pub filter_names: Option<Vec<String>>,

    /// Path to a rustc wrapper script to use for macro expansion.
    #[arg(long)]
    pub rustc_wrapper_path: Option<PathBuf>,

    // New arguments for split-expanded-bin functionality
    /// Run the split-expanded-bin functionality.
    #[arg(long, default_value_t = false)]
    pub run_split_expanded_bin: bool,

    /// Paths to the input Rust files (e.g., expanded .rs files) for split-expanded-bin.
    #[arg(long)]
    pub split_expanded_files: Vec<PathBuf>,

    /// Directory to output the generated declaration files for split-expanded-bin.
    #[arg(long)]
    pub split_expanded_project_root: Option<PathBuf>,

    /// Rustc version (e.g., "1.89.0") for split-expanded-bin.
    #[arg(long)]
    pub split_expanded_rustc_version: Option<String>,

    /// Rustc host triple (e.g., "aarch64-unknown-linux-gnu") for split-expanded-bin.
    #[arg(long)]
    pub split_expanded_rustc_host: Option<String>,

    /// Path to output the global declarations TOML file for split-expanded-bin.
    #[arg(long)]
    pub split_expanded_output_global_toml: Option<PathBuf>,

    /// Path to output the global symbol map TOML file.
    #[arg(long)]
    pub output_symbol_map: Option<PathBuf>,

    /// Run the declaration splitter functionality.
    #[arg(long, default_value_t = false)]
    pub run_decl_splitter: bool,

    /// Path to output a TOML file containing all declarations, types, modules, and crates.
    #[arg(long)]
    pub output_declarations_toml: Option<PathBuf>,

    /// Analyze type usage in expressions.
    #[arg(long, default_value_t = false)]
    pub analyze_type_usage: bool,

    /// The maximum expression depth to consider for type usage analysis. Required if `analyze_type_usage` is true.
    #[arg(long)]
    pub max_expression_depth: Option<usize>,

    /// Path to output the leveled type usage report. Required if `analyze_type_usage` is true.
    #[arg(long)]
    pub output_type_usage_report: Option<PathBuf>,

    /// Path to output all collected analysis data (expressions, lattices) to a TOML file.
    #[arg(long)]
    pub output_toml_report: Option<PathBuf>,

    /// Path to output all collected analysis data (expressions, lattices) to a JSON file.
    #[arg(long)]
    pub output_analysis_data_json: Option<PathBuf>,

    /// Paths to exclude from processing.
    #[arg(long, value_delimiter = ',')]
    pub exclude_paths: Vec<PathBuf>,
}

impl Args {
    /// Resolves all relative paths in `exclude_paths` to absolute paths based on `self.path`.
    pub fn resolve_exclude_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        let mut resolved_paths = Vec::new();
        for p in &self.exclude_paths {
            let resolved_p = if p.is_relative() {
                if *p == PathBuf::from(".") { // Dereference p here
                    self.path.canonicalize().context("Failed to canonicalize project root path")?
                } else {
                    self.path.join(p).canonicalize().context(format!("Failed to canonicalize exclude path: {:?}", p))?
                }
            } else {
                p.canonicalize().context(format!("Failed to canonicalize absolute exclude path: {:?}", p))?
            };
            resolved_paths.push(resolved_p);
        }
        Ok(resolved_paths)
    }
}
