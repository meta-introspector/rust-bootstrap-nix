use anyhow::Context;
use std::path::{PathBuf, Path};
use syn::visit::Visit;
use quote::quote;

use crate::decls_visitor::DeclsVisitor;
use std::collections::HashMap;
use crate::use_extractor::expand_macros_and_parse;
use crate::use_extractor::rustc_info::RustcInfo;
use crate::type_extractor;
use crate::error_collector::ErrorSample;
use crate::declaration::Declaration;
use crate::gem_parser::GemConfig;

pub async fn extract_all_declarations_from_crate(
    manifest_path: &Path,
    _args: &crate::Args,
    _type_map: &HashMap<String, type_extractor::TypeInfo>,
    filter_names: &Option<Vec<String>>,
    rustc_info: &RustcInfo,
    cache_dir: &Path,
    gem_config: &GemConfig,
) -> anyhow::Result<(
    Vec<Declaration>,
    usize, // total_files_processed
    usize, // total_fns
    usize, // total_structs
    usize, // total_enums
    usize, // total_statics
    usize, // total_other_items
    HashMap<usize, usize>, // total_structs_per_layer
    Vec<ErrorSample>,
)> {
    let mut all_declarations: Vec<Declaration> = Vec::new();
    let mut all_collected_errors: Vec<ErrorSample> = Vec::new();
    let mut total_files_processed = 0;
    let mut total_fns = 0;
    let mut total_structs = 0;
    let mut total_enums = 0;
    let mut total_statics = 0;
    let mut total_other_items = 0;
    let total_structs_per_layer: HashMap<usize, usize> = HashMap::new();

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(manifest_path)
        .exec()?;

    let package = metadata.packages.into_iter().find(|p| p.manifest_path == manifest_path.to_path_buf())
        .context(format!("Package with manifest path {} not found in metadata", manifest_path.display()))?;

    let crate_root = manifest_path.parent().unwrap();

    for target in package.targets {
        if target.kind.contains(&"lib".to_string()) || target.kind.contains(&"bin".to_string()) {
            let path = target.src_path.into_std_path_buf();
            if let Some(names) = filter_names {
                if !names.iter().any(|name| path.to_string_lossy().contains(name)) {
                    continue;
                }
            }

            total_files_processed += 1;
            println!("  Processing file: {}", path.display());

            let mut writer = tokio::io::stdout();
            let (file, error_sample) = expand_macros_and_parse(&mut writer, &path, &crate_root, rustc_info, cache_dir).await
                .with_context(|| format!("Failed to expand macros and parse file: {}", path.display()))?;

            if let Some(err) = error_sample {
                all_collected_errors.push(err);
                continue;
            }

            let mut visitor = DeclsVisitor::new(gem_config);
            visitor.visit_file(&file);

            all_declarations.extend(visitor.declarations);

            total_fns += visitor.fn_count;
            total_structs += visitor.struct_count;
            total_enums += visitor.enum_count;
            total_statics += visitor.static_count;
            total_other_items += visitor.other_item_count;
        }
    }

    // Filter declarations with no dependencies
    // This filtering logic will be moved to a separate layering function.
    // let filtered_declarations: Vec<Declaration> = all_declarations.into_iter().filter(|decl| {
    //     decl.referenced_types.is_empty() && decl.referenced_functions.is_empty() && decl.external_identifiers.is_empty()
    // }).collect();


    Ok((
        all_declarations,
        total_files_processed,
        total_fns,
        total_structs,
        total_enums,
        total_statics,
        total_other_items,
        total_structs_per_layer,
        all_collected_errors,
    ))
}

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

        tokio::fs::write(&output_path, content.as_bytes()).await
            .context(format!("Failed to write constant {:?} to {:?}", const_name, output_path))?;
        println!("  -> Wrote constant {:?} to {:?}", const_name, output_path);

        // Determine if it's a numerical or string constant and add to respective vectors
        if let syn::Expr::Lit(expr_lit) = &constant.expr.as_ref() {
            match &expr_lit.lit {
                syn::Lit::Int(_) | syn::Lit::Float(_) => all_numerical_constants.push(constant.clone()),
                syn::Lit::Str(_) => all_string_constants.push(constant.clone()),
                _ => {},
            }
        }
    }

    Ok(())
}

pub fn layer_declarations(
    all_declarations: Vec<Declaration>,
) -> HashMap<usize, Vec<Declaration>> {
    let mut layered_decls: HashMap<usize, Vec<Declaration>> = HashMap::new();
    let mut remaining_decls = all_declarations;
    let mut current_layer_num = 0;

    loop {
        if remaining_decls.is_empty() {
            break;
        }

        let mut current_layer_decls = Vec::new();
        let mut next_remaining_decls = Vec::new();
        let mut current_layer_idents = std::collections::HashSet::new();

        // Identify declarations for the current layer
        for decl in remaining_decls.into_iter() {
            let has_unresolved_deps = decl.referenced_types.iter().any(|dep| {
                !current_layer_idents.contains(dep) && !layered_decls.values().flatten().any(|d| d.get_identifier() == *dep)
            });

            if !has_unresolved_deps {
                current_layer_idents.insert(decl.get_identifier());
                current_layer_decls.push(decl);
            } else {
                next_remaining_decls.push(decl);
            }
        }

        if current_layer_decls.is_empty() {
            // No new declarations could be layered, break to prevent infinite loop
            break;
        }

        layered_decls.insert(current_layer_num, current_layer_decls);
        remaining_decls = next_remaining_decls;
        current_layer_num += 1;

        if current_layer_num > 8 { // Stop at layer 8 as per requirement
            break;
        }
    }

    layered_decls
}
