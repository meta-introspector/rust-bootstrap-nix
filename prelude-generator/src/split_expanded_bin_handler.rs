use anyhow::Context;
use std::path::{PathBuf};
use std::fs;

use split_expanded_lib::ErrorCollection;
use split_expanded_lib::{Declaration, RustcInfo};
use crate::gem_parser::GemConfig;
use toml;
use split_expanded_lib::SerializableDeclaration;
use crate::validation::{DeclarationValidator, DependencyValidator};
use crate::symbol_map::SymbolMap;
use crate::reference_visitor::ReferenceVisitor;

pub async fn handle_split_expanded_bin(inputs: crate::types::SplitExpandedBinInputs<'_>) -> anyhow::Result<()> {
    println!("Running split-expanded-bin functionality...");

    let files_to_process = inputs.files_to_process;
    let project_root = inputs.project_root;
    let rustc_version = inputs.rustc_version;
    let rustc_host = inputs.rustc_host;
    let verbose = inputs.verbose;
    let _output_global_toml = inputs.output_global_toml;
    let _output_symbol_map = inputs.output_symbol_map;
    let warnings = inputs.warnings;
    let canonical_output_root = inputs.canonical_output_root;

    if project_root.exists() {
        fs::remove_dir_all(&project_root).context(format!("Failed to remove existing project root: {:?}", project_root))?;
    }
    fs::create_dir_all(&project_root).context(format!("Failed to create project root: {:?}", project_root))?;

    let mut error_collection = ErrorCollection::default();
    let gem_config = GemConfig::load_from_file(&PathBuf::from("gems.toml"))?;
    let mut all_declarations: Vec<Declaration> = Vec::new();
    let dependency_validator = DependencyValidator;

    let mut symbol_map = SymbolMap::new();
    // Populate symbol_map with built-ins from gems.toml
    for gem_entry in &gem_config.gem {
        for identifier in &gem_entry.identifiers {
            symbol_map.add_declaration(
                identifier.clone(),
                "builtin".to_string(),
                gem_entry.crate_name.clone(),
                gem_entry.crate_name.clone(),
            );
        }
    }

    // Populate symbol_map with initial cargo metadata if needed, or leave empty for incremental build
    symbol_map.populate_from_cargo_metadata(&project_root)?;
    if files_to_process.is_empty() {
        println!("No expanded files provided for processing.");
        return Ok(());
    }

    let rustc_info = RustcInfo { version: rustc_version, host: rustc_host };

    let mut parsed_files: Vec<(PathBuf, syn::File)> = Vec::new();

    // --- Pass 1: Collect Declarations ---
    println!("\n--- Pass 1: Collecting Declarations ---");
    for file_path in &files_to_process {
        println!("Processing file for declarations: {:?}", file_path);

        let current_crate_name = project_root.file_name().and_then(|s| s.to_str()).unwrap_or("unknown_crate").to_string();

        match split_expanded_lib::processing::extract_declarations_from_single_file(
            file_path,
            &rustc_info,
            &current_crate_name,
            verbose,
            warnings,
            canonical_output_root,
        ).await {
            Ok(extraction_result) => {
                let declarations = extraction_result.declarations;
                let errors = extraction_result.errors;
                // _file_metadata and _public_symbols are not directly used here,
                // but can be accessed if needed:
                // let _file_metadata = extraction_result.file_metadata;
                // let _public_symbols = extraction_result.public_symbols;

                // Store the parsed file for Pass 2 if parsing was successful
                let file_content = fs::read_to_string(&file_path)
                    .context(format!("Failed to read file: {:?}", file_path))?;
                match syn::parse_file(&file_content) {
                    Ok(file) => parsed_files.push((file_path.clone(), file)),
                    Err(e) => {
                        eprintln!("Warning: Could not re-parse file for Pass 2 {}: {}", file_path.display(), e);
                        // Collect this error as well if needed
                    }
                }

                for (identifier, decl) in declarations {
                    match dependency_validator.validate(&decl) {
                        Ok(_) => all_declarations.push(decl),
                        Err(e) => {
                            eprintln!("Validation Error for declaration {:?}: {:?}", identifier, e);
                            // Depending on desired behavior, you might want to stop here or collect errors
                        }
                    }
                }
                error_collection.errors.extend(errors);
            },
            Err(e) => {
                eprintln!("Error extracting declarations from file {:?}: {}", file_path, e);
                // Collect this error as well if needed
            }
        }
    }

    // --- Pass 2: Resolve References ---
    println!("\n--- Pass 2: Resolving References ---");
    for (file_path, file) in parsed_files {
        println!("Processing file for references: {:?}", file_path);

        let current_crate_name = project_root.file_name().and_then(|s| s.to_str()).unwrap_or("unknown_crate").to_string();
        let current_module_path = file_path.strip_prefix(&project_root).unwrap_or(&file_path).with_extension("").to_str().unwrap_or("unknown_module").to_string().replace("/", "::");

        let mut visitor = ReferenceVisitor::new(
            &mut symbol_map,
            &mut all_declarations, // Pass all_declarations for potential future updates
            current_crate_name,
            current_module_path,
            verbose,
        );
        syn::visit::Visit::visit_file(&mut visitor, &file);
    }

    if let Some(output_path) = _output_global_toml {
        let serializable_decls: Vec<SerializableDeclaration> = all_declarations.into_iter().map(Into::into).collect();
        let toml_string = toml::to_string_pretty(&serializable_decls)
            .context("Failed to serialize declarations to TOML ")?;
        fs::write(&output_path, toml_string)
            .context(format!("Failed to write TOML to file: {:?}", output_path))?;
        println!("Successfully wrote declarations to {:?}", output_path);
    }

    if let Some(output_path) = _output_symbol_map {
        let toml_string = toml::to_string_pretty(&symbol_map.map)
            .context("Failed to serialize symbol map to TOML ")?;
        fs::write(&output_path, toml_string)
            .context(format!("Failed to write symbol map to file: {:?}", output_path))?;
        println!("Successfully wrote symbol map to {:?}", output_path);
    }

    if !error_collection.errors.is_empty() {
        let errors_json_path = project_root.join("collected_errors.json ");
        error_collection.write_to_file(&errors_json_path).await?;
        eprintln!("Errors collected during processing. See {:?} for details.", errors_json_path);
    }

    println!("Split expanded bin functionality completed.");
    Ok(())
}
