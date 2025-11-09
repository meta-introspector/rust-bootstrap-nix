
//#![feature(prelude_import)]
//#[macro_use]
//extern crate std;
//#[prelude_import]
//use std::prelude::rust_2021::*;

use std::path::Path;
use std::collections::HashSet;
use std::fs;
use anyhow::{Context, Result};
use crate::types::CollectedPreludeInfo;
use crate::use_extractor::get_rustc_info;
use cargo_metadata::{MetadataCommand};
use walkdir::WalkDir;
use crate::declaration_processing::extract_all_declarations_from_file;
use split_expanded_lib::RustcInfo as SplitExpandedRustcInfo; // Alias to avoid name collision
use crate::trait_generator::generator::generate_traits;
use crate::trait_generator::writer::write_trait_to_file;
use crate::constant_storage::numerical_constants::write_numerical_constants_to_hierarchical_structure;
use crate::constant_storage::string_constants::write_string_constants_to_hierarchical_structure;
use split_expanded_lib::types::DeclarationItem;
use syn::ItemConst;

pub async fn collect_prelude_info(
    workspace_path: &Path,
    _exclude_crates: &HashSet<String>,
) -> Result<Vec<CollectedPreludeInfo>> {
    let rustc_info = get_rustc_info()?;
    let split_expanded_rustc_info = SplitExpandedRustcInfo {
        version: rustc_info.version,
        host: rustc_info.host,
    };
    let cache_dir = workspace_path.join(".prelude_cache");
    fs::create_dir_all(&cache_dir).context("Failed to create prelude cache directory")?;

    let metadata = MetadataCommand::new()
        .no_deps()
        .current_dir(workspace_path)
        .exec()
        .context("Failed to run cargo metadata")?;

    let mut collected_info_list = Vec::new();

    for package in metadata.packages {
        let mut package_use_statements = HashSet::new();
        let mut package_extern_crates = HashSet::new();
        let package_feature_attributes = HashSet::new(); // Assuming this will be collected later

        let package_src_dir = package.manifest_path.parent().unwrap().join("src");

        let mut all_declarations_extraction_results: Vec<crate::types::AllDeclarationsExtractionResult> = Vec::new();

        if package_src_dir.exists() && package_src_dir.is_dir() {
            for entry in WalkDir::new(&package_src_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
            {
                let file_path = entry.path();
                // Assuming a default verbosity and canonical_output_root for now
                let verbose = 0;
                let canonical_output_root = Path::new("./generated"); // Placeholder

                let extraction_result = extract_all_declarations_from_file(
                    file_path,
                    &Path::new("."), // Placeholder for output_dir
                    false, // Placeholder for dry_run
                    verbose,
                    &split_expanded_rustc_info, // Use the aliased RustcInfo
                    &package.name,
                    &mut Vec::new(), // Placeholder for warnings
                    canonical_output_root,
                ).await?;

                package_use_statements.extend(extraction_result.file_metadata.global_uses.clone());
                package_extern_crates.extend(extraction_result.file_metadata.extern_crates.clone());
                // package_feature_attributes.extend(extraction_result.file_metadata.feature_attributes); // Assuming this will be collected later

                all_declarations_extraction_results.push(extraction_result);
            }
        }

        // Call generate_traits with the collected declarations
        // For now, we'll just pass the first extraction result as a placeholder
        if let Some(first_result) = all_declarations_extraction_results.first() {
            let generated_traits = generate_traits(first_result)?;
            println!("Generated traits for package {}: {:?}", package.name, generated_traits);

            // Determine output directory for generated traits
            let generated_traits_output_dir = workspace_path.join("generated_traits");
            fs::create_dir_all(&generated_traits_output_dir)
                .context("Failed to create generated_traits output directory")?;

            // Write each generated trait to a file
            for generated_trait in &generated_traits {
                write_trait_to_file(&generated_traits_output_dir, generated_trait)?;
            }

            // Collect ItemConsts and write them
            let mut item_consts = Vec::new();
            for declaration in &first_result.declarations {
                if let DeclarationItem::Const(s) = &declaration.item {
                    if let Ok(item_const) = syn::parse_str::<ItemConst>(s) {
                        item_consts.push(item_const);
                    }
                }
            }

            if !item_consts.is_empty() {
                let numerical_constants_output_dir = workspace_path.join("generated_numerical_constants");
                fs::create_dir_all(&numerical_constants_output_dir)
                    .context("Failed to create generated_numerical_constants output directory")?;
                write_numerical_constants_to_hierarchical_structure(&item_consts, &numerical_constants_output_dir).await?;

                let string_constants_output_dir = workspace_path.join("generated_string_constants");
                fs::create_dir_all(&string_constants_output_dir)
                    .context("Failed to create generated_string_constants output directory")?;
                write_string_constants_to_hierarchical_structure(&item_consts, &string_constants_output_dir).await?;
            }
        }


        collected_info_list.push(CollectedPreludeInfo {
            package_name: package.name.clone(),
            manifest_path: package.manifest_path.into_std_path_buf(),
            use_statements: package_use_statements,
            extern_crates: package_extern_crates,
            feature_attributes: package_feature_attributes,
            crate_name: package.name, // Placeholder
            crate_root: package_src_dir.parent().unwrap().to_path_buf().into(), // Placeholder
            prelude_content: String::new(), // Placeholder
            modified_files: Vec::new(), // Placeholder
            crate_root_modified: false, // Placeholder
            file_processing_results: Vec::new(), // Placeholder
        });
    }

    Ok(collected_info_list)
}