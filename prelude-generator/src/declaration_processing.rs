use anyhow::Context;
use std::path::PathBuf;
use syn::visit::Visit;
use quote::quote;

use crate::{Level0DeclsVisitor, use_statements, utils, type_extractor};
use std::collections::HashMap;

pub async fn extract_level0_declarations(
    project_root: &PathBuf,
    _args: &crate::Args,
    type_map: &HashMap<String, type_extractor::TypeInfo>,
    filter_names: &Option<Vec<String>>,
) -> anyhow::Result<(
    Vec<syn::ItemConst>,
    Vec<syn::ItemStruct>,
    usize, // total_files_processed
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
)> {
    let src_dir = project_root.join("prelude-generator/src");
    println!("Attempting to read directory: {}", src_dir.display());
    let mut all_constants: Vec<syn::ItemConst> = Vec::new();
    let mut all_layer0_structs: Vec<syn::ItemStruct> = Vec::new();
    let mut total_files_processed = 0;
    let mut total_fns = 0;
    let mut total_structs = 0;
    let mut total_enums = 0;
    let mut total_statics = 0;
    let mut total_other_items = 0;
    let mut total_layer0_structs = 0;

    for entry in std::fs::read_dir(&src_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| {
            if let Some(names) = filter_names {
                names.iter().any(|name| e.file_name().to_string_lossy().contains(name))
            } else {
                true
            }
        })
    {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            total_files_processed += 1;
            println!("  Processing file: {}", path.display());
            let content = std::fs::read_to_string(&path)?;
            let file = syn::parse_file(&content)?;

            let mut visitor = Level0DeclsVisitor::new();
            visitor.visit_file(&file);

            // Filter structs based on layer information
            for structure in visitor.layer0_structs {
                let struct_name = structure.ident.to_string();
                if type_map.get(&struct_name).map_or(false, |info| info.layer == Some(0)) {
                    all_layer0_structs.push(structure);
                }
            }

            all_constants.extend(visitor.constants);

            total_fns += visitor.fn_count;
            total_structs += visitor.struct_count;
            total_enums += visitor.enum_count;
            total_statics += visitor.static_count;
            total_other_items += visitor.other_item_count;
            total_layer0_structs += all_layer0_structs.len(); // Update count after filtering
        }
    }

    Ok((
        all_constants,
        all_layer0_structs,
        total_files_processed,
        total_fns,
        total_structs,
        total_enums,
        total_statics,
        total_other_items,
        total_layer0_structs,
    ))
}

pub async fn process_constants(
    all_constants: Vec<syn::ItemConst>,
    _args: &crate::Args,
    project_root: &PathBuf,
    _all_numerical_constants: &mut Vec<syn::ItemConst>,
    _all_string_constants: &mut Vec<syn::ItemConst>,
    type_map: &HashMap<String, type_extractor::TypeInfo>,
) -> anyhow::Result<()> {
    let generated_decls_output_dir = _args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });

    let numerical_output_dir = project_root.join("generated/numerical_constants");
    println!("Attempting to create numerical constants output directory: {}", numerical_output_dir.display());
    tokio::fs::create_dir_all(&numerical_output_dir).await?;


    let string_output_dir = project_root.join("generated/string_constants");
    println!("Attempting to create string constants output directory: {}", string_output_dir.display());
    tokio::fs::create_dir_all(&string_output_dir).await?;


    println!("  -> Generated constants will be written to layer-specific directories.");

    let mut errors: Vec<anyhow::Error> = Vec::new();

    for constant in &all_constants {
        let const_name = constant.ident.to_string();
        let layer = type_map.get(&const_name).and_then(|info| info.layer).unwrap_or(0);
        let consts_output_dir = generated_decls_output_dir.join(format!("layer_{}", layer)).join("const");
        println!("Attempting to create directory: {}", consts_output_dir.display());
        tokio::fs::create_dir_all(&consts_output_dir).await
            .context(format!("Failed to create output directory {:?}", consts_output_dir))?;

        let file_name = format!("{}.rs", const_name);
        let output_path = consts_output_dir.join(&file_name);
        println!("Attempting to write file: {}", output_path.display());
        let result = async {
            let tokens = quote! { #constant };
            let mut code = tokens.to_string();

            let required_uses = use_statements::get_required_uses_for_item_const(&constant);
            code = format!("{}{}", required_uses, code);

            tokio::fs::write(&output_path, code.as_bytes()).await
                .context(format!("Failed to write constant {:?} to {:?}", const_name, output_path))?;
            println!("  -> Wrote constant {:?} to {:?}", const_name, output_path);

            // Format the generated code
            utils::format_rust_code(&output_path).await
                .context(format!("Constant {:?} formatting failed for {:?}", const_name, output_path))?;
            println!("  -> Constant {:?} formatted successfully.\n", const_name);

            // Validate the generated code
            utils::validate_rust_code(&output_path).await
                .context(format!("Constant {:?} validation failed for {:?}", const_name, output_path))?;
            println!(r"  -> Constant {:?} validated successfully.\n", const_name);
            Ok(())
        }.await;

        if let Err(e) = result {
            eprintln!(r"Error processing constant {}: {:?}\n", const_name, e);
            errors.push(e);
        }
    }

    if !errors.is_empty() {
        eprintln!(r"\n--- Errors Encountered during constant processing ---");
        for error in &errors {
            eprintln!(r"{:?}", error);
        }
        eprintln!(r"-----------------------------------------------------");
        return Err(anyhow::anyhow!("Constant processing completed with errors."));
    } else {
        println!(r"Declaration processing completed successfully.");
        return Ok(());
    }
}

pub async fn process_structs(
    all_layer0_structs: Vec<syn::ItemStruct>,
    args: &crate::Args,
    project_root: &PathBuf,
    type_map: &HashMap<String, type_extractor::TypeInfo>,
) -> anyhow::Result<()> {
    let generated_decls_output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });

    println!("  -> Generated structs will be written to layer-specific directories.");

    let mut errors: Vec<anyhow::Error> = Vec::new();

    for structure in &all_layer0_structs {
        let struct_name = structure.ident.to_string();
        let layer = type_map.get(&struct_name).and_then(|info| info.layer).unwrap_or(0);
        let structs_output_dir = generated_decls_output_dir.join(format!("layer_{}", layer)).join("struct");
        println!("Attempting to create directory: {}", structs_output_dir.display());
        tokio::fs::create_dir_all(&structs_output_dir).await
            .context(format!("Failed to create output directory {:?}", structs_output_dir))?;

        let file_name = format!("{}.rs", struct_name);
        let output_path = structs_output_dir.join(&file_name);
        println!("Attempting to write file: {}", output_path.display());
            let content = quote::quote! { #structure }.to_string();
            let code = match struct_name.as_str() {
                "Level0DeclsVisitor" => {
                    format!("use syn::{{visit::Visit, ItemConst, ItemFn, ItemStruct, ItemEnum, ItemStatic, Item}};
{}", content)
                },
                "TypeCollector" => {
                    format!("use std::collections::HashMap;
use crate::type_extractor::TypeInfo;
{}", content)
                },
                _ => content,
            };

            let result = async {
                tokio::fs::write(&output_path, code.as_bytes()).await
                    .context(format!("Failed to write struct {:?} to {:?}", struct_name, output_path))?;
                println!("  -> Wrote struct {:?} to {:?}", struct_name, output_path);

                // Format the generated code
                utils::format_rust_code(&output_path).await
                    .context(format!("Struct {:?} formatting failed for {:?}", struct_name, output_path))?;
                println!("  -> Struct {:?} formatted successfully.\n", struct_name);

                // Validate the generated code
                utils::validate_rust_code(&output_path).await
                    .context(format!("Struct {:?} validation failed for {:?}", struct_name, output_path))?;
                println!(r"  -> Struct {:?} validated successfully.\n", struct_name);
                Ok(())
            }.await;

            if let Err(e) = result {
                eprintln!(r"Error processing struct {}: {:?}\n", struct_name, e);
                errors.push(e);
            }
    }

    if !errors.is_empty() {
        eprintln!(r"\n--- Errors Encountered during struct processing ---");
        for error in &errors {
            eprintln!(r"{:?}", error);
        }
        eprintln!(r"---------------------------------------------------");
        return Err(anyhow::anyhow!("Struct processing completed with errors."));
    } else {
        println!(r"Declaration processing completed successfully.");
        return Ok(());
    }
}
