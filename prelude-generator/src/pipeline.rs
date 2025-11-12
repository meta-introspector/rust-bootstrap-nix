use anyhow::Context;

use std::path::PathBuf;
use crate::args::Args;
use pipeline_traits::Config; // Use Config from pipeline_traits
use pipeline_traits::{RawFile, ValidatedFile, PipelineFunctor};
use crate::parser::ParseFunctor;
use crate::prelude_category_pipeline::{ExtractUsesFunctor, ClassifyUsesFunctor, AstReconstructionFunctor};
use crate::measurement;

pub async fn run_category_pipeline<W: tokio::io::AsyncWriteExt + Unpin + Send>(
    writer: &mut W,
    file_content: &str,
    file_path_str: &str,
    _args: &Args,
    _config: &Option<Config>, // Changed to pipeline_traits::Config
) -> anyhow::Result<()> {
    let raw_file = RawFile(file_path_str.to_string(), file_content.to_string());

    writer.write_all(format!("---\n--- Stage 1: Parsing ---\n").as_bytes()).await?;
    let parse_functor = ParseFunctor;
    let parsed_file = parse_functor.map(writer, raw_file, _config).await.context("Parsing failed")?;
    writer.write_all(format!("  -> Parsed file successfully.\n").as_bytes()).await?;

    writer.write_all(format!("---\n--- Stage 2: Extracting Use Statements ---\n").as_bytes()).await?;
    let extract_uses_functor = ExtractUsesFunctor;
    let use_statements = extract_uses_functor.map(writer, parsed_file.clone(), _config).await.context("Extracting use statements failed")?;
    writer.write_all(format!("  -> Extracted {} use statements.\n", use_statements.0.len()).as_bytes()).await?;

    writer.write_all(format!("---\n--- Stage 3: Classifying Use Statements ---\n").as_bytes()).await?;
    let classify_uses_functor = ClassifyUsesFunctor;
    let classified_uses = classify_uses_functor.map(writer, use_statements, _config).await.context("Classifying use statements failed")?;
    writer.write_all(format!("  -> Classified use statements:\n").as_bytes()).await?;
    writer.write_all(format!("{:#?}\n", classified_uses).as_bytes()).await?;

    writer.write_all(format!("---\n--- Stage 4: AST Reconstruction from Hugging Face Dataset ---\n").as_bytes()).await?;
    let ast_reconstruction_functor = AstReconstructionFunctor;
    let validated_file = ValidatedFile(parsed_file.0.clone(), parsed_file.1.clone());
    let reconstructed_code = PipelineFunctor::map(&ast_reconstruction_functor, writer, validated_file.clone(), _config).await.context("AST Reconstruction failed")?;
    writer.write_all(format!("  -> AST Reconstruction completed successfully.\n").as_bytes()).await?;

    // Write generated code to a file
    let generated_output_dir = _config.as_ref()
        .and_then(|c| c.generated_output_dir.as_ref())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".gemini/generated/"));

    let output_file_path = generated_output_dir.join("self_generated_code.rs"); // Define output path
    tokio::fs::create_dir_all(output_file_path.parent().unwrap()).await?;
    tokio::fs::write(&output_file_path, reconstructed_code.as_bytes()).await
        .context(format!("Failed to write generated code to {:?}", output_file_path))
        ?;
    writer.write_all(format!("  -> Generated code written to {:?}\n", output_file_path).as_bytes()).await?;

    // Validate the generated code
    writer.write_all(format!("---\n--- Validating Generated Code ---\n").as_bytes()).await?;
    crate::utils::validate_rust_code(&output_file_path).await
        .context(format!("Generated code validation failed for {:?}", output_file_path))
        ?;
    writer.write_all(format!("  -> Generated code validated successfully.\n").as_bytes()).await?;

    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).context("Failed to serialize metrics to JSON")?;
    writer.write_all(format!("---\n--- METRICS_START ---\n{}\n--- METRICS_END ---\n", json_metrics).as_bytes()).await?;

    Ok(())
}