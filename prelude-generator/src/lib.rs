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
pub mod decls_visitor;
pub mod bag_of_words_visitor;
pub mod config_parser;
pub mod cli;
pub mod use_statements;
pub mod utils;
pub mod error_handling;
pub mod error_collector;
pub mod declaration;
pub mod gem_parser;
pub mod ast_stats;
pub mod constant_storage;
pub mod declaration_processing;
pub mod command_handlers;
pub mod type_extractor;
pub mod public_tests;
pub mod split_expanded_bin_handler; // Add this line
pub mod validation;
pub mod symbol_map;
pub mod reference_visitor;
//pub mod global_level0_decls;
pub use args::Args;
//pub use declaration_processing::{extract_level0_declarations, process_structs};
pub use report::generate_report;
pub use generate_prelude::generate_prelude;
pub use modify_file::modify_file;
pub use modify_crate_root::modify_crate_root;
pub use processor::process_crates;
pub use test_extractor::{collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};
pub use pipeline_traits::{AstStatistics, VariableInfo, FunctionInfo, ImportInfo};
pub use ast_decoder::AstTraversalFunctor;

pub use bag_of_words_visitor::{BagOfWordsVisitor, tokenize_ident_to_subwords};

pub mod types;
pub mod collect_prelude_info;
pub use types::{FileProcessingResult, FileProcessingStatus, CollectedPreludeInfo, FileMetadata, RustcInfo, DeclarationExtractionArgs};
pub use collect_prelude_info::*;
pub use crate::declaration::Declaration;
pub use crate::error_collector::ErrorCollection;

use anyhow::Context;

use std::fs;
use syn::parse_file;
use crate::decls_visitor::DeclsVisitor;

use crate::symbol_map::SymbolMap;

pub async fn extract_declarations_for_composer(
    args: DeclarationExtractionArgs,
) -> anyhow::Result<(Vec<Declaration>, ErrorCollection, FileMetadata)> {
    let file_path = args.file_path;
    let _rustc_info = args.rustc_info; // rustc_info is not directly used in DeclsVisitor, but kept for compatibility
    let crate_name = args.crate_name.unwrap_or_else(|| "unknown_crate".to_string());

    let file_content = fs::read_to_string(&file_path)
        .context(format!("Failed to read file: {:?}", file_path))?;

    let file = match parse_file(&file_content) {
        Ok(file) => file,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to parse file {}: {}", file_path.display(), e));
        }
    };

    let mut symbol_map = SymbolMap::new();
    let module_path = file_path.to_string_lossy().to_string(); // A simple approximation
    let verbose = 0; // Default verbose level

    let mut visitor = DeclsVisitor::new(
        Some(file_path.clone()),
        crate_name,
        module_path,
        &mut symbol_map,
        verbose,
    );
    syn::visit::Visit::visit_file(&mut visitor, &file);

    let mut file_metadata = FileMetadata::default();
    for decl in &visitor.declarations {
        file_metadata.global_uses.extend(decl.required_imports.iter().cloned());
        file_metadata.extern_crates.extend(decl.extern_crates.iter().cloned());
    }
    // Feature attributes are not directly collected by DeclsVisitor yet, so it remains empty for now.

    Ok((visitor.declarations, ErrorCollection::default(), file_metadata))
}
