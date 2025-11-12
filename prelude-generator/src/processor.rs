use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashSet;
use anyhow::Context;
use std::time::{Instant, Duration};
use std::fs; // Use std::fs for synchronous operations

use crate::args::Args;
use crate::report::generate_report;
use crate::types::FileProcessingResult; // Corrected import for FileProcessingResult

pub fn process_crates(args: &Args) -> Result<()> {
    println!("--- Prelude Generator Started ---");
    println!("Parsed arguments: {:?}", args);

    let _start_time = Instant::now();
    let _timeout_duration = args.timeout.map(Duration::from_secs);

    if args.cache_report {
        let cache_dir = PathBuf::from(&args.path).join(".prelude_cache");
        if cache_dir.exists() {
            let count = fs::read_dir(&cache_dir)?.count();
            println!("Prelude cache at {} contains {} items.", cache_dir.display(), count);
        } else {
            println!("Prelude cache directory not found at {}.", cache_dir.display());
        }
        return Ok(());
    }

    let all_file_processing_results: Vec<FileProcessingResult> = Vec::new();

    if args.report {
        if let Some(results_file_path) = &args.results_file {
            if results_file_path.exists() {
                let json_content = fs::read_to_string(results_file_path)
                    .context("Failed to read results file")?;
                let results: Vec<FileProcessingResult> = serde_json::from_str(&json_content)
                    .context("Failed to deserialize results from JSON")?;
                generate_report(&results)?;
            } else {
                eprintln!("Error: Results file not found at {}. Cannot generate report.", results_file_path.display());
            }
        }
    } else {
        // Perform prelude generation and save results
        let mut excluded_crates: HashSet<String> = args.exclude_crates.clone().into_iter().collect();
        // Always exclude prelude-generator and rust-decl-splitter from processing itself
        excluded_crates.insert("prelude-generator".to_string());
        excluded_crates.insert("rust-decl-splitter".to_string());
        // Add dependency-analyzer to excluded crates
        excluded_crates.insert("dependency-analyzer".to_string());
        // Add prelude-collector to excluded crates
        excluded_crates.insert("prelude-collector".to_string());
        println!("Excluded crates: {:?}", excluded_crates);

        // println!("Calling collect_prelude_info with path: {} and excluded crates: {:?}", PathBuf::from(&args.path).display(), excluded_crates);
        // let collected_info_vec = collect_prelude_info(PathBuf::from(&args.path).as_path(), &excluded_crates)?;
        // println!("Finished collect_prelude_info. Collected {} crates.", collected_info_vec.len());

        // for info in collected_info_vec {
        //     if let Some(duration) = timeout_duration {
        //         if start_time.elapsed() > duration {
        //             println!("Timeout of {} seconds reached. Stopping prelude generation.", duration.as_secs());
        //             break;
        //         }
        //     }

        //     println!(
        //         "\nProcessing collected info for crate: {} ({})",
        //         info.crate_name,
        //         info.crate_root.display()
        //     );
        //     println!("  Prelude content for this crate:\n---\n{}\n---", info.prelude_content);
        //     println!("  Files to be modified in this crate: {:?}", info.modified_files);

        //     all_file_processing_results.extend(info.file_processing_results);

        //     let src_dir = info.crate_root.join("src");

        //     // Generate the prelude file
        //     generate_prelude(&src_dir, &info.prelude_content, args.dry_run, args.force)?;

        //     // Modify files to use the prelude
        //     for path in &info.modified_files {
        //         modify_file(path, args.dry_run, args.force)?;
        //     }
            
        //     // Modify crate root to include the prelude
        //     if info.crate_root_modified {
        //         modify_crate_root(&src_dir, args.dry_run, args.force)?;
        //     }
        // }

        println!("\nPrelude generation complete.");
        println!("  -> Contents of all_file_processing_results: {:?}", all_file_processing_results);

        // Save results to file
        if let Some(results_file_path) = &args.results_file {
            let json_content = serde_json::to_string_pretty(&all_file_processing_results)
                .context("Failed to serialize results to JSON")?;
            println!("  -> Attempting to save processing results to: {}", results_file_path.display());
            fs::write(results_file_path, json_content)
                .context("Failed to write results to file")?;
            println!("Processing results saved to {}.", results_file_path.display());
        } else {
            println!("No results file specified. Skipping saving processing results.");
        }
    }
    Ok(())
}
