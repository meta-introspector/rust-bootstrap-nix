use super::args::Args;
use std::path::PathBuf;
use anyhow::Result;
use std::fs;
//use prelude_collector::{FileProcessingResult, FileProcessingStatus};
use tempfile::tempdir;
use super::generate_prelude;
use clap::Parser;
use crate::types::{FileProcessingResult, FileProcessingStatus};
pub fn test_args_default_values() {
    let args = Args::parse_from(&["prelude-generator"]);
    assert!(!args.dry_run);
    assert_eq!(args.path, PathBuf::from("."));
    assert!(args.exclude_crates.is_empty());
    assert!(!args.report);
    assert_eq!(args.results_file, Some(PathBuf::from("prelude_processing_results.json")));
    assert!(!args.cache_report);
    assert!(args.timeout.is_none());
    assert!(!args.force);
}

pub fn test_args_custom_values() {
    let args = Args::parse_from(&[
        "prelude-generator",
        "--dry-run",
        "--path", "/tmp/my_project",
        "--exclude-crates", "crate1,crate2",
        "--report",
        "--results-file", "custom_results.json",
        "--cache-report",
        "--timeout", "60",
        "--force",
    ]);

    assert!(args.dry_run);
    assert_eq!(args.path, PathBuf::from("/tmp/my_project"));
    assert_eq!(args.exclude_crates, vec!["crate1".to_string(), "crate2".to_string()]);
    assert!(args.report);
    assert_eq!(args.results_file, Some(PathBuf::from("custom_results.json")));
    assert!(args.cache_report);
    assert_eq!(args.timeout, Some(60));
    assert!(args.force);
}

pub fn test_generate_report_empty_results() -> Result<()> {
    let dir = tempdir()?;
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&dir)?;

    super::report::generate_report(&[])?;

    let report_path = dir.path().join("prelude_generator_summary.md");
    assert!(report_path.exists());
    let content = fs::read_to_string(&report_path)?;
    assert!(content.contains("# Prelude Generation Summary Report"));
    assert!(content.contains("- Total files processed: 0"));
    assert!(!content.contains("## Detailed Results"));

    std::env::set_current_dir(&original_dir)?;
    Ok(())
}

pub fn test_generate_report_with_results() -> Result<()> {
    let dir = tempdir()?;
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&dir)?;

    let results = vec![
        FileProcessingResult {
            path: PathBuf::from("src/file1.rs"),
            status: FileProcessingStatus::Success,
        },
        FileProcessingResult {
            path: PathBuf::from("src/file2.rs"),
            status: FileProcessingStatus::Skipped { reason: "already processed".to_string() },
        },
        FileProcessingResult {
            path: PathBuf::from("src/file3.rs"),
            status: FileProcessingStatus::Failed { error: "syntax error".to_string() },
        },
    ];

    super::report::generate_report(&results)?;

    let report_path = dir.path().join("prelude_generator_summary.md");
    assert!(report_path.exists());
    let content = fs::read_to_string(&report_path)?;

    assert!(content.contains("# Prelude Generation Summary Report"));
    assert!(content.contains("- Total files processed: 3"));
    assert!(content.contains("- Successfully processed: 1"));
    assert!(content.contains("- Skipped: 1"));
    assert!(content.contains("- Failed: 1"));
    assert!(content.contains("### src/file1.rs\n- Status: ✅ Successfully Processed"));
    assert!(content.contains("### src/file2.rs\n- Status: ⏭️ Skipped (Reason: already processed"));
    assert!(content.contains("### src/file3.rs\n- Status: ❌ Failed (Error: syntax error"));

    std::env::set_current_dir(&original_dir)?;
    Ok(())
}

pub fn test_generate_prelude_creates_file() -> Result<()> {
    let dir = tempdir()?;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir)?;
    let prelude_content = "// Test prelude content";

    generate_prelude::generate_prelude(&src_dir, prelude_content, false, false)?;

    let prelude_path = src_dir.join("prelude.rs");
    assert!(prelude_path.exists());
    assert_eq!(fs::read_to_string(&prelude_path)?, prelude_content);

    Ok(())
}

pub fn test_generate_prelude_dry_run() -> Result<()> {
    let dir = tempdir()?;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir)?;
    let prelude_content = "// Test prelude content";

    generate_prelude::generate_prelude(&src_dir, prelude_content, true, false)?;

    let prelude_path = src_dir.join("prelude.rs");
    assert!(!prelude_path.exists()); // File should not be created in dry run

    Ok(())
}

pub fn test_generate_prelude_force_overwrite() -> Result<()> {
    let dir = tempdir()?;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir)?;
    let prelude_path = src_dir.join("prelude.rs");
    fs::write(&prelude_path, "// Original content")?;

    let new_prelude_content = "// New prelude content";
    generate_prelude::generate_prelude(&src_dir, new_prelude_content, false, true)?;

    assert!(prelude_path.exists());
    assert_eq!(fs::read_to_string(&prelude_path)?, new_prelude_content);

    Ok(())
}

pub fn test_generate_prelude_no_force_no_overwrite() -> Result<()> {
    let dir = tempdir()?;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir)?;
    let prelude_path = src_dir.join("prelude.rs");
    let original_content = "// Original content";
    fs::write(&prelude_path, original_content)?;

    let new_prelude_content = "// New prelude content";
    generate_prelude::generate_prelude(&src_dir, new_prelude_content, false, false)?;

    assert!(prelude_path.exists());
    assert_eq!(fs::read_to_string(&prelude_path)?, original_content); // Content should not change

    Ok(())
}
