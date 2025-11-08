use crate::type_extractor;
use crate::BagOfWordsVisitor;
use crate::config_parser::Config;
use anyhow::{Context, Result};
use std::path::{PathBuf, Path};
use syn::visit::Visit;
use crate::args::Args;
use crate::pipeline;
use crate::constant_storage::numerical_constants::write_numerical_constants_to_hierarchical_structure;
pub mod decl_splitter_handler;
pub use decl_splitter_handler::handle_run_decl_splitter;

pub async fn handle_plan_mode(_args: &Args, _config: &Config) -> Result<()> {
    println!("Planning mode activated. This will list available tasks and their estimated sizes.");
    // TODO: Implement task discovery and size estimation

    // let generated_exports = crate::module_exporter::generate_module_exports(config);
    // println!("\n--- Generated Module Exports ---");
    // println!("{}", generated_exports);
    // println!("--------------------------------");

    Ok(())
}

pub async fn handle_commit_task(_args: &Args, task_id: &str) -> Result<()> {
    println!("Committing task: {}", task_id);
    // TODO: Implement task execution based on task_id
    Ok(())
}

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

pub async fn handle_run_pipeline(args: &crate::Args, config: &Config) -> anyhow::Result<()> {
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
    project_root: &PathBuf,
    args: &crate::Args,
    all_numerical_constants: &mut Vec<syn::ItemConst>,
    all_string_constants: &mut Vec<syn::ItemConst>,
    rustc_info: &crate::use_extractor::rustc_info::RustcInfo,
    _cache_dir: &std::path::Path,
    crate_name: &str,
    warnings: &mut Vec<String>,
    canonical_output_root: &Path,
) -> anyhow::Result<()> {
    let output_dir = args.generated_decls_output_dir.clone().ok_or_else(|| anyhow::anyhow!("generated_decls_output_dir is required"))?;
    tokio::fs::create_dir_all(&output_dir).await.context("Failed to create output directory")?;

    let mut all_public_symbols: Vec<split_expanded_lib::PublicSymbol> = Vec::new();
    let mut all_collected_errors: Vec<split_expanded_lib::ErrorSample> = Vec::new();

    let rustc_info_for_split_expanded_lib = split_expanded_lib::RustcInfo {
        version: rustc_info.version.clone(),
        host: rustc_info.host.clone(),
    };

    let walker = walkdir::WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"));

    for entry in walker {
        let file_path = entry.path().to_path_buf();


        let should_process_file = args.filter_names.as_ref().map_or(true, |filter_names| {
            filter_names.iter().any(|f| file_path.to_string_lossy().contains(f))
        });

        if should_process_file {
            let extraction_result = split_expanded_lib::processing::extract_declarations_from_single_file(
                &file_path,
                &rustc_info_for_split_expanded_lib,
                &crate_name,
                args.verbose,
                warnings,
                canonical_output_root,
            ).await?;

            let declarations = extraction_result.declarations;
            let errors = extraction_result.errors;
            let public_symbols = extraction_result.public_symbols;

            all_collected_errors.extend(errors);
            all_public_symbols.extend(public_symbols);

            for (_identifier, decl) in declarations {
                match &decl.item {
                    split_expanded_lib::DeclarationItem::Const(s) => {
                        if let Ok(item_const) = syn::parse_str::<syn::ItemConst>(&s) {
                            if item_const.ident.to_string().ends_with("_NUM") {
                                all_numerical_constants.push(item_const);
                            } else {
                                all_string_constants.push(item_const);
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    // Save public symbols to a JSON file
    let public_symbols_output_path = output_dir.join("public_symbols.json");
    let json_content = serde_json::to_string_pretty(&all_public_symbols)
        .context("Failed to serialize public symbols to JSON")?;
    tokio::fs::write(&public_symbols_output_path, json_content).await
        .context(format!("Failed to write public symbols to file: {:?}", public_symbols_output_path))?;

    println!("Extracted {} public symbols to {:?}", all_public_symbols.len(), public_symbols_output_path);

    // Handle errors if any
    if !all_collected_errors.is_empty() {
        let error_output_path = output_dir.join("errors.json");
        let error_json_content = serde_json::to_string_pretty(&all_collected_errors)
            .context("Failed to serialize errors to JSON")?;
        tokio::fs::write(&error_output_path, error_json_content).await
            .context(format!("Failed to write errors to file: {:?}", error_output_path))?;
        eprintln!("{} errors collected during declaration extraction. See {:?}", all_collected_errors.len(), error_output_path);
    }

    Ok(())
}

pub async fn handle_extract_numerical_constants(    _project_root: &PathBuf,
    _args: &crate::Args,
    all_numerical_constants: &Vec<syn::ItemConst>,
) -> anyhow::Result<()> {
    let numerical_output_dir = _project_root.join(r"generated/numerical_constants");
    tokio::fs::create_dir_all(&numerical_output_dir).await
        .context(format!("Failed to create output directory {:?}", numerical_output_dir))?;
    write_numerical_constants_to_hierarchical_structure(&all_numerical_constants, &numerical_output_dir).await?;
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
    crate::constant_storage::string_constants::write_string_constants_to_hierarchical_structure(&all_string_constants, &string_output_dir).await?;
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
        PathBuf::from(&args.path)
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
