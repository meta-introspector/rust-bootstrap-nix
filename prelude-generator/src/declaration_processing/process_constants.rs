use anyhow::Context;
use std::path::PathBuf;
use std::collections::HashMap;
use quote::quote;

use crate::type_extractor;

pub async fn process_constants(
    all_constants: Vec<syn::ItemConst>,
    args: &crate::Args,
    project_root: &PathBuf,
    all_numerical_constants: &mut Vec<syn::ItemConst>,
    all_string_constants: &mut Vec<syn::ItemConst>,
    _type_map: &HashMap<String, type_extractor::TypeInfo>,
) -> anyhow::Result<()> {
    let generated_decls_output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });
    let constants_output_dir = generated_decls_output_dir.join("constants");
    tokio::fs::create_dir_all(&constants_output_dir).await
        .context(format!("Failed to create output directory {:?}", constants_output_dir))?;

    println!("  -> Generated constants will be written to: {:?}", constants_output_dir);

    for constant in all_constants {
        let const_name = constant.ident.to_string();
        let file_name = format!("{}.rs", const_name);
        let output_path = constants_output_dir.join(&file_name);
        let content = quote! { #constant }.to_string();

        tokio::fs::write(&output_path, content).await
            .context(format!("Failed to write constant {:?} to {:?}", const_name, output_path))?;
        println!("  -> Wrote constant {:?} to {:?}", const_name, output_path);

        // Determine if it's a numerical or string constant and add to respective vectors
        if let syn::Expr::Lit(expr_lit) = &constant.expr.as_ref() {
            match &expr_lit.lit {
                syn::Lit::Int(_) | syn::Lit::Float(_) => all_numerical_constants.push(constant.clone()),
                syn::Lit::Str(_) => all_string_constants.push(constant.clone()),
                _ => {{}},
            }
        }
    }

    Ok(())
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
