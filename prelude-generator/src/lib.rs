pub mod args;
pub mod report;
pub mod generate_prelude;
pub mod modify_file;
pub mod modify_crate_root;
pub mod processor;
pub mod test_extractor;
pub mod pipeline;
pub mod use_extractor;
pub mod prelude_category_pipeline;
pub mod code_generator;
pub mod measurement;
pub mod parser;
pub mod bag_of_words_visitor;
pub mod config_parser;
pub mod cli;
pub mod use_statements;
pub mod utils;
pub mod error_handling;
pub mod gem_parser;
pub mod ast_stats;
pub mod constant_storage;
pub mod declaration_processing;
pub mod command_handlers;
pub mod type_extractor;
pub mod public_tests;
pub mod split_expanded_bin_handler;
pub mod validation;
pub mod symbol_map;
pub mod reference_visitor;
pub mod struct_lattice_info;
pub mod enum_lattice_info;
pub mod impl_lattice_info;
pub mod dependency_analyzer;
pub mod type_usage_analyzer;
pub mod expression_info;
pub mod type_collector;
pub mod type_usage_visitor;
pub mod report_generator;
pub mod processor_tests;
pub mod trait_visitors;
pub mod conceptual_traits;
pub mod trait_generator; // Added
pub use args::Args;
pub use trait_visitors::vernacular_declaration_visitor::VernacularDeclarationVisitor;
pub use trait_visitors::vernacular_walk::VernacularWalk;
pub use trait_visitors::type_collector_visitor::TypeCollectorVisitor;
pub use trait_generator::generate_traits; // Added
pub use trait_generator::write_trait_to_file; // Added // Added // Added // Added
pub use report::generate_report;
pub use generate_prelude::generate_prelude;
pub use modify_file::modify_file;
pub use modify_crate_root::modify_crate_root;
pub use processor::process_crates;
pub use test_extractor::{collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};
pub use pipeline_traits::{AstStatistics, VariableInfo, FunctionInfo, ImportInfo};
pub use ast_decoder::AstTraversalFunctor;
pub use bag_of_words_visitor::{BagOfWordsVisitor, tokenize_ident_to_subwords};
pub mod constant_reporting;
pub mod types;
pub use types::{FileProcessingResult, FileProcessingStatus, CollectedPreludeInfo};
pub use split_expanded_lib::{Declaration, ErrorSample, RustcInfo, FileMetadata as SplitExpandedFileMetadata, SerializableDeclaration, PublicSymbol};

pub type PipelineConfig = pipeline_traits::Config; // Updated to use pipeline_traits::Config
pub use pipeline_traits::{Config, NixConfig, RustConfig, BuildConfig, EnvConfig, InstallConfig, DistConfig, LlvmConfig, ChangeIdConfig, BinsConfig, ModuleExportsConfig};

pub async fn extract_declarations_for_composer(
    file_path: &std::path::Path,
    rustc_info: &RustcInfo,
    crate_name: &str,
    verbose: u8,
    warnings: &mut Vec<String>,
    canonical_output_root: &std::path::Path,
) -> anyhow::Result<crate::types::AllDeclarationsExtractionResult> {
            let extraction_result = crate::declaration_processing::extract_all_declarations_from_file(
                file_path,
                &std::path::PathBuf::new(), // Placeholder for output_dir, not used by new function
                false, // Placeholder for dry_run, not used by new function
                verbose,
                &rustc_info,
                crate_name,
                warnings,
                canonical_output_root,
            ).await?;
        Ok(extraction_result)
}
