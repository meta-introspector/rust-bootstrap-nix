use anyhow::Context;
use std::path::PathBuf;
use std::collections::HashMap;
use quote::quote;

use crate::utils;
use crate::type_extractor;

pub async fn process_structs(
    all_structs_by_layer: HashMap<usize, Vec<syn::ItemStruct>>,
    args: &crate::Args,
    project_root: &PathBuf,
    _type_map: &HashMap<String, type_extractor::TypeInfo>,
) -> anyhow::Result<()> {
    let generated_decls_output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });

    println!("  -> Generated structs will be written to layer-specific directories.");

    let mut errors: Vec<anyhow::Error> = Vec::new();

    // Only process structs for layer 0 for now, as per requirement
    if let Some(layer0_structs) = all_structs_by_layer.get(&0) {
        for structure in layer0_structs {
            let struct_name = structure.ident.to_string();
            let layer = 0; // Explicitly set to 0 for Level 0 processing
            let structs_output_dir = generated_decls_output_dir.join(format!("layer_{}", layer)).join("struct");
            println!("Attempting to create directory: {}", structs_output_dir.display());
            tokio::fs::create_dir_all(&structs_output_dir).await
                .context(format!("Failed to create output directory {:?}, for struct {}", structs_output_dir, struct_name))?;

            let file_name = format!("{}.rs", struct_name);
            let output_path = structs_output_dir.join(&file_name);
            println!("Attempting to write file: {}", output_path.display());
            let content = quote::quote! { #structure }.to_string();
            let code = match struct_name.as_str() {
                "DeclsVisitor" => {
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
    } else {
        println!("No Level 0 structs found to process.");
    }

    if !errors.is_empty() {
        eprintln!(r"\n--- Errors Encountered during struct processing ---");
        for error in &errors {
            eprintln!("{:?}", error);
        }
        eprintln!(r"---------------------------------------------------");
        return Err(anyhow::anyhow!("Struct processing completed with errors."));
    } else {
        println!(r"Declaration processing completed successfully.");
        return Ok(())
    }
}

pub fn generate_structs_module(structs: &[syn::ItemStruct]) -> String {
    let generated_decl_strings: Vec<String> = structs.iter().map(|s| {
        let tokens = quote! { #s };
        tokens.to_string()
    }).collect();

    if generated_decl_strings.is_empty() {
        return "// No struct declarations found in this module.\n".to_string();
    }

    let header = "// This module contains extracted struct declarations.\n// It is automatically generated.\n\n";
    let joined_decls = generated_decl_strings.join("\n\n");

    format!("{}{}", header, joined_decls)
}
