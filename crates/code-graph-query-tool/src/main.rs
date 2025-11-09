use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use code_graph_flattener::{CodeGraph, GraphNode, GraphEdge};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the serialized CodeGraph JSON file.
    #[arg(long)]
    graph_path: PathBuf,

    /// Type of query to perform (e.g., "command-usage").
    #[arg(long)]
    query_type: String,

    /// Path to save the query report.
    #[arg(long)]
    output_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Loading CodeGraph from: {}", args.graph_path.display());
    let graph_content = fs::read_to_string(&args.graph_path)
        .context(format!("Failed to read CodeGraph file from {}", args.graph_path.display()))?;
    let code_graph: CodeGraph = serde_json::from_str(&graph_content)
        .context("Failed to deserialize CodeGraph from JSON")?;

    println!("CodeGraph loaded with {} nodes and {} edges.", code_graph.nodes.len(), code_graph.edges.len());

    let mut report_content = String::new();
    report_content.push_str(&format!("Query Report for CodeGraph: {}
", args.graph_path.display()));
    report_content.push_str(&format!("Query Type: {}

", args.query_type));

    match args.query_type.as_str() {
        "command-usage" => {
            report_content.push_str("Command Object Usage Report:

");

            let mut command_related_nodes = Vec::new();
            for node in &code_graph.nodes {
                if node.node_type == "Type" && node.id.contains("Command") {
                    command_related_nodes.push(node);
                } else if node.node_type == "Expression" {
                    if let Some(expr_str) = node.properties.get("expression_string") {
                        if expr_str.contains("Command") {
                            command_related_nodes.push(node);
                        }
                    }
                }
            }

            if command_related_nodes.is_empty() {
                report_content.push_str("No direct Command object usage found in nodes.\n");
            } else {
                report_content.push_str("Nodes referencing 'Command':\n");
                for node in command_related_nodes {
                    report_content.push_str(&format!("  - ID: {}, Type: {}, Properties: {:?}\n", node.id, node.node_type, node.properties));
                }
            }

            let mut command_related_edges = Vec::new();
            for edge in &code_graph.edges {
                if edge.source.contains("Command") || edge.target.contains("Command") {
                    command_related_edges.push(edge);
                }
            }

            if command_related_edges.is_empty() {
                report_content.push_str("\nNo direct Command object usage found in edges.\n");
            } else {
                report_content.push_str("\nEdges referencing 'Command':\n");
                for edge in command_related_edges {
                    report_content.push_str(&format!("  - Source: {}, Target: {}, Type: {}, Properties: {:?}\n", edge.source, edge.target, edge.edge_type, edge.properties));
                }
            }
        },
        "most-used-types" => {
            report_content.push_str("Most Used Types Report:

");
            let mut type_counts: HashMap<String, usize> = HashMap::new();

            let mut type_counts: HashMap<String, usize> = HashMap::new();

            for edge in &code_graph.edges {
                if edge.edge_type == "UsesType" {
                    // The target of a "UsesType" edge is the ID of a Type node (e.g., "type_u32")
                    let type_node_id = &edge.target;
                    let type_name = if let Some(stripped_id) = type_node_id.strip_prefix("type_") {
                        stripped_id.to_string()
                    } else {
                        type_node_id.clone() // Fallback if prefix is not found
                    };
                    *type_counts.entry(type_name).or_insert(0) += 1;
                }
            }

            let mut sorted_types: Vec<(&String, &usize)> = type_counts.iter().collect();
            sorted_types.sort_by(|a, b| b.1.cmp(a.1));

            if sorted_types.is_empty() {
                report_content.push_str("No type nodes found in the graph.\n");
            } else {
                report_content.push_str("Top 20 Most Used Types:\n");
                for (type_name, count) in sorted_types.iter().take(20) {
                    report_content.push_str(&format!("  - {}: {}\n", type_name, count));
                }
            }
        },
        _ => {
            report_content.push_str(&format!("Error: Unknown query type '{}'\n", args.query_type));
        }
    }

    fs::create_dir_all(args.output_path.parent().unwrap())
        .context(format!("Failed to create parent directory for query report: {}", args.output_path.display()))?;
    fs::write(&args.output_path, report_content)
        .context(format!("Failed to write query report to {}", args.output_path.display()))?;
    println!("Query report successfully written to {}", args.output_path.display());

    Ok(())
}