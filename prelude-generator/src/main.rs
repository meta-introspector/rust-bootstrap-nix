//Context
use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};
use syn::Item;
use prelude_collector::{collect_prelude_info, FileProcessingResult, FileProcessingStatus}; // Import the function and new types
use serde_json;

/// Command-line arguments for the prelude generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long)]
    dry_run: bool,
    /// The path to the workspace root.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Comma-separated list of crate names to exclude from processing.
    #[arg(long, value_delimiter = ',')]
    exclude_crates: Vec<String>,
    /// Generate a summary report of the prelude generation process.
    #[arg(long, default_value_t = false)]
    report: bool,
    /// Path to a file to save/load processing results.
    #[arg(long, default_value = "prelude_processing_results.json")]
    results_file: PathBuf,
    /// Generate a report on the prelude cache.
    #[arg(long, default_value_t = false)]
    cache_report: bool,
    /// Timeout in seconds for the prelude generation process.
    #[arg(long)]
    timeout: Option<u64>,
    /// Force overwriting of files even if they exist.
    #[arg(long, default_value_t = false)]
    force: bool,
}

/// Generates a markdown report of the file processing results.
fn generate_report(results: &[FileProcessingResult]) -> Result<()> {
    let mut report_content = String::new();
    report_content.push_str("# Prelude Generation Summary Report\n\n");
    report_content.push_str("This report summarizes the processing of Rust files during prelude generation.\n\n");

    let total_files = results.len();
    let successful_files = results.iter().filter(|r| matches!(r.status, FileProcessingStatus::Success)).count();
    let skipped_files = results.iter().filter(|r| matches!(r.status, FileProcessingStatus::Skipped { .. })).count();
    let failed_files = results.iter().filter(|r| matches!(r.status, FileProcessingStatus::Failed { .. })).count();

    report_content.push_str("## Summary\n");
    report_content.push_str(&format!("- Total files processed: {}\n", total_files));
    report_content.push_str(&format!("- Successfully processed: {}\n", successful_files));
    report_content.push_str(&format!("- Skipped: {}\n", skipped_files));
    report_content.push_str(&format!("- Failed: {}\n\n", failed_files));

    if !results.is_empty() {
        report_content.push_str("## Detailed Results\n");
        for result in results {
            report_content.push_str(&format!("### {}\n", result.path.display()));
            match &result.status {
                FileProcessingStatus::Success => {
                    report_content.push_str("- Status: ✅ Successfully Processed\n\n");
                }
                FileProcessingStatus::Skipped { reason } => {
                    report_content.push_str(&format!("- Status: ⏭️ Skipped (Reason: {}\n\n", reason));
                }
                FileProcessingStatus::Failed { error } => {
                    report_content.push_str(&format!("- Status: ❌ Failed (Error: {}\n\n", error));
                }
            }
        }
    }

    fs::write("prelude_generator_summary.md", report_content)?;
    println!("Report generated: prelude_generator_summary.md");
    Ok(())
}

/// Generates the `prelude.rs` file for a crate.
fn generate_prelude(
    src_dir: &Path,
    prelude_content: &str,
    dry_run: bool,
    force: bool,
) -> Result<()> {
    println!("  -> Entering generate_prelude for src_dir: {}", src_dir.display());
    let prelude_path = src_dir.join("prelude.rs");

    if dry_run {
        println!(
            "[DRY RUN] Would generate prelude file: {}\n---\n{}---",
            prelude_path.display(),
            prelude_content
        );
    } else {
        if prelude_path.exists() && !force {
            println!("  -> Skipping prelude file generation for {} (file exists, use --force to overwrite).", prelude_path.display());
        } else {
            println!("  -> Generating prelude file: {}", prelude_path.display());
            println!("    -> Writing prelude content to: {}", prelude_path.display());
            fs::write(&prelude_path, prelude_content)?;
        }
    }
    Ok(())
}

/// Modifies a source file to remove its `use` statements and add `use crate::prelude::*;`.
fn modify_file(path: &Path, dry_run: bool, force: bool) -> Result<()> {
    println!("  -> Entering modify_file for path: {}", path.display());
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
            if path.exists() && !force {
                println!("  -> Skipping file modification for {} (file exists, use --force to overwrite).", path.display());
            } else {
                println!("  -> Modifying file: {}", path.display());
                println!("    -> Writing modified content to: {}", path.display());
                fs::write(path, new_content)?;
            }
        }
    }
    Ok(())
}

/// Modifies the crate root (`lib.rs` or `main.rs`) to ensure it contains `pub mod prelude;`.
fn modify_crate_root(src_dir: &Path, dry_run: bool, force: bool) -> Result<()> {
    println!("  -> Entering modify_crate_root for src_dir: {}", src_dir.display());
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
            if crate_root_path.exists() && !force {
                println!("  -> Skipping crate root modification for {} (file exists, use --force to overwrite).", crate_root_path.display());
            } else {
                println!("  -> Adding 'pub mod prelude;' to: {}", crate_root_path.display());
                println!("    -> Writing modified content to: {}", crate_root_path.display());
                fs::write(&crate_root_path, new_content)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("--- Prelude Generator Started ---");
    println!("Parsed arguments: {:?}", args);

    let start_time = Instant::now();
    let timeout_duration = args.timeout.map(Duration::from_secs);

    if args.cache_report {
        let cache_dir = args.path.join(".prelude_cache");
        if cache_dir.exists() {
            let count = fs::read_dir(&cache_dir)?.count();
            println!("Prelude cache at {} contains {} items.", cache_dir.display(), count);
        } else {
            println!("Prelude cache directory not found at {}.", cache_dir.display());
        }
        return Ok(());
    }

    let mut all_file_processing_results: Vec<FileProcessingResult> = Vec::new();

    if args.report {
        // Load results from file and generate report
        if args.results_file.exists() {
            let json_content = fs::read_to_string(&args.results_file)
                .context("Failed to read results file")?;
            all_file_processing_results = serde_json::from_str(&json_content)
                .context("Failed to deserialize results from JSON")?;
            generate_report(&all_file_processing_results)?;
        } else {
            eprintln!("Error: Results file not found at {}. Cannot generate report.", args.results_file.display());
        }
    } else {
        // Perform prelude generation and save results
        let mut excluded_crates: HashSet<String> = args.exclude_crates.into_iter().collect();
        // Always exclude prelude-generator and rust-decl-splitter from processing itself
        excluded_crates.insert("prelude-generator".to_string());
        excluded_crates.insert("rust-decl-splitter".to_string());
        // Add dependency-analyzer to excluded crates
        excluded_crates.insert("dependency-analyzer".to_string());
        // Add prelude-collector to excluded crates
        excluded_crates.insert("prelude-collector".to_string());
        println!("Excluded crates: {:?}", excluded_crates);

        println!("Calling collect_prelude_info with path: {} and excluded crates: {:?}", args.path.display(), excluded_crates);
        let collected_info_vec = collect_prelude_info(&args.path, &excluded_crates)?; // Use the collector
        println!("Finished collect_prelude_info. Collected {} crates.", collected_info_vec.len());

        for info in collected_info_vec {
            if let Some(duration) = timeout_duration {
                if start_time.elapsed() > duration {
                    println!("Timeout of {} seconds reached. Stopping prelude generation.", duration.as_secs());
                    break;
                }
            }

            println!(
                "\nProcessing collected info for crate: {} ({})",
                info.crate_name,
                info.crate_root.display()
            );
            println!("  Prelude content for this crate:\n---\n{}\n---", info.prelude_content);
            println!("  Files to be modified in this crate: {:?}", info.modified_files);

            all_file_processing_results.extend(info.file_processing_results);

            let src_dir = info.crate_root.join("src");

            // Generate the prelude file
            generate_prelude(&src_dir, &info.prelude_content, args.dry_run, args.force)?;

            // Modify files to use the prelude
            for path in &info.modified_files {
                modify_file(path, args.dry_run, args.force)?;
            }
            
            // Modify crate root to include the prelude
            if info.crate_root_modified {
                modify_crate_root(&src_dir, args.dry_run, args.force)?;
            }
        }

        println!("\nPrelude generation complete.");
        println!("  -> Contents of all_file_processing_results: {:?}", all_file_processing_results);

        // Save results to file
        let json_content = serde_json::to_string_pretty(&all_file_processing_results)
            .context("Failed to serialize results to JSON")?;
        println!("  -> Attempting to save processing results to: {}", args.results_file.display());
        fs::write(&args.results_file, json_content)
            .context("Failed to write results to file")?;
        println!("Processing results saved to {}.", args.results_file.display());

        if args.report {
            generate_report(&all_file_processing_results)?;
        }
    }

    Ok(())
}
