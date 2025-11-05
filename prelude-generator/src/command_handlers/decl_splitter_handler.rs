use std::path::PathBuf;
use anyhow::Context;


use split_expanded_lib::{Declaration, DeclarationItem, RustcInfo};
use crate::Args; // Use prelude_generator's Args

use std::collections::HashMap;
use quote::ToTokens;
use toml;
use crate::types::CollectedProjectInfo;
use crate::declaration_processing::extract_declarations::extract_all_declarations_from_file;
use crate::declaration_processing::layer_declarations::layer_declarations;


pub async fn handle_run_decl_splitter(args: &Args) -> anyhow::Result<()> {
    println!("Running declaration splitter functionality...");

    let input_path = args.path.clone(); // This will now be the path to the expanded .rs file

    let output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| PathBuf::from("./generated_declarations"));

    if output_dir.exists() {
        tokio::fs::remove_dir_all(&output_dir).await.context("Failed to remove existing output directory")?;
    }
    tokio::fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;





    let mut all_declarations_aggregated: Vec<Declaration> = Vec::new();
    let all_collected_errors_aggregated: Vec<crate::ErrorSample> = Vec::new();
    let mut symbol_map_aggregated = crate::symbol_map::SymbolMap::new();

    let (declarations, symbol_map, _errors, _file_metadata) = extract_all_declarations_from_file(&input_path, &output_dir, args.dry_run, args.verbose, &RustcInfo { version: args.split_expanded_rustc_version.clone().unwrap_or_else(|| "1.89.0".to_string()), host: args.split_expanded_rustc_host.clone().unwrap_or_else(|| "aarch64-unknown-linux-gnu".to_string()) }, &input_path.file_stem().unwrap().to_string_lossy()).await?;
    all_declarations_aggregated.extend(declarations);

    symbol_map_aggregated.map.extend(symbol_map.map);
            
            if !all_collected_errors_aggregated.is_empty() {
                eprintln!("Errors collected during declaration extraction:");
                for error in all_collected_errors_aggregated {
                    eprintln!("  {:?}\n", error);
                }
            }
            
            // Layer the declarations
            let layered_decls = layer_declarations(all_declarations_aggregated);
            
            let mut declaration_count = 0;
            
            for (layer_num, declarations_in_layer) in &layered_decls {        println!("Processing layer {}", layer_num);
        for decl in declarations_in_layer {
            let item_str = match &decl.item {
                DeclarationItem::Const(item) => item.to_token_stream().to_string(),
                DeclarationItem::Struct(item) => item.to_token_stream().to_string(),
                DeclarationItem::Enum(item) => item.to_token_stream().to_string(),
                DeclarationItem::Fn(item) => item.to_token_stream().to_string(),
                DeclarationItem::Static(item) => item.to_token_stream().to_string(),
                DeclarationItem::Macro(item) => item.to_token_stream().to_string(),
                DeclarationItem::Mod(item) => item.to_token_stream().to_string(),
                DeclarationItem::Trait(item) => item.to_token_stream().to_string(),
                DeclarationItem::TraitAlias(item) => item.to_token_stream().to_string(),
                DeclarationItem::Type(item) => item.to_token_stream().to_string(),
                DeclarationItem::Union(item) => item.to_token_stream().to_string(),
                DeclarationItem::Other(item) => item.to_token_stream().to_string(),
            };
            let item_name = decl.get_identifier();

            let level_dir = output_dir.join(layer_num.to_string());
            let decl_dir = level_dir.join(&item_name);

            if args.dry_run {
                println!("Dry run: Would create directory: {}", level_dir.display());
            } else {
                tokio::fs::create_dir_all(&level_dir).await.context(format!("Failed to create directory {:?}\n", level_dir))?;
            }
            if args.dry_run {
                println!("Dry run: Would create directory: {}", decl_dir.display());
            } else {
                tokio::fs::create_dir_all(&decl_dir).await.context(format!("Failed to create directory {:?}\n", decl_dir))?;
            }
            if args.dry_run {
                println!("Dry run: Would create directory: {}", decl_dir.join("src").display());
            } else {
                tokio::fs::create_dir_all(decl_dir.join("src")).await.context(format!("Failed to create directory {:?}/src\n", decl_dir))?;
            }
            let cargo_toml_content = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
                item_name
            );
            if args.dry_run {
                println!("Dry run: Would write Cargo.toml for {:?} to {:?}", item_name, decl_dir.join("Cargo.toml"));
            } else {
                tokio::fs::write(decl_dir.join("Cargo.toml"), cargo_toml_content).await.context(format!("Failed to write Cargo.toml for {:?}\n", item_name))?;
            }
            let lib_rs_content = format!(
                "#![feature(panic_internals)]\n#![feature(print_internals)]\n\nuse prelude::*\n\n{}",
                item_str
            );
            if args.dry_run {
                println!("Dry run: Would write lib.rs for {:?} to {:?}", item_name, decl_dir.join("src/lib.rs"));
            } else {
                tokio::fs::write(decl_dir.join("src/lib.rs"), lib_rs_content).await.context(format!("Failed to write lib.rs for {:?}\n", item_name))?;
            }
            declaration_count += 1;
        }
    }

    println!("Split {} declarations into separate crates.\n", declaration_count);

    if let Some(output_path) = args.output_declarations_toml.clone() {
        let mut types_map = HashMap::new();
        let mut modules_map = HashMap::new();
        let mut crates_map = HashMap::new();

        for (id, dep) in symbol_map_aggregated.map.iter() {
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
        tokio::fs::write(&output_path, toml_string).await
            .context(format!("Failed to write collected project info to file: {:?}", output_path))?;
        println!("Successfully wrote collected project info to {:?}", output_path);
    }

    if let Some(output_path) = args.output_symbol_map.clone() {
        let toml_string = toml::to_string_pretty(&symbol_map_aggregated.map)
            .context("Failed to serialize symbol map to TOML")?;
        tokio::fs::write(&output_path, toml_string).await
            .context(format!("Failed to write symbol map to file: {:?}", output_path))?;
        println!("Successfully wrote symbol map to {:?}", output_path);
    }

    Ok(())
}
