use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use prelude_generator::types::CollectedAnalysisData;
use code_graph_flattener::flatten_analysis_data_to_graph;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the serialized CollectedAnalysisData JSON file.
    #[arg(long)]
    input_analysis_data_path: PathBuf,

    /// Path to save the serialized CodeGraph JSON file.
    #[arg(long)]
    output_graph_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Loading CollectedAnalysisData from: {}", args.input_analysis_data_path.display());
    let analysis_data_content = fs::read_to_string(&args.input_analysis_data_path)
        .context(format!("Failed to read CollectedAnalysisData file from {}", args.input_analysis_data_path.display()))?;
    let collected_analysis_data: CollectedAnalysisData = serde_json::from_str(&analysis_data_content)
        .context("Failed to deserialize CollectedAnalysisData from JSON")?;

    println!("Flattening analysis data to CodeGraph...");
    let code_graph = flatten_analysis_data_to_graph(collected_analysis_data)?;

    println!("CodeGraph generated with {} nodes and {} edges.", code_graph.nodes.len(), code_graph.edges.len());

    let graph_json = serde_json::to_string_pretty(&code_graph)
        .context("Failed to serialize CodeGraph to JSON")?;

    fs::write(&args.output_graph_path, graph_json)
        .context(format!("Failed to write CodeGraph to {}", args.output_graph_path.display()))?;

    println!("CodeGraph successfully written to {}", args.output_graph_path.display());

    Ok(())
}
