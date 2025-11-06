pub mod args;
pub mod ast_stats;
pub mod bag_of_words_visitor;
pub mod cli;
pub mod code_generator;
pub mod command_handlers;
pub mod config_parser;
pub mod constant_reporting;
pub mod constant_storage;
pub mod declaration_processing;
pub mod dependency_analyzer;
pub mod enum_lattice_info;
pub mod error_handling;
pub mod expression_info;

//pub mod external_interfaces;
pub mod gem_parser;
pub mod generate_prelude;
pub mod impl_lattice_info;
pub mod measurement;
pub mod modify_crate_root;
pub mod modify_file;
pub mod module_exporter;
//pub mod module_verifier;
pub mod parser;
pub mod pipeline;
pub mod prelude_category_pipeline;
pub mod processor;
pub mod processor_tests;
pub mod public_tests;
pub mod reference_visitor;
pub mod report;
pub mod report_generator;
pub mod split_expanded_bin_handler;
pub mod struct_lattice_info;
pub mod symbol_map;
pub mod test_extractor;
pub mod type_collector;
pub mod type_extractor;
pub mod type_usage_analyzer;
pub mod type_usage_visitor;
pub mod types;
pub mod use_extractor;
pub mod use_statements;
pub mod utils;
pub mod validation;
pub use args::Args;
pub use ast_decoder::AstTraversalFunctor;
pub use bag_of_words_visitor::{BagOfWordsVisitor, tokenize_ident_to_subwords};
pub use generate_prelude::generate_prelude;
pub use modify_crate_root::modify_crate_root;
pub use modify_file::modify_file;
pub use pipeline_traits::{AstStatistics, VariableInfo, FunctionInfo, ImportInfo};
pub use processor::process_crates;
pub use report::generate_report;
pub use split_expanded_lib::{Declaration, ErrorSample, RustcInfo, FileMetadata as SplitExpandedFileMetadata, SerializableDeclaration, PublicSymbol};
pub use test_extractor::{collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};
pub use types::{FileProcessingResult, FileProcessingStatus, CollectedPreludeInfo};
use crate::symbol_map::SymbolMap;

pub async fn extract_declarations_for_composer(
    file_path: &std::path::Path,
    rustc_info: &RustcInfo,
    crate_name: &str,
    verbose: u8,
) -> anyhow::Result<(Vec<Declaration>, SymbolMap, Vec<ErrorSample>, SplitExpandedFileMetadata, Vec<PublicSymbol>)> {
            let (declarations, symbol_map, errors, file_metadata, public_symbols) = crate::declaration_processing::extract_all_declarations_from_file(
                file_path,
                &std::path::PathBuf::new(), // Placeholder for output_dir, not used by new function
                false, // Placeholder for dry_run, not used by new function
                verbose,
                &rustc_info,
                crate_name,
            ).await?;
        Ok((declarations, symbol_map, errors, file_metadata, public_symbols))
}
