use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use anyhow::Context;
use crate::expression_info::ExpressionInfo;
use crate::{struct_lattice_info::StructLatticeInfo, enum_lattice_info::EnumLatticeInfo, impl_lattice_info::ImplLatticeInfo};

pub fn generate_report(
    all_expression_info: &HashMap<String, ExpressionInfo>,
    max_expression_depth: usize,
    output_path: &PathBuf,
    struct_lattices: &HashMap<String, StructLatticeInfo>,
    enum_lattices: &HashMap<String, EnumLatticeInfo>,
    impl_lattices: &HashMap<String, ImplLatticeInfo>,
) -> anyhow::Result<()> {
    let mut report_content = String::new();
    report_content.push_str(&format!("Type Usage Report (Max Depth: {})", max_expression_depth));
    report_content.push_str("\n================================================================\n");

    // Collect all unique user-defined types found in expressions
    let mut all_user_defined_types: HashSet<String> = HashSet::new();
    for (_, info) in all_expression_info {
        for used_type in &info.used_types {
            all_user_defined_types.insert(used_type.clone());
        }
    }

    let mut sorted_user_defined_types: Vec<String> = all_user_defined_types.into_iter().collect();
    sorted_user_defined_types.sort_unstable();

    for target_type in sorted_user_defined_types {
        report_content.push_str(&format!("\n### Analyzing expressions using type: '{}' ###\n", target_type));

        let mut grouped_by_node_type: HashMap<String, HashMap<usize, HashMap<usize, Vec<&ExpressionInfo>>>> = HashMap::new();

        for (_, info) in all_expression_info {
            if info.used_types.contains(&target_type) {
                let other_types_count = info.used_types.len().saturating_sub(1);
                grouped_by_node_type
                    .entry(info.node_type.clone())
                    .or_default()
                    .entry(other_types_count)
                    .or_default()
                    .entry(info.depth)
                    .or_default()
                    .push(info);
            }
        }

        let mut sorted_node_types: Vec<String> = grouped_by_node_type.keys().cloned().collect();
        sorted_node_types.sort_unstable();

        for node_type in sorted_node_types {
            report_content.push_str(&format!("\n--- AST Node Type: {} ---\n", node_type));
            let grouped_by_other_types = grouped_by_node_type.get(&node_type).unwrap();
            let mut sorted_other_types_keys: Vec<usize> = grouped_by_other_types.keys().cloned().collect();
            sorted_other_types_keys.sort_unstable();

            for other_types_count in sorted_other_types_keys {
                report_content.push_str(&format!("  Expressions using '{}' with {} other type(s):\n", target_type, other_types_count));
                let grouped_by_depth = grouped_by_other_types.get(&other_types_count).unwrap();
                let mut sorted_depth_keys: Vec<usize> = grouped_by_depth.keys().cloned().collect();
                sorted_depth_keys.sort_unstable();

                for depth in sorted_depth_keys {
                    let mut expressions_at_depth = grouped_by_depth.get(&depth).unwrap().clone();
                    expressions_at_depth.sort_by_key(|info| info.expression_str.clone());

                    report_content.push_str(&format!("    Depth {}: (Count: {})
", depth, expressions_at_depth.len()));
                    for info in expressions_at_depth {
                        report_content.push_str(&format!("      - '{}' (Used Types: {:?})\n", info.expression_str, info.used_types));
                    }
                }
            }
        }
    }

    fs::write(output_path, &report_content)
        .context(format!("Failed to write type usage report to {:?}", output_path))?;

    // --- Struct Lattice Information ---
    if !struct_lattices.is_empty() {
        report_content.push_str("\n\nStruct Lattice Information\n");
        report_content.push_str("================================================================
");
        for (struct_name, lattice_info) in struct_lattices {
            report_content.push_str(&format!("\n### Struct: '{}' (Expressions Analyzed: {}) ###\n", struct_name, lattice_info.total_expressions_analyzed));
            if lattice_info.field_co_occurrences.is_empty() {
                report_content.push_str("  No field co-occurrence data collected.\n");
            } else {
                let mut sorted_co_occurrences: Vec<(&String, &usize)> = lattice_info.field_co_occurrences.iter().collect();
                sorted_co_occurrences.sort_by_key(|&(_, count)| count);
                for (field_types, count) in sorted_co_occurrences {
                    report_content.push_str(&format!("  - Co-occurring fields: {:?} (Count: {})\n", field_types, count));
                }
            }
        }
    }

    // --- Enum Lattice Information ---
    if !enum_lattices.is_empty() {
        report_content.push_str("\n\nEnum Lattice Information\n");
        report_content.push_str("================================================================
");
        for (enum_name, lattice_info) in enum_lattices {
            report_content.push_str(&format!("\n### Enum: '{}' (Expressions Analyzed: {}) ###\n", enum_name, lattice_info.total_expressions_analyzed));
            if lattice_info.variant_type_co_occurrences.is_empty() {
                report_content.push_str("  No variant type co-occurrence data collected.\n");
            } else {
                let mut sorted_co_occurrences: Vec<(&String, &usize)> = lattice_info.variant_type_co_occurrences.iter().collect();
                sorted_co_occurrences.sort_by_key(|&(_, count)| count);
                for (variant_types, count) in sorted_co_occurrences {
                    report_content.push_str(&format!("  - Co-occurring variant types: {:?} (Count: {})\n", variant_types, count));
                }
            }
        }
    }

    // --- Impl Lattice Information ---
    if !impl_lattices.is_empty() {
        report_content.push_str("\n\nImpl Lattice Information\n");
        report_content.push_str("================================================================
");
        for (impl_for_type, lattice_info) in impl_lattices {
            report_content.push_str(&format!("\n### Impl for Type: '{}' (Expressions Analyzed: {}) ###\n", impl_for_type, lattice_info.total_expressions_analyzed));
            if lattice_info.method_co_occurrences.is_empty() {
                report_content.push_str("  No method co-occurrence data collected.\n");
            } else {
                let mut sorted_co_occurrences: Vec<(&String, &usize)> = lattice_info.method_co_occurrences.iter().collect();
                sorted_co_occurrences.sort_by_key(|&(_, count)| count);
                for (method_names, count) in sorted_co_occurrences {
                    report_content.push_str(&format!("  - Co-occurring methods: {:?} (Count: {})\n", method_names, count));
                }
            }
        }
    }

    Ok(())
}
