use anyhow::{Context, Result};
use clap::Parser;
use prelude_generator::{category_pipeline, pipeline, use_extractor, Args, process_crates, collect_all_test_cases, generate_test_report_json, generate_test_verification_script_and_report, TestInfo};
use crate::category_pipeline::{ClassifyUsesFunctor, ExtractUsesFunctor, ParseFunctor, PipelineFunctor, RawFile, HuggingFaceValidatorFunctor, ValidatedFile};
use std::fs;
use prelude_generator::measurement;

fn run_category_pipeline(file_path: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let raw_file = RawFile(file_path.to_string(), content);

    println!("--- Stage 1: Parsing ---");
    let parse_functor = ParseFunctor;
    let parsed_file = parse_functor.map(raw_file)?;
    println!("  -> Parsed file successfully.");

    println!("--- Stage 2: Extracting Use Statements ---");
    let extract_uses_functor = ExtractUsesFunctor;
    let use_statements = extract_uses_functor.map(parsed_file)?;
    println!("  -> Extracted {} use statements.", use_statements.0.len());

    println!("--- Stage 3: Classifying Use Statements ---");
    let classify_uses_functor = ClassifyUsesFunctor;
    let classified_uses = classify_uses_functor.map(use_statements)?;
    println!("  -> Classified use statements:");
    println!("{:#?}", classified_uses);

    println!("--- Stage 4: Hugging Face Validation ---"); // New stage
    let hf_validator_functor = HuggingFaceValidatorFunctor;
    let validated_file = hf_validator_functor.map(parsed_file)?; // Use parsed_file as input
    println!("  -> Hugging Face Validation Result: {:#?}", validated_file);

    // Print collected metrics
    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).expect("Failed to serialize metrics to JSON");
    println!("--- METRICS_START ---\n{}\n--- METRICS_END ---", json_metrics);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(file_path) = &args.file {
        return run_category_pipeline(file_path);
    }

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