use anyhow::Context;
use std::path::PathBuf;
use syn::{self, visit::Visit};
use crate::{constant_storage, BagOfWordsVisitor, config_parser, pipeline, type_extractor};

pub fn handle_analyze_ast(_args: &crate::Args) -> anyhow::Result<()> {
    let path = _args.ast_analysis_path.clone().ok_or_else(|| anyhow::anyhow!("ast_analysis_path is required when analyze_ast is true"))?;
    println!("Analyzing AST for project: {}", path.display());
    Ok(())
}

pub fn handle_generate_test_report(_args: &crate::Args) -> anyhow::Result<()> {
    let _output_file = _args.test_report_output_file.clone().unwrap_or_else(|| PathBuf::from("test_report.json"));
    // TODO: Implement generate_test_report_json(&_args.path)?;
    Ok(())
}

pub fn handle_compile_tests(_args: &crate::Args) -> anyhow::Result<()> {
    let _input_file = _args.test_report_input_file.clone().ok_or_else(|| anyhow::anyhow!("test_report_input_file is required when compile_tests is true"))?;
    let _output_dir = _args.test_verification_output_dir.clone().ok_or_else(|| anyhow::anyhow!("test_verification_output_dir is required when compile_tests is true"))?;
    // TODO: Implement generate_test_verification_script_and_report(&_input_file)?;
    Ok(())
}

pub fn handle_extract_use_statements(_args: &crate::Args) -> anyhow::Result<()> {
    let output_dir = _args.use_statements_output_dir.clone().ok_or_else(|| anyhow::anyhow!("use_statements_output_dir is required when extract_use_statements is true"))?;
    // TODO: Implement actual use statement extraction logic here
    println!("Extracting use statements to: {}", output_dir.display());
    Ok(())
}

pub fn handle_collect_and_process_use_statements() {
    // TODO: Implement logic for collecting and processing use statements
    println!("Collecting and processing use statements...");
}

pub fn handle_generate_aggregated_test_file() {
    // TODO: Implement logic for generating aggregated test file
    println!("Generating aggregated test file...");
}

pub async fn handle_run_pipeline(args: &crate::Args, config: &config_parser::Config) -> anyhow::Result<()> {
    println!("Running main pipeline...");
    let mut stdout = tokio::io::stdout();
    let dummy_content = "fn main() { println!(\"Hello, world!\"); }".to_string();
    let dummy_path = "dummy_file.rs".to_string();

    pipeline::run_category_pipeline(
        &mut stdout,
        &dummy_content,
        &dummy_path,
        &args,
        &Some(config.clone()),
    ).await?;
    Ok(())
}

pub fn handle_verify_config() {
    // TODO: Implement config verification logic
    println!("Verifying configuration...");
}

// Removed use crate::gem_parser::GemConfig;
// Removed use cargo_metadata::{MetadataCommand, Package};

pub async fn handle_extract_global_level0_decls(
    _project_root: &PathBuf,
    _args: &crate::Args,
    _all_numerical_constants: &mut Vec<syn::ItemConst>,
    _all_string_constants: &mut Vec<syn::ItemConst>,
    _rustc_info: &crate::use_extractor::rustc_info::RustcInfo,
    _cache_dir: &std::path::Path,
) -> anyhow::Result<()> {
    println!("handle_extract_global_level0_decls is now handled by split-expanded-bin.");
    Ok(())
}

pub async fn handle_extract_numerical_constants(    _project_root: &PathBuf,
    _args: &crate::Args,
    all_numerical_constants: &Vec<syn::ItemConst>,
) -> anyhow::Result<()> {
    let numerical_output_dir = _project_root.join(r"generated/numerical_constants");
    tokio::fs::create_dir_all(&numerical_output_dir).await
        .context(format!("Failed to create output directory {:?}", numerical_output_dir))?;
    constant_storage::numerical_constants::write_numerical_constants_to_hierarchical_structure(&all_numerical_constants, &numerical_output_dir).await?;
    println!(r"  -> Numerical constants will be written to: {:?}", numerical_output_dir);
    println!(r"  -> Total numerical constants extracted: {}", all_numerical_constants.len());
    Ok(())
}

pub async fn handle_extract_string_constants(
    _project_root: &PathBuf,
    _args: &crate::Args,
    all_string_constants: &Vec<syn::ItemConst>,
) -> anyhow::Result<()> {
    let string_output_dir = _project_root.join(r"generated/string_constants");
    tokio::fs::create_dir_all(&string_output_dir).await
        .context(format!("Failed to create output directory {:?}", string_output_dir))?;
    constant_storage::string_constants::write_string_constants_to_hierarchical_structure(&all_string_constants, &string_output_dir).await?;
    println!(r"  -> String constants will be written to: {:?}", string_output_dir);
    println!(r"  -> Total string constants extracted: {}", all_string_constants.len());
    Ok(())
}

pub fn handle_analyze_bag_of_words(
    _project_root: &PathBuf,
    args: &crate::Args,
) -> anyhow::Result<()> {
    println!(r"Analyzing bag of words...");
    let project_root = if args.path == PathBuf::from(".") {
        std::env::current_dir()?.parent().unwrap().to_path_buf()
    } else {
        args.path.clone()
    };

    let mut bag_of_words_visitor = BagOfWordsVisitor::new();
    let mut files_processed_for_bow = 0;

    for entry in walkdir::WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == r"rs"))
    {
        let path = entry.path();
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(file) = syn::parse_file(&content) {
                bag_of_words_visitor.visit_file(&file);
                files_processed_for_bow += 1;
            } else {
                eprintln!(r"Warning: Could not parse file for bag of words analysis: {}", path.display());
            }
        } else {
            eprintln!(r"Warning: Could not read file for bag of words analysis: {}", path.display());
        }
    }

    println!(r"Processed {} files for bag of words analysis.", files_processed_for_bow);
    println!(r"Top 20 most common terms:");

    let mut sorted_terms: Vec<(&String, &usize)> = bag_of_words_visitor.bag_of_words.iter().collect();
    sorted_terms.sort_by(|a, b| b.1.cmp(a.1));

    for (term, count) in sorted_terms.iter().take(20) {
        println!(r"  - {}: {}", term, count);
    }
    Ok(())
}

pub async fn handle_calculate_layers(project_root: &PathBuf, args: &crate::Args) -> anyhow::Result<()> {
    println!("Calculating type layers...");
    let type_map = type_extractor::extract_bag_of_types(project_root, &args.filter_names).await?;

    println!("\n--- Type Layer Analysis ---");
    println!("{:<30} {:<10} {:<10}", "Type", "Count", "Layer");
    println!("---------------------------------------------------");

    let mut sorted_types: Vec<(&String, &type_extractor::TypeInfo)> = type_map.iter().collect();
    sorted_types.sort_by(|a, b| b.1.count.cmp(&a.1.count));

    for (type_name, info) in sorted_types.iter() {
        println!("{:<30} {:<10} {:<10?}", type_name, info.count, info.layer);
    }
    println!("---------------------------------------------------");

    Ok(())
}
