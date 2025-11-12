use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Item};
use walkdir::WalkDir;
use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct TestInfo {
    pub name: String,
    pub file_path: PathBuf,
}

/// Recursively extracts test functions from a list of AST items.
fn extract_test_functions_from_items(items: Vec<Item>, file_path: &Path) -> Vec<TestInfo> {
    let mut test_functions = Vec::new();
    for item in items {
        match item {
            Item::Fn(func) => {
                if func.attrs.iter().any(|attr| attr.path().is_ident("test")) {
                    test_functions.push(TestInfo {
                        name: func.sig.ident.to_string(),
                        file_path: file_path.to_path_buf(),
                    });
                }
            }
            Item::Mod(module) => {
                // Recursively check items within the module
                if let Some((_, module_items)) = module.content {
                    test_functions.extend(extract_test_functions_from_items(module_items, file_path));
                }
            }
            _ => {}
        }
    }
    test_functions
}

/// Extracts test functions from a single Rust file.
fn extract_test_cases_from_file(file_path: &Path) -> Result<Vec<TestInfo>> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
    let ast = syn::parse_file(&content)
        .with_context(|| format!("Failed to parse Rust file: {}", file_path.display()))?;

    Ok(extract_test_functions_from_items(ast.items, file_path))
}

/// Collects all unique test functions from the repository.
pub fn collect_all_test_cases(repo_root: &Path) -> Result<Vec<TestInfo>> {
    let mut all_test_functions = Vec::new();
    let mut seen_test_names = HashSet::new();

    for entry in WalkDir::new(repo_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let file_path = entry.path();
        println!("  -> Processing file for tests: {}", file_path.display());
        match extract_test_cases_from_file(file_path) {
            Ok(functions) => {
                for func_info in functions {
                    // Ensure uniqueness by function name
                    if seen_test_names.insert(func_info.name.clone()) {
                        all_test_functions.push(func_info);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not extract tests from {}: {}", file_path.display(), e);
            }
        }
    }
    Ok(all_test_functions)
}

/// Generates a JSON report of all collected test cases.
pub fn generate_test_report_json(output_path: &Path, test_functions: Vec<TestInfo>) -> Result<()> {
    let json_content = serde_json::to_string_pretty(&test_functions)
        .context("Failed to serialize test info to JSON")?;

    fs::write(output_path, json_content)
        .with_context(|| format!("Failed to write aggregated test report to {}", output_path.display()))?;

    println!("Aggregated test report generated at: {}", output_path.display());
    Ok(())
}

/// Generates a test verification script and a Markdown report.
pub fn generate_test_verification_script_and_report(output_dir: &Path, test_infos: Vec<TestInfo>) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory {}", output_dir.display()))?;

    let script_path = output_dir.join("run_all_tests.sh");
    let report_path = output_dir.join("Test_Verification_Report.md");

    // --- Generate run_all_tests.sh ---
    let mut script_content = String::new();
    script_content.push_str("#!/bin/bash\n\n");
    script_content.push_str("set -e\n\n"); // Exit immediately if a command exits with a non-zero status.

    let mut unique_crate_paths = HashSet::new();
    let repo_root = PathBuf::from("."); // Assuming current directory is the repo root for this script

    for entry in WalkDir::new(&repo_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name() == "Cargo.toml")
    {
        let cargo_toml_path = entry.path();
        if let Some(parent) = cargo_toml_path.parent() {
            // Exclude Cargo.toml files within target directories
            if !parent.components().any(|c| c.as_os_str() == "target") {
                unique_crate_paths.insert(parent.to_path_buf());
            }
        }
    }

    let mut sorted_crate_paths: Vec<&PathBuf> = unique_crate_paths.iter().collect();
    sorted_crate_paths.sort_by(|a, b| a.cmp(b));

    for crate_path in sorted_crate_paths {
        script_content.push_str(&format!("echo \"Running tests in {}...\"\n", crate_path.display()));
        script_content.push_str(&format!("pushd \"{}\"\n", crate_path.display()));
        script_content.push_str("cargo test\n");
        script_content.push_str("popd\n\n");
    }

    fs::write(&script_path, script_content)
        .with_context(|| format!("Failed to write run_all_tests.sh to {}", script_path.display()))?;

    // Make the script executable
    let mut perms = fs::metadata(&script_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)?;

    println!("Test verification script generated at: {}", script_path.display());

    // --- Generate Test_Verification_Report.md ---
    let mut report_content = String::new();
    report_content.push_str("# Test Verification Report\n\n");
    report_content.push_str("This report summarizes the tests found and provides a script to run them.\n\n");
    report_content.push_str(&format!("Total unique test functions found: {}\n\n", test_infos.len()));

    report_content.push_str("## How to Run Tests\n");
    report_content.push_str(&format!("To run all tests, execute the generated script:\n\n```bash\n./{}\n```\n\n", script_path.file_name().unwrap().to_str().unwrap()));
    report_content.push_str("The script will navigate to each identified Rust crate and run `cargo test` within it.\n\n");

    report_content.push_str("## Tests by Crate\n");
    let mut tests_by_crate: HashMap<PathBuf, Vec<String>> = HashMap::new();
    for info in test_infos {
        let mut current_path = info.file_path.as_path();
        while let Some(parent) = current_path.parent() {
            if parent.join("Cargo.toml").exists() {
                tests_by_crate.entry(parent.to_path_buf()).or_default().push(info.name);
                break;
            }
            current_path = parent;
        }
    }

    let mut sorted_crates: Vec<&PathBuf> = tests_by_crate.keys().collect();
    sorted_crates.sort_by(|a, b| a.cmp(b));

    for crate_path in sorted_crates {
        report_content.push_str(&format!("### Crate: {}\n", crate_path.display()));
        let mut sorted_test_names = tests_by_crate.get(crate_path).unwrap().clone();
        sorted_test_names.sort();
        for test_name in sorted_test_names {
            report_content.push_str(&format!("- {}\n", test_name));
        }
        report_content.push_str("\n");
    }

    fs::write(&report_path, report_content)
        .with_context(|| format!("Failed to write Test_Verification_Report.md to {}", report_path.display()))?;

    println!("Test verification report generated at: {}", report_path.display());
    Ok(())
}
