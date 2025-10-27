use anyhow::{Context, Result};
use clap::Parser;
use prelude_generator::{Args, process_crates, collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};
use std::fs;
mod use_extractor;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.generate_test_report {
        println!("Collecting all unique test cases for JSON report...");
        let test_infos = collect_all_test_cases(&args.path)?;
        println!("Found {} unique test functions.", test_infos.len());

        let output_path = args.test_report_output_file.unwrap_or_else(|| {
            args.path.join("test_report.json")
        });

        generate_test_report_json(&output_path, test_infos)
            .context("Failed to generate JSON test report")?;
    } else if args.compile_tests {
        println!("Generating test verification script and report...");
        let input_file = args.test_report_input_file.as_ref().context("--test-report-input-file is required when --compile-tests is true")?;
        let output_dir = args.test_verification_output_dir.as_ref().context("--test-verification-output-dir is required when --compile-tests is true")?;

        let json_content = fs::read_to_string(input_file)
            .with_context(|| format!("Failed to read test report from {}", input_file.display()))?;
        let test_infos: Vec<TestInfo> = serde_json::from_str(&json_content)
            .context("Failed to deserialize test report from JSON")?;

        generate_test_verification_script_and_report(output_dir, test_infos)
            .context("Failed to generate test verification script and report")?;
    } else if args.extract_use_statements {
        println!("Extracting unique use statements and generating test files...");
        let use_statements = use_extractor::extract_unique_use_statements(&args.path)?;
        let output_dir = args.use_statements_output_dir.as_ref().context("--use-statements-output-dir is required when --extract-use-statements is true")?;

        use_extractor::generate_use_statement_test_files(output_dir, use_statements)
            .context("Failed to generate use statement test files")?;
    } else {
        process_crates(&args)?;
    }

    Ok(())
}