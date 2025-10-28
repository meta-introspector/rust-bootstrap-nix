use anyhow::{Context, Result};
use clap::Parser;
use prelude_generator::{pipeline, Args, process_crates, collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};
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
        println!("This feature has been removed.");
    } else if args.collect_and_process_use_statements {
        println!("Collecting and processing use statements...");
        use_extractor::collect_and_process_use_statements(
            &args.path,
            args.stop_after,
            args.step_timeout,
            args.verbose,
            args.dry_run,
        )?;
    } else if args.generate_aggregated_test_file {
        println!("Generating aggregated use statement test file...");
        use_extractor::generate_aggregated_use_test_file(&args.path)?;
    } else if args.run_pipeline {
        println!("Running use statement processing pipeline...");
        pipeline::run_pipeline(&args.stage, args.batch_size, args.batch_limit, args.verbose)?;
    } else {
        process_crates(&args)?;
    }

    Ok(())
}