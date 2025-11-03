
use std::path::PathBuf;
use anyhow::Context;
use prelude_generator::use_extractor::RustcInfo;
use prelude_generator::gem_parser::GemConfig;
use prelude_generator::declaration_processing;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input directory to scan for Rust files.
    #[clap(value_parser = clap::value_parser!(PathBuf))]
    input_dir: PathBuf,
    /// Output directory to write generated declaration crates.
    #[clap(value_parser = clap::value_parser!(PathBuf), short, long)]
    output_dir: Option<PathBuf>,
}

use std::collections::HashMap;
use quote::quote;

// The original prelude_generator::Args is not used directly here.
// use prelude_generator::Args;
// use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = Args::parse();

    let input_dir = cli_args.input_dir;
    let output_dir = cli_args.output_dir.unwrap_or_else(|| PathBuf::from("./generated_declarations"));


    if output_dir.exists() {
        tokio::fs::remove_dir_all(&output_dir).await.context("Failed to remove existing output directory")?;
    }
    tokio::fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;

    println!("rust-decl-splitter is being refactored to use prelude-generator.");
    println!("Input directory: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

    // Create dummy RustcInfo and GemConfig for now
    let rustc_info = RustcInfo { version: "unknown".to_string(), host: "unknown".to_string() };
    let gem_config = GemConfig { gem: Vec::new() }; // Empty GemConfig

    // Call prelude-generator's extraction function
    let (all_declarations, _, _, _, _, _, _, _, collected_errors) = 
        declaration_processing::extract_all_declarations_from_crate(
            &input_dir, // Assuming input_dir is the manifest_path for now
            &prelude_generator::Args::parse_from(&["prelude-generator"]), // Pass a dummy prelude_generator::Args
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

            tokio::fs::create_dir_all(&decl_dir).await.context(format!("Failed to create directory {:?}", decl_dir))?;
            tokio::fs::create_dir_all(decl_dir.join("src")).await.context(format!("Failed to create directory {:?}/src", decl_dir))?;

            let cargo_toml_content = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
                item_name
            );
            tokio::fs::write(decl_dir.join("Cargo.toml"), cargo_toml_content).await.context(format!("Failed to write Cargo.toml for {:?}", item_name))?;

            let lib_rs_content = format!(
                "#![feature(panic_internals)]\n#![feature(print_internals)]\n\nuse prelude::*;

{}",
                item_str
            );
            tokio::fs::write(decl_dir.join("src/lib.rs"), lib_rs_content).await.context(format!("Failed to write lib.rs for {:?}", item_name))?;

            declaration_count += 1;
        }
    }

    println!("Split {} declarations into separate crates.", declaration_count);

    Ok(())
}
