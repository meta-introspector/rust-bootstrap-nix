use std::path::PathBuf;
use anyhow::Context;
use crate::use_extractor::RustcInfo;

use crate::declaration_processing;
use crate::Args; // Use prelude_generator's Args

use std::collections::HashMap;
use quote::quote;
use toml;
use crate::declaration::{DeclarationItem, CollectedProjectInfo};

pub async fn handle_run_decl_splitter(args: &Args) -> anyhow::Result<()> {
    println!("Running declaration splitter functionality...");

    let input_dir = args.path.clone(); // Use args.path as input_dir
    let output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| PathBuf::from("./generated_declarations"));


    if output_dir.exists() {
        tokio::fs::remove_dir_all(&output_dir).await.context("Failed to remove existing output directory")?;
    }
    tokio::fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;

    println!("Input directory: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

    // Create dummy RustcInfo and GemConfig for now
    let rustc_info = RustcInfo { version: "unknown".to_string(), host: "unknown".to_string() };


    // Call prelude-generator's extraction function
    let (all_declarations, _, _, _, _, _, _, _, collected_errors, symbol_map) = 
        declaration_processing::extract_all_declarations_from_crate(
            &input_dir, // Assuming input_dir is the manifest_path for now
            args, // Pass the actual args
            &HashMap::new(), // Empty type_map
            &None, // No filter_names
            &rustc_info,
            &PathBuf::from("./cache"), // Dummy cache_dir
        ).await?;

    if !collected_errors.is_empty() {
        eprintln!("Errors collected during declaration extraction:");
        for error in collected_errors {
            eprintln!("  {:?}\n", error);
        }
    }

    // Layer the declarations
    let layered_decls = declaration_processing::layer_declarations(all_declarations);

    let mut declaration_count = 0;

    for (layer_num, declarations_in_layer) in layered_decls {
        println!("Processing layer {}", layer_num);
        for decl in declarations_in_layer {
            let item_str = match &decl.item {
                DeclarationItem::Const(item) => quote! { #item }.to_string(),
                DeclarationItem::Struct(item) => quote! { #item }.to_string(),
                DeclarationItem::Enum(item) => quote! { #item }.to_string(),
                DeclarationItem::Fn(item) => quote! { #item }.to_string(),
                DeclarationItem::Static(item) => quote! { #item }.to_string(),
                DeclarationItem::Other(item) => quote! { #item }.to_string(),
                DeclarationItem::Macro(item) => quote! { #item }.to_string(),
                DeclarationItem::Mod(item) => quote! { #item }.to_string(),
                DeclarationItem::Trait(item) => quote! { #item }.to_string(),
                DeclarationItem::TraitAlias(item) => quote! { #item }.to_string(),
                DeclarationItem::Type(item) => quote! { #item }.to_string(),
                DeclarationItem::Union(item) => quote! { #item }.to_string(),
            };
            let item_name = decl.get_identifier();

            let level_dir = output_dir.join(layer_num.to_string());
            let decl_dir = level_dir.join(&item_name);

            tokio::fs::create_dir_all(&decl_dir).await.context(format!("Failed to create directory {:?}\n", decl_dir))?;
            tokio::fs::create_dir_all(decl_dir.join("src")).await.context(format!("Failed to create directory {:?}/src\n", decl_dir))?;

            let cargo_toml_content = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
                item_name
            );
            tokio::fs::write(decl_dir.join("Cargo.toml"), cargo_toml_content).await.context(format!("Failed to write Cargo.toml for {:?}\n", item_name))?;

            let lib_rs_content = format!(
                "#![feature(panic_internals)]\n#![feature(print_internals)]\n\nuse prelude::*\n\n{}",
                item_str
            );
            tokio::fs::write(decl_dir.join("src/lib.rs"), lib_rs_content).await.context(format!("Failed to write lib.rs for {:?}\n", item_name))?;

            declaration_count += 1;
        }
    }

    println!("Split {} declarations into separate crates.\n", declaration_count);

    if let Some(output_path) = args.output_declarations_toml.clone() {
        let mut types_map = HashMap::new();
        let mut modules_map = HashMap::new();
        let mut crates_map = HashMap::new();

        for (id, dep) in symbol_map.map.iter() {
            match dep.dependency_type.as_str() {
                "type" => { types_map.insert(id.clone(), dep.clone()); },
                "module" => { modules_map.insert(id.clone(), dep.clone()); },
                "crate" => { crates_map.insert(id.clone(), dep.clone()); },
                _ => {},
            }
        }

        let collected_info = CollectedProjectInfo {
            declarations: layered_decls.into_values().flatten().collect(), // Flatten all layers into a single Vec
            types: types_map,
            modules: modules_map,
            crates: crates_map,
        };

        let toml_string = toml::to_string_pretty(&collected_info)
            .context("Failed to serialize collected project info to TOML")?;
        tokio::fs::write(&output_path, toml_string)
            .context(format!("Failed to write collected project info to file: {:?}", output_path))?;
        println!("Successfully wrote collected project info to {:?}", output_path);
    }

    Ok(())
}
