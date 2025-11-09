use anyhow::Context;
use std::fs;
use crate::Args;
use walkdir::WalkDir;
use syn::{self, visit::Visit};
use std::collections::HashMap;
use crate::expression_info::ExpressionInfo;
use crate::type_usage_visitor::TypeUsageVisitor;
use crate::report_generator::generate_report;
use crate::{struct_lattice_info::StructLatticeInfo, enum_lattice_info::EnumLatticeInfo, impl_lattice_info::ImplLatticeInfo};
use crate::types::CollectedAnalysisData;
use toml;

pub async fn analyze_type_usage(args: &Args) -> anyhow::Result<CollectedAnalysisData> { // Modified return type
    println!("Running type usage analysis...");

    let max_expression_depth = args.max_expression_depth.context("Max expression depth must be specified for type usage analysis")?;
    let output_path = args.output_type_usage_report.as_ref().context("Output path for type usage report must be specified")?;

    println!("Max Expression Depth: {}", max_expression_depth);
    println!("Output Report Path: {:?}", output_path);

    let mut all_expression_info: HashMap<String, ExpressionInfo> = HashMap::new();
    let mut all_struct_lattices: HashMap<String, StructLatticeInfo> = HashMap::new();
    let mut all_enum_lattices: HashMap<String, EnumLatticeInfo> = HashMap::new();
    let mut all_impl_lattices: HashMap<String, ImplLatticeInfo> = HashMap::new();

    for entry in WalkDir::new(&args.path) {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "rs") {
            println!("Processing file for type usage analysis: {}", file_path.display());

            let file_content = fs::read_to_string(&file_path)
                .context(format!("Failed to read file: {:?}", file_path))?;

            let file = match syn::parse_file(&file_content) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Warning: Could not parse file {}: {}", file_path.display(), e);
                    continue;
                }
            };

            let mut visitor = TypeUsageVisitor::new(max_expression_depth);
            visitor.visit_file(&file);

            for (expr_str, info) in visitor.expressions {
                all_expression_info.insert(expr_str, info);
            }
            all_struct_lattices.extend(visitor.struct_lattices);
            all_enum_lattices.extend(visitor.enum_lattices);
            all_impl_lattices.extend(visitor.impl_lattices);
        }
    }

    generate_report(&all_expression_info, max_expression_depth, output_path, &all_struct_lattices, &all_enum_lattices, &all_impl_lattices)?;

    let collected_data = CollectedAnalysisData { // Construct CollectedAnalysisData
        expressions: all_expression_info,
        struct_lattices: all_struct_lattices,
        enum_lattices: all_enum_lattices,
        impl_lattices: all_impl_lattices,
    };

    if let Some(toml_output_path) = &args.output_toml_report {
        let toml_content = toml::to_string_pretty(&collected_data)
            .context("Failed to serialize collected analysis data to TOML")?;
        fs::write(toml_output_path, toml_content)
            .context(format!("Failed to write TOML report to {:?}", toml_output_path))?;
        println!("TOML report saved to {:?}", toml_output_path);
    }

    println!("Type usage analysis completed. Report saved to {:?}", output_path);
    Ok(collected_data) // Return CollectedAnalysisData
}
