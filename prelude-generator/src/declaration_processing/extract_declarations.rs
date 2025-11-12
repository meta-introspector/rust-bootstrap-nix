use std::path::Path;
use split_expanded_lib::{Declaration, RustcInfo};
use crate::symbol_map::SymbolMap;
use syn::visit::Visit; // Added
use anyhow::Context;
use crate::trait_visitors::dependency_analysis_visitor::DependencyAnalysisVisitor; // Added

pub async fn extract_all_declarations_from_file(
    file_path: &Path,
    _output_dir: &Path, // output_dir is not directly used by split_expanded_lib's function
    _dry_run: bool,     // dry_run is not directly used by split_expanded_lib's function
    verbose: u8,
    rustc_info: &RustcInfo, // Pass RustcInfo to the new function
    crate_name: &str,       // Pass crate_name to the new function
    warnings: &mut Vec<String>,
    canonical_output_root: &Path,
) -> anyhow::Result<crate::types::AllDeclarationsExtractionResult> {
    let extraction_result = split_expanded_lib::processing::extract_declarations_from_single_file(
        file_path,
        rustc_info,
        crate_name,
        verbose,
        warnings,
        canonical_output_root,
    ).await?;

    // Parse the file content into a syn::File
    let file_content = tokio::fs::read_to_string(file_path)
        .await
        .context(format!("Failed to read file for AST parsing: {}", file_path.display()))?;
    let syntax_tree = syn::parse_file(&file_content)
        .context(format!("Failed to parse file as Rust syntax tree for dependency analysis: {}", file_path.display()))?;

    // Create an instance of our DependencyAnalysisVisitor
    let mut visitor = DependencyAnalysisVisitor::default();

    // Traverse the AST with our visitor
    visitor.visit_file(&syntax_tree);

    // For now, let's just print the collected dependencies
    if verbose >= 1 {
        warnings.push(format!("Dependencies collected by visitor for {}: {:?}", file_path.display(), visitor.dependencies));
        warnings.push(format!("Types used collected by visitor for {}: {:?}", file_path.display(), visitor.types_used));
    }

    let declarations = extraction_result.declarations;
    let errors = extraction_result.errors;
    let file_metadata = extraction_result.file_metadata;
    let public_symbols = extraction_result.public_symbols;

    let mut symbol_map = SymbolMap::new();
    for (identifier, decl) in &declarations {
        symbol_map.add_declaration(
            identifier.clone(),
            match &decl.item {
                split_expanded_lib::DeclarationItem::Const(_) => "const".to_string(),
                split_expanded_lib::DeclarationItem::Struct(_) => "struct".to_string(),
                split_expanded_lib::DeclarationItem::Enum(_) => "enum".to_string(),
                split_expanded_lib::DeclarationItem::Fn(_) => "function".to_string(),
                split_expanded_lib::DeclarationItem::Static(_) => "static".to_string(),
                split_expanded_lib::DeclarationItem::Macro(_) => "macro".to_string(),
                split_expanded_lib::DeclarationItem::Mod(_) => "module".to_string(),
                split_expanded_lib::DeclarationItem::Trait(_) => "trait".to_string(),
                split_expanded_lib::DeclarationItem::TraitAlias(_) => "trait_alias".to_string(),
                split_expanded_lib::DeclarationItem::Type(_) => "type_alias".to_string(),
                split_expanded_lib::DeclarationItem::Union(_) => "union".to_string(),
                split_expanded_lib::DeclarationItem::Other(_) => "other".to_string(),
            },
            decl.crate_name.clone(),
            decl.source_file.to_string_lossy().to_string(),
        );
    }

    let declarations_vec: Vec<Declaration> = declarations.into_values().collect();

    Ok(crate::types::AllDeclarationsExtractionResult {
        declarations: declarations_vec,
        symbol_map,
        errors,
        file_metadata,
        public_symbols,
    })
}
