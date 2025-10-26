//Context
use anyhow::{ Result};
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use syn::Item;
use prelude_collector::collect_prelude_info; // Import the function
//use prelude_collector::CollectedPreludeInfo; // Import the struct

/// Command-line arguments for the prelude generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long, default_value_t = true)]
    dry_run: bool,
    /// The path to the workspace root.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Comma-separated list of crate names to exclude from processing.
    #[arg(long, value_delimiter = ',')]
    exclude_crates: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut excluded_crates: HashSet<String> = args.exclude_crates.into_iter().collect();
    // Always exclude prelude-generator and rust-decl-splitter from processing itself
    excluded_crates.insert("prelude-generator".to_string());
    excluded_crates.insert("rust-decl-splitter".to_string());
    // Add dependency-analyzer to excluded crates
    excluded_crates.insert("dependency-analyzer".to_string());
    // Add prelude-collector to excluded crates
    excluded_crates.insert("prelude-collector".to_string());


    let collected_info = collect_prelude_info(&args.path, &excluded_crates)?; // Use the collector

    for info in collected_info {
        println!(
            "\nProcessing collected info for crate: {} ({})",
            info.crate_name,
            info.crate_root.display()
        );

        let src_dir = info.crate_root.join("src");

        // Generate the prelude file
        generate_prelude(&src_dir, &info.prelude_content, args.dry_run)?;

        // Modify files to use the prelude
        for path in &info.modified_files {
            modify_file(path, args.dry_run)?;
        }
        
        // Modify crate root to include the prelude
        if info.crate_root_modified {
            modify_crate_root(&src_dir, args.dry_run)?;
        }
    }

    println!("\nPrelude generation complete.");
    Ok(())
}

/// Generates the `prelude.rs` file for a crate.
fn generate_prelude(
    src_dir: &Path,
    prelude_content: &str,
    dry_run: bool,
) -> Result<()> {
    let prelude_path = src_dir.join("prelude.rs");

    if dry_run {
        println!(
            "[DRY RUN] Would generate prelude file: {}\n---\n{}---",
            prelude_path.display(),
            prelude_content
        );
    } else {
        println!("  -> Generating prelude file: {}", prelude_path.display());
        fs::write(&prelude_path, prelude_content)?;
    }
    Ok(())
}

/// Modifies a source file to remove its `use` statements and add `use crate::prelude::*;`.
fn modify_file(path: &Path, dry_run: bool) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let ast = syn::parse_file(&content)?;
    let mut new_items = Vec::new();
    let mut has_use_statements = false;

    for item in &ast.items {
        if let Item::Use(_) = item {
            has_use_statements = true;
        } else {
            new_items.push(item.clone());
        }
    }

    if has_use_statements {
        let prelude_use: Item = syn::parse_quote! {
            use crate::prelude::*;
        };
        new_items.insert(0, prelude_use);

        let mut new_ast = ast.clone();
        new_ast.items = new_items;
        let new_content = prettyplease::unparse(&new_ast);

        if dry_run {
            println!(
                "[DRY RUN] Would modify file: {}\n---\n{}---",
                path.display(),
                new_content
            );
        } else {
            println!("  -> Modifying file: {}", path.display());
            fs::write(path, new_content)?;
        }
    }
    Ok(())
}

/// Modifies the crate root (`lib.rs` or `main.rs`) to ensure it contains `pub mod prelude;`.
fn modify_crate_root(src_dir: &Path, dry_run: bool) -> Result<()> {
    let lib_rs = src_dir.join("lib.rs");
    let main_rs = src_dir.join("main.rs");

    let crate_root_path = if lib_rs.exists() {
        lib_rs
    } else if main_rs.exists() {
        main_rs
    } else {
        return Ok(());
    };

    let content = fs::read_to_string(&crate_root_path)?;
    let ast = syn::parse_file(&content)?;
    let mut has_prelude_mod = false;

    for item in &ast.items {
        if let Item::Mod(mod_item) = item {
            if mod_item.ident == "prelude" {
                has_prelude_mod = true;
                break;
            }
        }
    }

    if !has_prelude_mod {
        let mut new_ast = ast.clone();
        let prelude_mod: Item = syn::parse_quote! {
            pub mod prelude;
        };
        new_ast.items.insert(0, prelude_mod);
        let new_content = prettyplease::unparse(&new_ast);

        if dry_run {
            println!(
                "[DRY RUN] Would add 'pub mod prelude;' to: {}",
                crate_root_path.display()
            );
        } else {
            println!("  -> Adding 'pub mod prelude;' to: {}", crate_root_path.display());
            fs::write(&crate_root_path, new_content)?;
        }
    }
    Ok(())
}
