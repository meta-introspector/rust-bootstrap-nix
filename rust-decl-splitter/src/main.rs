use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_directory> <output_directory>", args[0]);
        return Ok(());
    }
    let input_dir = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);

    if !input_dir.is_dir() {
        eprintln!("Error: Input directory does not exist or is not a directory.");
        return Ok(());
    }

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).context("Failed to remove existing output directory")?;
    }
    fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    println!("rust-decl-splitter is being refactored to use prelude-generator.");
    println!("Input directory: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

use prelude_generator::declaration::DeclarationItem;
use std::collections::HashMap;
use quote::quote;

use prelude_generator::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = Args::parse();

    let input_dir = cli_args.manifest_path.clone();
    let output_dir = cli_args.output_dir.clone().unwrap_or_else(|| PathBuf::from("./generated_declarations"));

    if !input_dir.is_dir() {
        eprintln!("Error: Input directory does not exist or is not a directory.");
        return Ok(());
    }

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).context("Failed to remove existing output directory")?;
    }
    fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    println!("rust-decl-splitter is being refactored to use prelude-generator.");
    println!("Input directory: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

    // Create dummy RustcInfo and GemConfig for now
    let rustc_info = RustcInfo { version: "unknown".to_string() };
    let gem_config = GemConfig::new(HashMap::new()); // Empty GemConfig

    // Call prelude-generator's extraction function
    let (all_declarations, _, _, _, _, _, _, _, collected_errors) = 
        declaration_processing::extract_all_declarations_from_crate(
            &input_dir, // Assuming input_dir is the manifest_path for now
            &cli_args, // Use the parsed Args
            &HashMap::new(), // Empty type_map
            &None, // No filter_names
            &rustc_info,
            &PathBuf::from("./cache"), // Dummy cache_dir
            &gem_config,
        ).await?;

    if !collected_errors.is_empty() {
        eprintln!("Errors collected during declaration extraction:");
        for error in collected_errors {
            eprintln!("  {:?}", error);
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
            };
            let item_name = decl.get_identifier();

            let level_dir = output_dir.join(layer_num.to_string());
            let decl_dir = level_dir.join(&item_name);

            fs::create_dir_all(&decl_dir).context(format!("Failed to create directory {:?}", decl_dir))?;
            fs::create_dir_all(decl_dir.join("src")).context(format!("Failed to create directory {:?}/src", decl_dir))?;

            let cargo_toml_content = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
                item_name
            );
            fs::write(decl_dir.join("Cargo.toml"), cargo_toml_content).context(format!("Failed to write Cargo.toml for {:?}", item_name))?;

            let lib_rs_content = format!(
                "#![feature(panic_internals)]\n#![feature(print_internals)]\n\nuse prelude::*;

{}",
                item_str
            );
            fs::write(decl_dir.join("src/lib.rs"), lib_rs_content).context(format!("Failed to write lib.rs for {:?}", item_name))?;

            declaration_count += 1;
        }
    }

    println!("Split {} declarations into separate crates.", declaration_count);

    Ok(())
}
