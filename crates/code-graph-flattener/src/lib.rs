use anyhow::Result;
use prelude_generator::types::CollectedAnalysisData;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Define a generic GraphNode structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String, // e.g., "Struct", "Enum", "Function", "Expression"
    pub properties: HashMap<String, String>, // Additional metadata
}

// Define a generic GraphEdge structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub edge_type: String, // e.g., "Uses", "DependsOn", "Contains"
    pub properties: HashMap<String, String>, // Additional metadata
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CodeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Flattens CollectedAnalysisData into a generic graph representation.
pub fn flatten_analysis_data_to_graph(
    analysis_data: CollectedAnalysisData,
) -> Result<CodeGraph> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut existing_nodes: HashMap<String, bool> = HashMap::new(); // To track existing nodes and avoid duplicates

    // Helper to add a node if it doesn't exist
    let mut add_node_if_not_exists = |id: String, node_type: String, properties: HashMap<String, String>| {
        if !existing_nodes.contains_key(&id) {
            nodes.push(GraphNode { id: id.clone(), node_type, properties });
            existing_nodes.insert(id, true);
        }
    };

    // Process expressions
    for (expr_str, expr_info) in analysis_data.expressions {
        let expr_node_id = format!("expr_{}", expr_str);
        let mut expr_props = HashMap::new();
        expr_props.insert("expression_string".to_string(), expr_info.expression_str.clone());
        expr_props.insert("depth".to_string(), expr_info.depth.to_string());
        expr_props.insert("other_types_count".to_string(), expr_info.other_types_count.to_string());
        expr_props.insert("node_type".to_string(), expr_info.node_type.clone());

        add_node_if_not_exists(
            expr_node_id.clone(),
            "Expression".to_string(),
            expr_props,
        );

        // Add nodes for types used in expressions and create edges
        for used_type in expr_info.used_types {
            let type_node_id = format!("type_{}", used_type);
            let mut type_props = HashMap::new();
            type_props.insert("name".to_string(), used_type.clone());

            add_node_if_not_exists(
                type_node_id.clone(),
                "Type".to_string(),
                type_props,
            );
            edges.push(GraphEdge {
                source: expr_node_id.clone(),
                target: type_node_id,
                edge_type: "UsesType".to_string(),
                properties: HashMap::new(),
            });
        }
    }

    // Process struct lattices
    for (struct_name, struct_lattice) in analysis_data.struct_lattices {
        let struct_node_id = format!("struct_{}", struct_name);
        let mut struct_props = HashMap::new();
        struct_props.insert("name".to_string(), struct_name.clone());
        struct_props.insert("total_expressions_analyzed".to_string(), struct_lattice.total_expressions_analyzed.to_string());

        add_node_if_not_exists(
            struct_node_id.clone(),
            "Struct".to_string(),
            struct_props,
        );

        for (field_types_set, count) in struct_lattice.field_co_occurrences {
            let co_occurrence_id = format!("struct_co_occurrence_{}_{:?}", struct_name, field_types_set);
            let mut co_occurrence_props = HashMap::new();
            co_occurrence_props.insert("fields".to_string(), format!("{:?}", field_types_set));
            co_occurrence_props.insert("count".to_string(), count.to_string());

            add_node_if_not_exists(
                co_occurrence_id.clone(),
                "StructFieldCoOccurrence".to_string(),
                co_occurrence_props,
            );
            edges.push(GraphEdge {
                source: struct_node_id.clone(),
                target: co_occurrence_id.clone(),
                edge_type: "HasFieldCoOccurrence".to_string(),
                properties: HashMap::new(),
            });

            // Also add nodes for individual field types within the co-occurrence
            for field_type in field_types_set {
                let field_node_id = format!("type_{}", field_type); // Assuming field_type is a type name
                let mut field_props = HashMap::new();
                field_props.insert("name".to_string(), field_type.clone());

                add_node_if_not_exists(
                    field_node_id.clone(),
                    "Type".to_string(),
                    field_props,
                );
                edges.push(GraphEdge {
                    source: co_occurrence_id.clone(),
                    target: field_node_id,
                    edge_type: "ContainsFieldType".to_string(),
                    properties: HashMap::new(),
                });
            }
        }
    }

    // Process enum lattices
    for (enum_name, enum_lattice) in analysis_data.enum_lattices {
        let enum_node_id = format!("enum_{}", enum_name);
        let mut enum_props = HashMap::new();
        enum_props.insert("name".to_string(), enum_name.clone());
        enum_props.insert("total_expressions_analyzed".to_string(), enum_lattice.total_expressions_analyzed.to_string());

        add_node_if_not_exists(
            enum_node_id.clone(),
            "Enum".to_string(),
            enum_props,
        );

        for (variant_types_set, count) in enum_lattice.variant_type_co_occurrences {
            let co_occurrence_id = format!("enum_co_occurrence_{}_{:?}", enum_name, variant_types_set);
            let mut co_occurrence_props = HashMap::new();
            co_occurrence_props.insert("variants".to_string(), format!("{:?}", variant_types_set));
            co_occurrence_props.insert("count".to_string(), count.to_string());

            add_node_if_not_exists(
                co_occurrence_id.clone(),
                "EnumVariantCoOccurrence".to_string(),
                co_occurrence_props,
            );
            edges.push(GraphEdge {
                source: enum_node_id.clone(),
                target: co_occurrence_id.clone(),
                edge_type: "HasVariantCoOccurrence".to_string(),
                properties: HashMap::new(),
            });

            // Also add nodes for individual variant types within the co-occurrence
            for variant_type in variant_types_set {
                let type_node_id = format!("type_{}", variant_type); // Assuming variant_type is a type name
                let mut type_props = HashMap::new();
                type_props.insert("name".to_string(), variant_type.clone());

                add_node_if_not_exists(
                    type_node_id.clone(),
                    "Type".to_string(),
                    type_props,
                );
                edges.push(GraphEdge {
                    source: co_occurrence_id.clone(),
                    target: type_node_id,
                    edge_type: "ContainsVariantType".to_string(),
                    properties: HashMap::new(),
                });
            }
        }
    }

    // Process impl lattices
    for (impl_for_type, impl_lattice) in analysis_data.impl_lattices {
        let impl_node_id = format!("impl_for_{}", impl_for_type);
        let mut impl_props = HashMap::new();
        impl_props.insert("for_type".to_string(), impl_for_type.clone());
        impl_props.insert("total_expressions_analyzed".to_string(), impl_lattice.total_expressions_analyzed.to_string());

        add_node_if_not_exists(
            impl_node_id.clone(),
            "ImplBlock".to_string(),
            impl_props,
        );

        for (method_names_set, count) in impl_lattice.method_co_occurrences {
            let co_occurrence_id = format!("impl_method_co_occurrence_{}_{:?}", impl_for_type, method_names_set);
            let mut co_occurrence_props = HashMap::new();
            co_occurrence_props.insert("methods".to_string(), format!("{:?}", method_names_set));
            co_occurrence_props.insert("count".to_string(), count.to_string());

            add_node_if_not_exists(
                co_occurrence_id.clone(),
                "ImplMethodCoOccurrence".to_string(),
                co_occurrence_props,
            );
            edges.push(GraphEdge {
                source: impl_node_id.clone(),
                target: co_occurrence_id.clone(),
                edge_type: "HasMethodCoOccurrence".to_string(),
                properties: HashMap::new(),
            });

            // Also add nodes for individual methods within the co-occurrence
            for method_name in method_names_set {
                let method_node_id = format!("method_{}", method_name); // Assuming method_name is unique enough
                let mut method_props = HashMap::new();
                method_props.insert("name".to_string(), method_name.clone());

                add_node_if_not_exists(
                    method_node_id.clone(),
                    "Method".to_string(),
                    method_props,
                );
                edges.push(GraphEdge {
                    source: co_occurrence_id.clone(),
                    target: method_node_id,
                    edge_type: "ContainsMethod".to_string(),
                    properties: HashMap::new(),
                });
            }
        }
    }

    Ok(CodeGraph { nodes, edges })
}
