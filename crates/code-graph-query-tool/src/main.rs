use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use code_graph_flattener::CodeGraph;
use std::collections::HashMap;
use regex::Regex;

mod command_implementations;
mod command_mocks;
mod generated_command_traits;

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
            let command_new_regex = Regex::new(r#"Command::new\("([^"]+)"\)"#).unwrap();
            let mut command_usages: Vec<(String, String, Option<String>, String)> = Vec::new(); // (node_id, expression_string, program_name, classification)

            for node in &code_graph.nodes {
                if node.node_type == "Expression" {
                    if let Some(expr_str) = node.properties.get("expression_string") {
                        if expr_str.contains("Command") {
                            let mut program_name: Option<String> = None;
                            let mut classification = "External/Unknown".to_string();

                            if let Some(captures) = command_new_regex.captures(expr_str) {
                                if let Some(name) = captures.get(1) {
                                    program_name = Some(name.as_str().to_string());
                                    // Simple heuristic for local vs external
                                    if name.as_str().starts_with("./") || name.as_str().contains("target/debug") || name.as_str().contains("target/release") {
                                        classification = "Local (Heuristic)".to_string();
                                    } else {
                                        classification = "External (Heuristic)".to_string();
                                    }
                                }
                            }

                            command_usages.push((node.id.clone(), expr_str.clone(), program_name, classification));
                        }
                    }
                }
            }

            if command_usages.is_empty() {
                report_content.push_str("No Command object usage found in expressions.\n");
            } else {
                report_content.push_str("Detailed Command Usages:\n");
                for (node_id, expr_str, prog_name_opt, classification) in command_usages {
                    report_content.push_str(&format!("  - Node ID: {}\n", node_id));
                    report_content.push_str(&format!("    Expression: {}\n", expr_str));
                    if let Some(prog_name) = prog_name_opt {
                        report_content.push_str(&format!("    Program Called: {}\n", prog_name));
                    } else {
                        report_content.push_str("    Program Called: (Not extracted)\n");
                    }
                    report_content.push_str(&format!("    Classification: {}\n", classification));
                    report_content.push_str("\n");
                }
            }
        },
        "most-used-types" => {
            report_content.push_str("Most Used Types Report:

");
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
        "type-definition-locations" => {
            report_content.push_str("Type Definition Locations Report:

");
            let mut type_locations: HashMap<String, Vec<String>> = HashMap::new();

            // First, collect all type nodes
            let mut type_node_ids = HashMap::new();
            for node in &code_graph.nodes {
                if node.node_type == "Type" {
                    type_node_ids.insert(node.id.clone(), node.id.strip_prefix("type_").unwrap_or(&node.id).to_string());
                }
            }

            // Then, iterate through edges to find where these types are referenced
            for edge in &code_graph.edges {
                if let Some(type_name) = type_node_ids.get(&edge.target) {
                    let location_description = format!("  - Referenced by '{}' ({} edge)", edge.source, edge.edge_type);
                    type_locations.entry(type_name.clone()).or_insert_with(Vec::new).push(location_description);
                }
            }

            if type_locations.is_empty() {
                report_content.push_str("No type definitions or usages found in the graph.\n");
            } else {
                let mut sorted_types: Vec<(&String, &Vec<String>)> = type_locations.iter().collect();
                sorted_types.sort_by_key(|a| a.0); // Sort by type name

                for (type_name, locations) in sorted_types {
                    report_content.push_str(&format!("Type: {}\n", type_name));
                    for location in locations {
                        report_content.push_str(&format!("{}\n", location));
                    }
                    report_content.push_str("\n");
                }
            }
        },
        "struct-field-access-frequency" => {
            report_content.push_str("Struct Field Access Frequency Report:

");
            let mut struct_field_access: HashMap<String, HashMap<String, usize>> = HashMap::new();

            for node in &code_graph.nodes {
                if node.node_type == "StructFieldCoOccurrence" {
                    if let Some(fields_str) = node.properties.get("fields") {
                        if let Some(count_str) = node.properties.get("count") {
                            if let Ok(count) = count_str.parse::<usize>() {
                                // Extract struct name from node.id
                                // node.id format: "struct_co_occurrence_{struct_name}_{field_types_str}"
                                let parts: Vec<&str> = node.id.splitn(3, '_').collect();
                                if parts.len() == 3 {
                                    let struct_name = parts[1].to_string();
                                    struct_field_access
                                        .entry(struct_name)
                                        .or_insert_with(HashMap::new)
                                        .insert(fields_str.clone(), count);
                                }
                            }
                        }
                    }
                }
            }

            if struct_field_access.is_empty() {
                report_content.push_str("No struct field access data found in the graph.\n");
            } else {
                let mut sorted_structs: Vec<(&String, &HashMap<String, usize>)> = struct_field_access.iter().collect();
                sorted_structs.sort_by_key(|a| a.0); // Sort by struct name

                for (struct_name, field_accesses) in sorted_structs {
                    report_content.push_str(&format!("Struct: {}\n", struct_name));
                    let mut sorted_fields: Vec<(&String, &usize)> = field_accesses.iter().collect();
                    sorted_fields.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count, descending

                    for (fields, count) in sorted_fields {
                        report_content.push_str(&format!("  - Fields '{}': {} accesses\n", fields, count));
                    }
                    report_content.push_str("\n");
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