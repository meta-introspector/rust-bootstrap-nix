use anyhow::Context;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use quote::quote;

use crate::use_statements;
use crate::utils;
use crate::type_extractor;

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

    let mut numerical_constants: Vec<syn::ItemConst> = Vec::new();
    let mut string_constants: Vec<syn::ItemConst> = Vec::new();

    for constant in all_constants {
        if let syn::Expr::Lit(expr_lit) = &*constant.expr {
            match &expr_lit.lit {
                syn::Lit::Int(_) | syn::Lit::Float(_) => numerical_constants.push(constant),
                syn::Lit::Str(_) => string_constants.push(constant),
                _ => { /* Handle other literal types or ignore */ }
            }
        } else { /* Handle non-literal expressions or ignore */ }
    }

    crate::constant_storage::numerical_constants::write_numerical_constants_to_hierarchical_structure(
        &numerical_constants,
        &numerical_output_dir,
    ).await?;

    crate::constant_storage::string_constants::write_string_constants_to_hierarchical_structure(
        &string_constants,
        &string_output_dir,
    ).await?;

    println!("  -> Generated constants will be written to layer-specific directories.");

    let mut errors: Vec<anyhow::Error> = Vec::new();

    for constant in numerical_constants.iter().chain(string_constants.iter()) {
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
            eprintln!(r"{:{}}", error);
        }
        eprintln!(r"-----------------------------------------------------");
        return Err(anyhow::anyhow!("Constant processing completed with errors."));
    } else {
        println!(r"Declaration processing completed successfully.");
        crate::constant_reporting::generate_numerical_constants_report(&numerical_output_dir).await?;
        return Ok(())
    }
}

pub fn generate_constants_module(constants: &[syn::ItemConst]) -> String {
    let generated_decl_strings: Vec<String> = constants.iter().map(|c| {
        let tokens = quote! { #c };
        tokens.to_string()
    }).collect();

    if generated_decl_strings.is_empty() {
        return "// No constant declarations found in this module.\n".to_string();
    }

    let header = "// This module contains extracted constant declarations.\n// It is automatically generated.\n\n";
    let joined_decls = generated_decl_strings.join("\n\n");

    format!("{}{}", header, joined_decls)
}
