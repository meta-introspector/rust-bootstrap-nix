use std::path::PathBuf;
use anyhow::Context;
use walkdir;

use split_expanded_lib::{Declaration, DeclarationItem, RustcInfo, PublicSymbol};
use crate::Args; // Use prelude_generator's Args

use std::collections::HashMap;
use quote::ToTokens;
use toml;
use crate::types::CollectedProjectInfo;
use crate::declaration_processing::extract_declarations::extract_all_declarations_from_file;
use crate::declaration_processing::layer_declarations::layer_declarations;


pub async fn handle_run_decl_splitter(
    args: &Args,
    project_root: &PathBuf,
    rustc_info: &crate::use_extractor::rustc_info::RustcInfo,
) -> anyhow::Result<()> {
    println!("Running declaration splitter functionality...");

    let input_path = args.path.clone(); // This will now be the path to the expanded .rs file

    let output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| PathBuf::from("./generated_declarations"));

    if output_dir.exists() {
        tokio::fs::remove_dir_all(&output_dir).await.context("Failed to remove existing output directory")?;
    }
    tokio::fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;





    let mut all_declarations_aggregated: Vec<Declaration> = Vec::new();
    let mut all_collected_errors_aggregated: Vec<split_expanded_lib::ErrorSample> = Vec::new();
    let mut symbol_map_aggregated = crate::symbol_map::SymbolMap::new();
    let mut all_public_symbols_aggregated: Vec<split_expanded_lib::PublicSymbol> = Vec::new();

    let rustc_info_for_split_expanded_lib = split_expanded_lib::RustcInfo {
        version: rustc_info.version.clone(),
        host: rustc_info.host.clone(),
    };

    let walker = walkdir::WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"));

    for entry in walker {
        let file_path = entry.path().to_path_buf();
        let current_crate_name = file_path.file_stem().unwrap().to_string_lossy().to_string();

        let should_process_file = args.filter_names.as_ref().map_or(true, |filter_names| {
            filter_names.iter().any(|f| file_path.to_string_lossy().contains(f))
        });

        if should_process_file {
            let (declarations, errors, _file_metadata, public_symbols) = split_expanded_lib::extract_declarations_from_single_file(
                &file_path,
                &rustc_info_for_split_expanded_lib,
                &current_crate_name,
                args.verbose,
            ).await?;

            all_declarations_aggregated.extend(declarations);
            all_collected_errors_aggregated.extend(errors);
            all_public_symbols_aggregated.extend(public_symbols);
        }
    }

    // Save public symbols to a JSON file
    let public_symbols_output_path = output_dir.join("public_symbols.json");
    let json_content = serde_json::to_string_pretty(&all_public_symbols_aggregated)
        .context("Failed to serialize public symbols to JSON")?;
    tokio::fs::write(&public_symbols_output_path, json_content).await
        .context(format!("Failed to write public symbols to file: {:?}", public_symbols_output_path))?;

    println!("Extracted {} public symbols to {:?}", all_public_symbols_aggregated.len(), public_symbols_output_path);

    // Handle errors if any
    if !all_collected_errors_aggregated.is_empty() {
        let error_output_path = output_dir.join("errors.json");
        let error_json_content = serde_json::to_string_pretty(&all_collected_errors_aggregated)
            .context("Failed to serialize errors to JSON")?;
        tokio::fs::write(&error_output_path, error_json_content).await
            .context(format!("Failed to write errors to file: {:?}", error_output_path))?;
        eprintln!("{} errors collected during declaration extraction. See {:?}", all_collected_errors_aggregated.len(), error_output_path);
    }

    // The rest of the function remains the same, processing aggregated data
    // ... (existing code for symbol_map_aggregated, layered_decls, etc.)

    // Placeholder for the rest of the function, which should now use the aggregated data
    // For now, just return Ok(())
    Ok(())
}
