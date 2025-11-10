use clap::Parser;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use code_graph_flattener::{CodeGraph, GraphNode, GraphEdge};
use std::collections::HashMap;
use regex::Regex;
use prelude_generator::types::CollectedAnalysisData;
use std::collections::HashSet;

// mod command_implementations;
// mod command_mocks;
// mod generated_command_traits;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the serialized CodeGraph JSON file.
    #[arg(long)]
    graph_path: PathBuf,

    /// Path to the serialized CollectedAnalysisData JSON file.
    #[arg(long)]
    analysis_data_path: Option<PathBuf>,

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
    let mut code_graph: CodeGraph = serde_json::from_str(&graph_content)
        .context("Failed to deserialize CodeGraph from JSON")?;

    println!("CodeGraph loaded with {} nodes and {} edges.", code_graph.nodes.len(), code_graph.edges.len());

    let collected_analysis_data: Option<CollectedAnalysisData> = if let Some(path) = &args.analysis_data_path {
        println!("Loading CollectedAnalysisData from: {}", path.display());
        let analysis_data_content = fs::read_to_string(path)
            .context(format!("Failed to read CollectedAnalysisData file from {}", path.display()))?;
        Some(serde_json::from_str(&analysis_data_content)
            .context("Failed to deserialize CollectedAnalysisData from JSON")?)
    } else {
        None
    };

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
        "trait-classification" => {
            report_content.push_str("Trait Classification Report:

");
            let analysis_data = collected_analysis_data.as_ref()
                .context("CollectedAnalysisData is required for 'trait-classification' query.")?;

            let mut existing_node_ids: HashSet<String> = code_graph.nodes.iter().map(|n| n.id.clone()).collect();
            let mut existing_edge_ids: HashSet<String> = code_graph.edges.iter().map(|e| format!("{}_{}_{}", e.source, e.edge_type, e.target)).collect();

            // Regex to parse "Type for Trait" or "Type"
            let impl_for_trait_regex = Regex::new(r"^(?P<implementing_type>[^ ]+) for (?P<implemented_trait>.*)$").unwrap();
            let impl_inherent_regex = Regex::new(r"^(?P<implementing_type>[^ ]+)$").unwrap();

            let mut trait_implementations_found = 0;
            let mut inherent_impls_found = 0;

            for (impl_for_type_str, impl_lattice) in &analysis_data.impl_lattices {
                let implementing_type_name;
                let mut implemented_trait_name: Option<String> = None;
                let mut is_inherent_impl = false;

                if let Some(captures) = impl_for_trait_regex.captures(impl_for_type_str) {
                    implementing_type_name = captures["implementing_type"].to_string();
                    implemented_trait_name = Some(captures["implemented_trait"].to_string());
                } else if let Some(captures) = impl_inherent_regex.captures(impl_for_type_str) {
                    implementing_type_name = captures["implementing_type"].to_string();
                    is_inherent_impl = true;
                } else {
                    // Fallback for more complex impl_for_type_str, treat as inherent for now
                    implementing_type_name = impl_for_type_str.to_string();
                    is_inherent_impl = true;
                }

                let implementing_type_node_id = format!("type_{}", implementing_type_name);

                // Ensure implementing type node exists
                if !existing_node_ids.contains(&implementing_type_node_id) {
                    code_graph.nodes.push(GraphNode {
                        id: implementing_type_node_id.clone(),
                        node_type: "TypeDefinition".to_string(),
                        properties: HashMap::from([("name".to_string(), implementing_type_name.clone())]),
                    });
                    existing_node_ids.insert(implementing_type_node_id.clone());
                }

                if is_inherent_impl {
                    let inherent_trait_name = format!("{}_InherentMethods", implementing_type_name);
                    let inherent_trait_node_id = format!("trait_{}", inherent_trait_name);

                    // Ensure inherent trait node exists
                    if !existing_node_ids.contains(&inherent_trait_node_id) {
                        code_graph.nodes.push(GraphNode {
                            id: inherent_trait_node_id.clone(),
                            node_type: "TraitDefinition".to_string(),
                            properties: HashMap::from([("name".to_string(), inherent_trait_name.clone()), ("kind".to_string(), "Inherent".to_string())]),
                        });
                        existing_node_ids.insert(inherent_trait_node_id.clone());
                    }

                    // Add edge: ImplementingType -> Implements -> InherentTrait
                    let impl_edge_id = format!("{}_implements_{}", implementing_type_node_id, inherent_trait_node_id);
                    if !existing_edge_ids.contains(&impl_edge_id) {
                        code_graph.edges.push(GraphEdge {
                            source: implementing_type_node_id.clone(),
                            target: inherent_trait_node_id.clone(),
                            edge_type: "Implements".to_string(),
                            properties: HashMap::new(),
                        });
                        existing_edge_ids.insert(impl_edge_id);
                        inherent_impls_found += 1;
                    }

                    // Add methods to the inherent trait
                    for (method_names_str, _count) in &impl_lattice.method_co_occurrences {
                        for method_name in method_names_str.split("::") {
                            let method_node_id = format!("method_{}_{}", inherent_trait_node_id, method_name);
                            if !existing_node_ids.contains(&method_node_id) {
                                code_graph.nodes.push(GraphNode {
                                    id: method_node_id.clone(),
                                    node_type: "MethodSignature".to_string(),
                                    properties: HashMap::from([("name".to_string(), method_name.to_string())]),
                                });
                                existing_node_ids.insert(method_node_id.clone());
                            }
                            let has_method_edge_id = format!("{}_has_method_{}", inherent_trait_node_id, method_node_id);
                            if !existing_edge_ids.contains(&has_method_edge_id) {
                                code_graph.edges.push(GraphEdge {
                                    source: inherent_trait_node_id.clone(),
                                    target: method_node_id.clone(),
                                    edge_type: "HasMethod".to_string(),
                                    properties: HashMap::new(),
                                });
                                existing_edge_ids.insert(has_method_edge_id);
                            }
                        }
                    }

                } else if let Some(trait_name) = implemented_trait_name {
                    let trait_node_id = format!("trait_{}", trait_name);

                    // Ensure trait node exists
                    if !existing_node_ids.contains(&trait_node_id) {
                        code_graph.nodes.push(GraphNode {
                            id: trait_node_id.clone(),
                            node_type: "TraitDefinition".to_string(),
                            properties: HashMap::from([("name".to_string(), trait_name.clone()), ("kind".to_string(), "Explicit".to_string())]),
                        });
                        existing_node_ids.insert(trait_node_id.clone());
                    }

                    // Add edge: ImplementingType -> Implements -> TraitDefinition
                    let impl_edge_id = format!("{}_implements_{}", implementing_type_node_id, trait_node_id);
                    if !existing_edge_ids.contains(&impl_edge_id) {
                        code_graph.edges.push(GraphEdge {
                            source: implementing_type_node_id.clone(),
                            target: trait_node_id.clone(),
                            edge_type: "Implements".to_string(),
                            properties: HashMap::new(),
                        });
                        existing_edge_ids.insert(impl_edge_id);
                        trait_implementations_found += 1;
                    }

                    // Add methods to the explicit trait
                    for (method_names_str, _count) in &impl_lattice.method_co_occurrences {
                        for method_name in method_names_str.split("::") {
                            let method_node_id = format!("method_{}_{}", trait_node_id, method_name);
                            if !existing_node_ids.contains(&method_node_id) {
                                code_graph.nodes.push(GraphNode {
                                    id: method_node_id.clone(),
                                    node_type: "MethodSignature".to_string(),
                                    properties: HashMap::from([("name".to_string(), method_name.to_string())]),
                                });
                                existing_node_ids.insert(method_node_id.clone());
                            }
                            let has_method_edge_id = format!("{}_has_method_{}", trait_node_id, method_node_id);
                            if !existing_edge_ids.contains(&has_method_edge_id) {
                                code_graph.edges.push(GraphEdge {
                                    source: trait_node_id.clone(),
                                    target: method_node_id.clone(),
                                    edge_type: "HasMethod".to_string(),
                                    properties: HashMap::new(),
                                });
                                existing_edge_ids.insert(has_method_edge_id);
                            }
                        }
                    }
                }
            }

            report_content.push_str(&format!("Found {} explicit trait implementations.\n", trait_implementations_found));
            report_content.push_str(&format!("Found {} inherent implementations.\n", inherent_impls_found));
            report_content.push_str("\n--- Enriched Graph Summary ---\n");
            report_content.push_str(&format!("Total Nodes: {}\n", code_graph.nodes.len()));
            report_content.push_str(&format!("Total Edges: {}\n", code_graph.edges.len()));

            report_content.push_str("\nDetailed Trait Implementations:\n");
            let mut trait_to_implementors: HashMap<String, Vec<String>> = HashMap::new();
            let mut type_to_inherent_methods: HashMap<String, Vec<String>> = HashMap::new();

            for edge in &code_graph.edges {
                if edge.edge_type == "Implements" {
                    let implementor_name = edge.source.strip_prefix("type_").unwrap_or(&edge.source).to_string();
                    let trait_id = edge.target.clone();
                    if trait_id.contains("_InherentMethods") {
                        let type_name = trait_id.strip_prefix("trait_").unwrap_or(&trait_id).strip_suffix("_InherentMethods").unwrap_or(&trait_id).to_string();
                        // Collect methods for this inherent impl
                        let methods: Vec<String> = code_graph.edges.iter()
                            .filter(|e| e.source == trait_id && e.edge_type == "HasMethod")
                            .map(|e| e.target.strip_prefix(&format!("method_{}_", trait_id)).unwrap_or(&e.target).to_string())
                            .collect();
                        type_to_inherent_methods.entry(type_name).or_insert_with(Vec::new).extend(methods);
                    } else {
                        let trait_name = trait_id.strip_prefix("trait_").unwrap_or(&trait_id).to_string();
                        trait_to_implementors.entry(trait_name).or_insert_with(Vec::new).push(implementor_name);
                    }
                }
            }

            for (trait_name, implementors) in trait_to_implementors {
                report_content.push_str(&format!("Trait '{}' implemented by:\n", trait_name));
                for implementor in implementors {
                    report_content.push_str(&format!("  - {}\n", implementor));
                }
                report_content.push_str("\n");
            }

            for (type_name, methods) in type_to_inherent_methods {
                report_content.push_str(&format!("Type '{}' has inherent methods:\n", type_name));
                for method in methods {
                    report_content.push_str(&format!("  - {}\n", method));
                }
                report_content.push_str("\n");
            }
        },
        _ => {
            report_content.push_str(&format!("Error: Unknown query type '{}'\n", args.query_type));
        }
    } // Closing brace for match statement

    fs::create_dir_all(args.output_path.parent().unwrap())
        .context(format!("Failed to create parent directory for query report: {}", args.output_path.display()))?;
    fs::write(&args.output_path, report_content)
        .context(format!("Failed to write query report to {}", args.output_path.display()))?;
    println!("Query report successfully written to {}", args.output_path.display());

    Ok(())
} // Closing brace for main function