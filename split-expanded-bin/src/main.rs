use clap::Parser;
use anyhow::Context;
use std::path::PathBuf;
use split_expanded_lib::{extract_declarations_from_single_file, RustcInfo, DeclarationItem};

use quote::quote;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the input Rust file (e.g., an expanded .rs file).
    #[arg(long)]
    pub file: PathBuf,

    /// Path to the output directory where individual declaration files will be saved.
    #[arg(long)]
    pub output_dir: PathBuf,

    /// Rustc version (e.g., "1.89.0").
    #[arg(long)]
    pub rustc_version: String,

    /// Rustc host triple (e.g., "aarch64-unknown-linux-gnu").
    #[arg(long)]
    pub rustc_host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Create RustcInfo from command-line arguments
    let rustc_info = RustcInfo {
        version: args.rustc_version,
        host: args.rustc_host,
    };

    // Ensure output directory exists
    tokio::fs::create_dir_all(&args.output_dir).await
        .context(format!("Failed to create output directory: {}", args.output_dir.display()))?;

    println!("Processing file: {}", args.file.display());

    let file_stem = args.file.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown_crate");
    let crate_name = file_stem.trim_start_matches(".expand_output_");

    let (declarations, errors) = extract_declarations_from_single_file(
        &args.file,
        &rustc_info,
        crate_name,
    ).await?;

    if !errors.is_empty() {
        eprintln!("Errors encountered during parsing:");
        for error in errors {
            eprintln!("  File: {}", error.file_path.display());
            eprintln!("  Error Type: {}", error.error_type);
            eprintln!("  Message: {}", error.error_message);
            if let Some(snippet) = error.code_snippet {
                eprintln!("  Code Snippet:\n{}", snippet);
            }
        }
    }

    for decl in declarations {
        let decl_name = decl.get_identifier();
        let file_name = format!("{}.rs", decl_name);
        let output_path = args.output_dir.join(&file_name);

        let content = match decl.item {
            DeclarationItem::Const(item) => quote! { #item }.to_string(),
            DeclarationItem::Struct(item) => quote! { #item }.to_string(),
            DeclarationItem::Enum(item) => quote! { #item }.to_string(),
            DeclarationItem::Fn(item) => quote! { #item }.to_string(),
            DeclarationItem::Static(item) => quote! { #item }.to_string(),
            DeclarationItem::Macro(item) => quote! { #item }.to_string(),
            DeclarationItem::Mod(item) => quote! { #item }.to_string(),
            DeclarationItem::Trait(item) => quote! { #item }.to_string(),
            DeclarationItem::TraitAlias(item) => quote! { #item }.to_string(),
            DeclarationItem::Type(item) => quote! { #item }.to_string(),
            DeclarationItem::Union(item) => quote! { #item }.to_string(),
            DeclarationItem::Other(item) => quote! { #item }.to_string(),
        };

        tokio::fs::write(&output_path, content.as_bytes()).await
            .context(format!("Failed to write declaration {} to {}", decl_name, output_path.display()))?;
        println!("Saved declaration: {} to {}", decl_name, output_path.display());
    }

    Ok(())
}
