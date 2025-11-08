use std::path::Path;
use split_expanded_lib::{Declaration, ErrorSample, FileMetadata, RustcInfo, PublicSymbol};
use crate::symbol_map::SymbolMap;

pub async fn extract_all_declarations_from_file(
    file_path: &Path,
    _output_dir: &Path, // output_dir is not directly used by split_expanded_lib's function
    _dry_run: bool,     // dry_run is not directly used by split_expanded_lib's function
    verbose: u8,
    rustc_info: &RustcInfo, // Pass RustcInfo to the new function
    crate_name: &str,       // Pass crate_name to the new function
) -> anyhow::Result<(Vec<Declaration>, SymbolMap, Vec<ErrorSample>, FileMetadata, Vec<PublicSymbol>)> {
    let (declarations, errors, file_metadata, public_symbols) = split_expanded_lib::processing::extract_declarations_from_single_file(
        file_path,
        rustc_info,
        crate_name,
        verbose,
    ).await?;

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

    Ok((declarations_vec, symbol_map, errors, file_metadata, public_symbols))
}
