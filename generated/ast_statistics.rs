use std::collections::HashMap;
use once_cell::sync::Lazy;
use pipeline_traits::AstStatistics;

pub static AST_STATISTICS: Lazy<AstStatistics> = Lazy::new(|| {
    let mut node_type_counts = HashMap::new();
    node_type_counts.insert("variable".to_string(), 6);
    node_type_counts.insert("import".to_string(), 6);
    node_type_counts.insert("struct".to_string(), 1);
    node_type_counts.insert("impl".to_string(), 2);
    node_type_counts.insert("other".to_string(), 63);
    node_type_counts.insert("function".to_string(), 2);

    let mut line_stats = HashMap::new();
    line_stats.insert("other".to_string(), (9, 80, 2969, 63));
    line_stats.insert("variable".to_string(), (19, 75, 188, 6));
    line_stats.insert("struct".to_string(), (7, 7, 7, 1));
    line_stats.insert("function".to_string(), (10, 25, 35, 2));
    line_stats.insert("import".to_string(), (1, 6, 21, 6));
    line_stats.insert("impl".to_string(), (8, 12, 20, 2));

    let mut column_stats = HashMap::new();
    column_stats.insert("import".to_string(), (1, 1, 6, 6));
    column_stats.insert("function".to_string(), (1, 1, 2, 2));
    column_stats.insert("impl".to_string(), (1, 1, 2, 2));
    column_stats.insert("struct".to_string(), (1, 1, 1, 1));
    column_stats.insert("variable".to_string(), (1, 1, 6, 6));
    column_stats.insert("other".to_string(), (1, 1, 63, 63));

    let mut processing_time_stats = HashMap::new();
    processing_time_stats.insert("other".to_string(), (1, 1, 63, 63));
    processing_time_stats.insert("function".to_string(), (1, 1, 2, 2));
    processing_time_stats.insert("impl".to_string(), (1, 1, 2, 2));
    processing_time_stats.insert("variable".to_string(), (1, 1, 6, 6));
    processing_time_stats.insert("import".to_string(), (1, 1, 6, 6));
    processing_time_stats.insert("struct".to_string(), (1, 1, 1, 1));

    let mut rust_version_counts = HashMap::new();
    rust_version_counts.insert("1.86.0".to_string(), 80);

    let mut analyzer_version_counts = HashMap::new();
    analyzer_version_counts.insert("0.3.2000".to_string(), 80);

    let mut snippet_length_stats = HashMap::new();
    snippet_length_stats.insert("other".to_string(), (1, 88, 2837, 63));
    snippet_length_stats.insert("impl".to_string(), (70, 77, 147, 2));
    snippet_length_stats.insert("import".to_string(), (18, 78, 182, 6));
    snippet_length_stats.insert("function".to_string(), (20, 89, 109, 2));
    snippet_length_stats.insert("variable".to_string(), (53, 76, 393, 6));
    snippet_length_stats.insert("struct".to_string(), (29, 29, 29, 1));

    AstStatistics {
        node_type_counts,
        line_stats,
        column_stats,
        processing_time_stats,
        rust_version_counts,
        analyzer_version_counts,
        snippet_length_stats,
    }
});
