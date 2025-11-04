use std::path::PathBuf;
use anyhow::Context;
use prelude_generator::use_extractor::RustcInfo;
use prelude_generator::gem_parser::GemConfig;
use prelude_generator::declaration_processing;
use prelude_generator::Args; // Use prelude_generator's Args

use std::collections::HashMap;
use quote::quote;

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
    let gem_config = GemConfig { gem: Vec::new() }; // Empty GemConfig

    // Call prelude-generator's extraction function
    let (all_declarations, _, _, _, _, _, _, _, collected_errors) = 
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
                prelude_generator::declaration::DeclarationItem::Const(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Struct(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Enum(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Fn(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Static(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Other(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Macro(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Mod(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Trait(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::TraitAlias(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Type(item) => quote! { #item }.to_string(),
                prelude_generator::declaration::DeclarationItem::Union(item) => quote! { #item }.to_string(),
            };
            let item_name = decl.get_identifier();

            let level_dir = output_dir.join(layer_num.to_string());
            let decl_dir = level_dir.join(&item_name);

            tokio::fs::create_dir_all(&decl_dir).await.context(format!("Failed to create directory {:?}\n", decl_dir))?;
            tokio::fs::create_dir_all(decl_dir.join("src")).await.context(format!("Failed to create directory {:?}/src\n", decl_dir))?;

            let cargo_toml_content = format!(
                "[package]\nname = \"{eyse}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
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

    Ok(())
}
