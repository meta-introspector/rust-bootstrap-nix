use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::{Instant, Duration};
use prelude_collector::{collect_prelude_info, FileProcessingResult};
use serde_json;

use crate::args::Args;
use crate::report::generate_report;
use crate::generate_prelude::generate_prelude;
use crate::modify_file::modify_file;
use crate::modify_crate_root::modify_crate_root;

pub fn process_crates(args: &Args) -> Result<()> {
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
        let mut excluded_crates: HashSet<String> = args.exclude_crates.clone().into_iter().collect();
        // Always exclude prelude-generator and rust-decl-splitter from processing itself
        excluded_crates.insert("prelude-generator".to_string());
        excluded_crates.insert("rust-decl-splitter".to_string());
        // Add dependency-analyzer to excluded crates
        excluded_crates.insert("dependency-analyzer".to_string());
        // Add prelude-collector to excluded crates
        excluded_crates.insert("prelude-collector".to_string());
        println!("Excluded crates: {:?}", excluded_crates);

        println!("Calling collect_prelude_info with path: {} and excluded crates: {:?}", args.path.display(), excluded_crates);
        let collected_info_vec = collect_prelude_info(&args.path, &excluded_crates)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::path::PathBuf;
    use prelude_collector::FileProcessingStatus;

    fn setup_test_crate(dir: &Path, crate_name: &str, lib_content: &str) -> PathBuf {
        let crate_path = dir.join(crate_name);
        fs::create_dir_all(&crate_path.join("src")).unwrap();
        fs::write(crate_path.join("Cargo.toml"), format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", crate_name)).unwrap();
        fs::write(crate_path.join("src/lib.rs"), lib_content).unwrap();
        crate_path
    }

    #[test]
    fn test_process_crates_integration() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().to_path_buf();

        // Setup a dummy crate
        let crate1_path = setup_test_crate(&project_root, "my-crate",
            "use std::collections::HashMap;\nfn my_func() {}\n"
        );

        // Create mock Args
        let args = Args {
            dry_run: false,
            path: project_root.clone(),
            exclude_crates: vec![],
            report: false,
            results_file: project_root.join("results.json"),
            cache_report: false,
            timeout: None,
            force: true,
        };

        process_crates(&args)?;

        // Verify prelude.rs is created
        let prelude_path = crate1_path.join("src/prelude.rs");
        assert!(prelude_path.exists());
        assert!(fs::read_to_string(&prelude_path)?.contains("// This is a generated prelude file"));

        // Verify lib.rs is modified
        let lib_rs_path = crate1_path.join("src/lib.rs");
        let lib_rs_content = fs::read_to_string(&lib_rs_path)?;
        assert!(lib_rs_content.contains("use crate::prelude::*;"));
        assert!(!lib_rs_content.contains("use std::collections::HashMap;"));

        // Verify results.json is created and contains expected data
        let results_file_content = fs::read_to_string(&args.results_file)?;
        let results: Vec<FileProcessingResult> = serde_json::from_str(&results_file_content)?;
        assert_eq!(results.len(), 2); // lib.rs and prelude.rs
        assert!(results.iter().any(|r| r.path.ends_with("src/lib.rs") && matches!(r.status, FileProcessingStatus::Success)));
        assert!(results.iter().any(|r| r.path.ends_with("src/prelude.rs") && matches!(r.status, FileProcessingStatus::Success)));

        Ok(())
    }

    #[test]
    fn test_process_crates_report_only() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().to_path_buf();

        // Create a dummy results.json file
        let dummy_results = vec![
            FileProcessingResult {
                path: PathBuf::from("dummy/file.rs"),
                status: FileProcessingStatus::Success,
            },
        ];
        let results_json_path = project_root.join("dummy_results.json");
        fs::write(&results_json_path, serde_json::to_string_pretty(&dummy_results)?)?;

        let args = Args {
            dry_run: false,
            path: project_root.clone(),
            exclude_crates: vec![],
            report: true,
            results_file: results_json_path.clone(),
            cache_report: false,
            timeout: None,
            force: false,
        };

        process_crates(&args)?;

        // Verify report is generated
        let report_path = project_root.join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let report_content = fs::read_to_string(&report_path)?;
        assert!(report_content.contains("Prelude Generation Summary Report"));
        assert!(report_content.contains("dummy/file.rs"));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    use std::path::PathBuf;
    use prelude_collector::FileProcessingStatus;

    fn setup_test_crate(dir: &Path, crate_name: &str, lib_content: &str) -> PathBuf {
        let crate_path = dir.join(crate_name);
        fs::create_dir_all(&crate_path.join("src")).unwrap();
        fs::write(crate_path.join("Cargo.toml"), format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", crate_name)).unwrap();
        fs::write(crate_path.join("src/lib.rs"), lib_content).unwrap();
        crate_path
    }

    #[test]
    fn test_process_crates_integration() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().to_path_buf();

        // Setup a dummy crate
        let crate1_path = setup_test_crate(&project_root, "my-crate",
            "use std::collections::HashMap;\nfn my_func() {}\n"
        );

        // Create mock Args
        let args = Args {
            dry_run: false,
            path: project_root.clone(),
            exclude_crates: vec![],
            report: false,
            results_file: project_root.join("results.json"),
            cache_report: false,
            timeout: None,
            force: true,
        };

        process_crates(&args)?;

        // Verify prelude.rs is created
        let prelude_path = crate1_path.join("src/prelude.rs");
        assert!(prelude_path.exists());
        assert!(fs::read_to_string(&prelude_path)?.contains("// This is a generated prelude file"));

        // Verify lib.rs is modified
        let lib_rs_path = crate1_path.join("src/lib.rs");
        let lib_rs_content = fs::read_to_string(&lib_rs_path)?;
        assert!(lib_rs_content.contains("use crate::prelude::*;"));
        assert!(!lib_rs_content.contains("use std::collections::HashMap;"));

        // Verify results.json is created and contains expected data
        let results_file_content = fs::read_to_string(&args.results_file)?;
        let results: Vec<FileProcessingResult> = serde_json::from_str(&results_file_content)?;
        assert_eq!(results.len(), 2); // lib.rs and prelude.rs
        assert!(results.iter().any(|r| r.path.ends_with("src/lib.rs") && matches!(r.status, FileProcessingStatus::Success)));
        assert!(results.iter().any(|r| r.path.ends_with("src/prelude.rs") && matches!(r.status, FileProcessingStatus::Success)));

        Ok(())
    }

    #[test]
    fn test_process_crates_report_only() -> Result<()> {
        let temp_dir = tempdir()?;
        let project_root = temp_dir.path().to_path_buf();

        // Create a dummy results.json file
        let dummy_results = vec![
            FileProcessingResult {
                path: PathBuf::from("dummy/file.rs"),
                status: FileProcessingStatus::Success,
            },
        ];
        let results_json_path = project_root.join("dummy_results.json");
        fs::write(&results_json_path, serde_json::to_string_pretty(&dummy_results)?)?;

        let args = Args {
            dry_run: false,
            path: project_root.clone(),
            exclude_crates: vec![],
            report: true,
            results_file: results_json_path.clone(),
            cache_report: false,
            timeout: None,
            force: false,
        };

        process_crates(&args)?;

        // Verify report is generated
        let report_path = project_root.join("prelude_generator_summary.md");
        assert!(report_path.exists());
        let report_content = fs::read_to_string(&report_path)?;
        assert!(report_content.contains("Prelude Generation Summary Report"));
        assert!(report_content.contains("dummy/file.rs"));

        Ok(())
    }
}
