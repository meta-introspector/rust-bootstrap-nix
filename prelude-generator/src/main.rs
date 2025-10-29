use anyhow::Context;
use tokio::io::AsyncWriteExt;
use clap::Parser;
use prelude_generator::Args;
use prelude_generator::category_pipeline::{ClassifyUsesFunctor, ExtractUsesFunctor, ParseFunctor, PipelineFunctor, RawFile, HuggingFaceValidatorFunctor};
use prelude_generator::category_pipeline::{AstReconstructionFunctor};
use std::fs;
use prelude_generator::measurement;
use std::path::Path;

mod hf_dataset_reader;

async fn run_category_pipeline<W: tokio::io::AsyncWriteExt + Unpin + Send>(writer: &mut W, file_path: &Path, _args: &Args) -> anyhow::Result<()> {
    let content = fs::read_to_string(file_path).context("Failed to read file content")?;
    let raw_file = RawFile(file_path.to_string_lossy().to_string(), content);

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

    writer.write_all(format!("--- Stage 4: Hugging Face Validation ---\n").as_bytes()).await?;
    let hf_validator_functor = HuggingFaceValidatorFunctor;
    let validated_file = hf_validator_functor.map(writer, parsed_file.clone()).await.context("Hugging Face Validation failed")?; // Use parsed_file as input
    writer.write_all(format!("  -> Hugging Face Validation Result: {:#?}\n", validated_file).as_bytes()).await?;

    writer.write_all(format!("--- Stage 5: AST Reconstruction from Hugging Face Dataset ---\n").as_bytes()).await?;
    let ast_reconstruction_functor = AstReconstructionFunctor;
    let _reconstructed_ast = PipelineFunctor::map(&ast_reconstruction_functor, writer, validated_file).await.context("AST Reconstruction failed")?;
    writer.write_all(format!("  -> AST Reconstruction completed successfully.\n").as_bytes()).await?;
    // Print collected metrics
    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).context("Failed to serialize metrics to JSON")?;
    writer.write_all(format!("--- METRICS_START ---\n{}\n--- METRICS_END ---\n", json_metrics).as_bytes()).await?;

    Ok(())
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let file_to_process = if let Some(file_name) = args.file.as_ref() {
        Path::new(file_name)
    } else {
        return Err(anyhow::anyhow!("No file specified to process. Use --file argument."));
    };

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()? // This line is causing the error
        .block_on(async {
            let result = run_category_pipeline(&mut tokio::io::stdout(), file_to_process, &args).await;

            if let Err(ref e) = result {
                tokio::io::stderr().write_all(format!("Pipeline failed: {:?}\n", e).as_bytes()).await?;
            } else {
                tokio::io::stdout().write_all(b"Pipeline completed successfully.\n").await?;
            }

            result
        })
}
