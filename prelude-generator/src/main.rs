use pipeline_traits::PipelineFunctor;
use prelude_generator::parser::ParseFunctor;
use prelude_generator::args::Args;
use crate::config_parser::Config;
use prelude_generator::measurement;
use clap::Parser;
use anyhow::Context;
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use pipeline_traits::RawFile;
use prelude_generator::prelude_category_pipeline::{AstReconstructionFunctor, ExtractUsesFunctor, ClassifyUsesFunctor, HuggingFaceValidatorFunctor};

// mod hf_dataset_reader;
mod config_parser;
mod parser;

async fn run_category_pipeline<W: tokio::io::AsyncWriteExt + Unpin + Send>(
    writer: &mut W,
    file_content: &str,
    file_path_str: &str,
    args: &Args,
    config: &Option<config_parser::Config>,
) -> anyhow::Result<()> {
    let raw_file = RawFile(file_path_str.to_string(), file_content.to_string());

    writer.write_all(format!("--- Stage 1: Parsing ---\n").as_bytes()).await?;
    let parse_functor = ParseFunctor;
    let parsed_file = parse_functor.map(writer, raw_file).await.context("Parsing failed")?;
    writer.write_all(format!("  -> Parsed file successfully.\n").as_bytes()).await?;

    writer.write_all(format!("--- Stage 2: Extracting Use Statements ---\n").as_bytes()).await?;
    let extract_uses_functor = ExtractUsesFunctor;
    let use_statements = extract_uses_functor.map(writer, parsed_file.clone()).await.context("Extracting use statements failed")?;
    writer.write_all(format!("  -> Extracted {} use statements.\n", use_statements.0.len()).as_bytes()).await?;

    writer.write_all(format!("--- Stage 3: Classifying Use Statements ---\n").as_bytes()).await?;
    let classify_uses_functor = ClassifyUsesFunctor;
    let classified_uses = classify_uses_functor.map(writer, use_statements).await.context("Classifying use statements failed")?;
    writer.write_all(format!("  -> Classified use statements:\n").as_bytes()).await?;
    writer.write_all(format!("{:#?}\n", classified_uses).as_bytes()).await?;

    writer.write_all(format!("--- Stage 4: Hugging Face Validation ---
").as_bytes()).await?;
    let hf_validator_path = config.as_ref().and_then(|c| {
        c.bins.paths.get("hf_validator").map(|p| p.to_path_buf())
    });
    let hf_validator_functor = HuggingFaceValidatorFunctor { args: args.clone(), hf_validator_path };
    let validated_file = hf_validator_functor.map(writer, parsed_file.clone()).await.context("Hugging Face Validation failed")?; // Use parsed_file as input
    writer.write_all(format!("  -> Hugging Face Validation Result: {:#?}\n", validated_file).as_bytes()).await?;

    writer.write_all(format!("--- Stage 5: AST Reconstruction from Hugging Face Dataset ---\n").as_bytes()).await?;
    let ast_reconstruction_functor = AstReconstructionFunctor;
    let reconstructed_code = PipelineFunctor::map(&ast_reconstruction_functor, writer, validated_file.clone()).await.context("AST Reconstruction failed")?;
    writer.write_all(format!("  -> AST Reconstruction completed successfully.\n").as_bytes()).await?;

    // Write generated code to a file
    let output_file_path = PathBuf::from("generated/self_generated_code.rs"); // Define output path
    tokio::fs::create_dir_all(output_file_path.parent().unwrap()).await?;
    tokio::fs::write(&output_file_path, reconstructed_code.as_bytes()).await
        .context(format!("Failed to write generated code to {:?}", output_file_path))?;
    writer.write_all(format!("  -> Generated code written to {:?}\n", output_file_path).as_bytes()).await?;

    // Validate the generated code
    writer.write_all(format!("--- Validating Generated Code ---\n").as_bytes()).await?;
    validate_rust_code(&output_file_path).await
        .context(format!("Generated code validation failed for {:?}", output_file_path))?;
    writer.write_all(format!("  -> Generated code validated successfully.\n").as_bytes()).await?;

    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).context("Failed to serialize metrics to JSON")?;
    writer.write_all(format!("--- METRICS_START ---\n{}\n--- METRICS_END ---\n", json_metrics).as_bytes()).await?;

    Ok(())
}

async fn validate_rust_code(file_path: &PathBuf) -> anyhow::Result<()> {
    use tokio::process::Command;

    let output = Command::new("rustc")
        .arg("--emit=metadata") // Only check for errors, don't produce artifacts
        .arg("--crate-type=lib") // Treat as a library crate
        .arg(file_path)
        .output()
        .await
        .context("Failed to execute rustc")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rustc check failed for file {:?}:\n{}", file_path, stderr);
    }

    Ok(())
}

fn parse_arguments_and_config() -> anyhow::Result<(Args, Option<Config>)> {

    let args = Args::parse();

    // Determine the project root. If args.path is ".", resolve it to the actual current directory.
    // Then, find the parent directory that contains the Cargo.toml for the workspace.
    // For now, we'll assume args.path is the project root if it's explicitly set,
    // otherwise, we'll try to find the workspace root from the current executable's directory.
    let project_root = if args.path == PathBuf::from(".") {
        // If path is ".", it means the current directory of the prelude-generator executable.
        // The actual project root is its parent.
        std::env::current_dir()?.parent().unwrap().to_path_buf()
    } else {
        args.path.clone()
    };


    let config = if let Some(config_path) = &args.config_file_path {
        Some(config_parser::read_config(config_path, &project_root)?)
    } else {
        // If config_file_path is not provided, try to read from the default location
        let default_config_path = project_root.join("config.toml");
        if default_config_path.exists() {
            Some(config_parser::read_config(&default_config_path, &project_root)?)
        } else {
            None
        }
    };
    Ok((args, config))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.analyze_ast {
        let path = args.ast_analysis_path.ok_or_else(|| anyhow::anyhow!("ast_analysis_path is required when analyze_ast is true"))?;
        println!("Analyzing AST for project: {}", path.display());
        // let mut pipeline = Pipeline::new();
        // pipeline.add_functor(ParseFunctor);
        // pipeline.add_functor(AstTraversalFunctor);

        // let results: Vec<AstStatistics> = pipeline.run_on_project(&path).await?;

        // for (i, stats) in results.iter().enumerate() {
        //     println!("--- AST Statistics for file {} ---", i + 1);
        //     println!("Node Type Counts: {:?}", stats.node_type_counts);
        //     println!("Function Definitions: {:?}", stats.function_definitions);
        //     println!("Variable Declarations: {:?}", stats.variable_declarations);
        //     println!("Import Statements: {:?}", stats.import_statements);
        // }
        return Ok(()); // Exit after AST analysis if requested
    }



    if args.generate_test_report {
        let output_file = args.test_report_output_file.unwrap_or_else(|| PathBuf::from("test_report.json"));
        // generate_test_report_json(&args.path)?;
    }

    if args.compile_tests {
        let input_file = args.test_report_input_file.ok_or_else(|| anyhow::anyhow!("test_report_input_file is required when compile_tests is true"))?;
        let output_dir = args.test_verification_output_dir.ok_or_else(|| anyhow::anyhow!("test_verification_output_dir is required when compile_tests is true"))?;
        // generate_test_verification_script_and_report(&input_file)?;
    }

    if args.extract_use_statements {
        let output_dir = args.use_statements_output_dir.ok_or_else(|| anyhow::anyhow!("use_statements_output_dir is required when extract_use_statements is true"))?;
        // TODO: Implement actual use statement extraction logic here
        println!("Extracting use statements to: {}", output_dir.display());
    }

    if args.collect_and_process_use_statements {
        // TODO: Implement logic for collecting and processing use statements
        println!("Collecting and processing use statements...");
    }

    if args.generate_aggregated_test_file {
        // TODO: Implement logic for generating aggregated test file
        println!("Generating aggregated test file...");
    }

    if args.run_pipeline {
        // TODO: Implement logic for running the main pipeline
        println!("Running main pipeline...");
    }

    if args.verify_config {
        // TODO: Implement config verification logic
        println!("Verifying configuration...");
    }

    // If no specific command was executed, print help or a default message
    if !args.analyze_ast && !args.generate_test_report && !args.compile_tests && !args.extract_use_statements && !args.collect_and_process_use_statements && !args.generate_aggregated_test_file && !args.run_pipeline && !args.verify_config {
        println!("No specific command executed. Use --help for options.");
    }

    Ok(())
}

async fn handle_pipeline_result(result: anyhow::Result<()>) -> anyhow::Result<()> {
    if let Err(ref e) = result {
        let mut stderr = tokio::io::stderr();
        stderr.write_all(format!("Pipeline failed: {:?}\n", e).as_bytes()).await?;
    } else {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(b"Pipeline completed successfully.\n").await?;
    }
    result
}

fn generate_ast_statistics_code(stats: &pipeline_traits::AstStatistics) -> String {
    let mut code = String::new();
    code.push_str("use std::collections::HashMap;\n");
    code.push_str("use once_cell::sync::Lazy;\n");
    code.push_str("use pipeline_traits::AstStatistics;\n\n");

    code.push_str("pub static AST_STATISTICS: Lazy<AstStatistics> = Lazy::new(|| {\n");
    code.push_str("    let mut node_type_counts = HashMap::new();\n");
    for (node_type, count) in &stats.node_type_counts {
        code.push_str(&format!("    node_type_counts.insert(\"{}\".to_string(), {});\n", node_type, count));
    }
    code.push_str("\n");

    // code.push_str("    let mut line_stats = HashMap::new();\n");
    // for (node_type, (min, max, sum, count)) in &stats.line_stats {
    //     code.push_str(&format!("    line_stats.insert(\"{}\".to_string(), ({}, {}, {}, {}));\n", node_type, min, max, sum, count));
    // }
    code.push_str("\n");

    // code.push_str("    let mut column_stats = HashMap::new();\n");
    // for (node_type, (min, max, sum, count)) in &stats.column_stats {
    //     code.push_str(&format!("    column_stats.insert(\"{}\".to_string(), ({}, {}, {}, {}));\n", node_type, min, max, sum, count));
    // }
    code.push_str("\n");

    // code.push_str("    let mut processing_time_stats = HashMap::new();\n");
    // for (node_type, (min, max, sum, count)) in &stats.processing_time_stats {
    //     code.push_str(&format!("    processing_time_stats.insert(\"{}\".to_string(), ({}, {}, {}, {}));\n", node_type, min, max, sum, count));
    // }
    code.push_str("\n");

    // code.push_str("    let mut rust_version_counts = HashMap::new();\n");
    // for (version, count) in &stats.rust_version_counts {
    //     code.push_str(&format!("    rust_version_counts.insert(\"{}\".to_string(), {});\n", version, count));
    // }
    code.push_str("\n");

    // code.push_str("    let mut analyzer_version_counts = HashMap::new();\n");
    // for (version, count) in &stats.analyzer_version_counts {
    //     code.push_str(&format!("    analyzer_version_counts.insert(\"{}\".to_string(), {});\n", version, count));
    // }
    code.push_str("\n");

    // code.push_str("    let mut snippet_length_stats = HashMap::new();\n");
    // for (node_type, (min, max, sum, count)) in &stats.snippet_length_stats {
    //     code.push_str(&format!("    snippet_length_stats.insert(\"{}\".to_string(), ({}, {}, {}, {}));\n", node_type, min, max, sum, count));
    // }
    code.push_str("\n");

    code.push_str("    AstStatistics {\n");
    code.push_str("        node_type_counts,\n");
    code.push_str("        // line_stats,\n");
    code.push_str("        // column_stats,\n");
    code.push_str("        // processing_time_stats,\n");
    code.push_str("        // rust_version_counts,\n");
    code.push_str("        // analyzer_version_counts,\n");
    code.push_str("        // snippet_length_stats,\n");
    code.push_str("    }\n");
    code.push_str("});\n");

    code
}

