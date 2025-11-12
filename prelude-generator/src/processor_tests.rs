#![cfg(test)]

use super::*;
use tempfile::tempdir;
use std::path::{Path, PathBuf};
// Added Path import
// Removed use std::io::Write;
use prelude_collector::FileProcessingStatus;

fn setup_test_crate(dir: &Path, crate_name: &str, lib_content: &str) -> PathBuf {
    let crate_path = dir.join(crate_name);
    fs::create_dir_all(&crate_path.join("src")).unwrap();
    fs::write(crate_path.join("Cargo.toml"), format!("[package]\nname = \"{{}}\nversion = \"0.1.0\"
edition = \"2021\"
", crate_name)).unwrap();
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
        results_file: Some(project_root.join("results.json")),
        cache_report: false,
        timeout: None,
        force: true,
        ..Default::default()
    };

    process_crates(&args)?;

    // Verify prelude.rs is created
    let prelude_path = crate1_path.join("src/prelude.rs");
    assert!(prelude_path.exists());
    assert!(fs::read_to_string(&prelude_path)?.contains("// This is a generated prelude file"));

    // Verify lib.rs is modified
    let lib_rs_path = crate1_path.join("src/lib.rs");
    let lib_rs_content = fs::read_to_string(&lib_rs_path)?;
    assert!(lib_rs_content.contains("use crate::prelude::*"));
    assert!(!lib_rs_content.contains("use std::collections::HashMap;"));

    // Verify results.json is created and contains expected data
    let results_file_content = fs::read_to_string(&args.results_file.unwrap())?;
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
        results_file: Some(results_json_path.clone()),
        cache_report: false,
        timeout: None,
        force: false,
        ..Default::default()
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
