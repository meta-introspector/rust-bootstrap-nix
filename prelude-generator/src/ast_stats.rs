

pub fn generate_ast_statistics_code(stats: &pipeline_traits::AstStatistics) -> String {
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
