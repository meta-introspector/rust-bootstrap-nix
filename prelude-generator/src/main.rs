use prelude_generator::parser::ParseFunctor;
use prelude_generator::args::Args;
use crate::config_parser::Config;
use prelude_generator::measurement;
use clap::Parser;
use anyhow::Context;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::{Path, PathBuf};
use pipeline_traits::{PipelineFunctor, RawFile};
use prelude_generator::prelude_category_pipeline::{AstReconstructionFunctor, ExtractUsesFunctor, ClassifyUsesFunctor, HuggingFaceValidatorFunctor};

mod hf_dataset_reader;
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
    let reconstructed_code = PipelineFunctor::map(&ast_reconstruction_functor, writer, validated_file).await.context("AST Reconstruction failed")?;
    writer.write_all(format!("  -> AST Reconstruction completed successfully.\n").as_bytes()).await?;

    // Write generated code to a file
    let output_file_path = PathBuf::from("generated/self_generated_code.rs"); // Define output path
    tokio::fs::create_dir_all(output_file_path.parent().unwrap()).await?;
    tokio::fs::write(&output_file_path, reconstructed_code.as_bytes()).await
        .context(format!("Failed to write generated code to {:?}", output_file_path))?;
    writer.write_all(format!("  -> Generated code written to {:?}\n", output_file_path).as_bytes()).await?;

    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).context("Failed to serialize metrics to JSON")?;
    writer.write_all(format!("--- METRICS_START ---\n{}\n--- METRICS_END ---\n", json_metrics).as_bytes()).await?;

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
    let (args, config) = parse_arguments_and_config()?;

    // For debugging: print the parsed config if available
    if let Some(ref cfg) = config {
        eprintln!("Parsed config: {:#?}", cfg);
    }

    if args.verify_config {
        eprintln!("Configuration verification complete. Exiting.");
        return Ok(());
    }

async fn read_input_file(args: &Args) -> anyhow::Result<(String, String)> {
    let file_to_process = if let Some(file_name) = args.file.as_ref() {
        Path::new(file_name)
    } else {
        return Err(anyhow::anyhow!("No file specified to process. Use --file argument."));
    };

    let content = fs::read_to_string(file_to_process).await.context("Failed to read file content")?;
    Ok((content, file_to_process.to_string_lossy().to_string()))
}

    let (content, file_path_str) = read_input_file(&args).await?;

    let mut stdout = tokio::io::stdout();
    let result = run_category_pipeline(&mut stdout, &content, &file_path_str, &args, &config).await;
    handle_pipeline_result(result).await
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


