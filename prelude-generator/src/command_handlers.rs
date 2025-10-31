use anyhow::Context;
use std::path::PathBuf;
use syn::{self, visit::Visit};
use crate::{declaration_processing, constant_storage, BagOfWordsVisitor, config_parser, pipeline, type_extractor, error_collector::ErrorCollection};
use std::collections::HashMap;

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

use crate::gem_parser::GemConfig;

use cargo_metadata::{MetadataCommand, Package};
//use walkdir::WalkDir;

pub async fn handle_extract_global_level0_decls(
    project_root: &PathBuf,
    args: &crate::Args,
    all_numerical_constants: &mut Vec<syn::ItemConst>,
    all_string_constants: &mut Vec<syn::ItemConst>,
    rustc_info: &crate::use_extractor::rustc_info::RustcInfo,
    cache_dir: &std::path::Path,
) -> anyhow::Result<()> {
    println!("Extracting global Level 0 declarations...");
    println!("Project root: {}", project_root.display());
    let generated_decls_output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });
    println!("Generated decls output dir: {}", generated_decls_output_dir.display());

    let gem_config_path = project_root.join("gems.toml");
    let gem_config = GemConfig::load_from_file(&gem_config_path)
        .context(format!("Failed to load gem config from {}", gem_config_path.display()))?;

    let type_map = type_extractor::extract_bag_of_types(project_root, &args.filter_names).await?;

    let metadata = MetadataCommand::new()
        .manifest_path(project_root.join("Cargo.toml"))
        .exec()
        .context("Failed to execute cargo metadata")?;

    let mut all_declarations: Vec<crate::declaration::Declaration> = Vec::new();
    let mut total_files_processed = 0;
    let mut collected_errors = Vec::new();
    let mut total_fns = 0;
    let mut total_structs = 0;
    let mut total_enums = 0;
    let mut total_statics = 0;
    let mut total_other_items = 0;
    let total_structs_per_layer: HashMap<usize, usize> = HashMap::new();

    // Collect all packages to process (workspace members + local path dependencies)
    let mut packages_to_process: Vec<&Package> = Vec::new();

    // Add workspace members
    for member_id in &metadata.workspace_members {
        if let Some(pkg) = metadata.packages.iter().find(|p| &p.id == member_id) {
            packages_to_process.push(pkg);
        }
    }

    // Add local path dependencies that are not workspace members
    for pkg in &metadata.packages {
        if pkg.manifest_path.starts_with(project_root) && !metadata.workspace_members.contains(&pkg.id) {
            // Check if it's a local path dependency and not already a workspace member
            // This heuristic might need refinement for complex dependency graphs
            packages_to_process.push(pkg);
        }
    }

    for pkg in packages_to_process {
        let manifest_path = pkg.manifest_path.to_path_buf();
        println!("Processing crate: {} at {}", pkg.name, manifest_path);

        let (decls, files_processed, fns, structs, enums, statics, other_items, _structs_per_layer, errors) =
            declaration_processing::extract_all_declarations_from_crate(
                manifest_path.as_ref(),
                &args,
                &type_map,
                &args.filter_names,
                rustc_info,
                cache_dir,
                &gem_config,
            ).await?;

        all_declarations.extend(decls);
        collected_errors.extend(errors);
        total_files_processed += files_processed;
        total_fns += fns;
        total_structs += structs;
        total_enums += enums;
        total_statics += statics;
        total_other_items += other_items;
        // Merge structs_per_layer if needed, or re-calculate from all_declarations later
    }

    let mut error_collection = ErrorCollection::default();
    for err_sample in collected_errors {
        error_collection.add_error(err_sample);
    }

    // Layer the declarations
    let layered_declarations = declaration_processing::layer_declarations(all_declarations);

    println!("\n--- Layered Declaration Analysis ---");
    for layer_num in 0..=8 { // Iterate up to 8 layers as per requirement
        if let Some(decls_in_layer) = layered_declarations.get(&layer_num) {
            println!("Layer {}: {} declarations", layer_num, decls_in_layer.len());
            // For now, just print the count. Further processing can be added here.
        } else if layer_num == 0 && layered_declarations.get(&0).is_none() {
            println!("Layer 0: No declarations found.");
        } else if layered_declarations.get(&layer_num).is_none() && layer_num > 0 {
            println!("Layer {}: No declarations found.", layer_num);
            // If a layer is empty and it's not layer 0, we can stop if no more layers are expected
            if layered_declarations.keys().all(|&k| k < layer_num) {
                break;
            }
        }
    }
    println!("-------------------------------------");

    // Separate constants and structs from all_declarations for further processing
    // Note: This part might need adjustment if constants/structs are now processed per layer.
    let mut constants: Vec<syn::ItemConst> = Vec::new();
    let mut structs: HashMap<usize, Vec<syn::ItemStruct>> = HashMap::new();
    // Re-collect constants and structs from layered_declarations if needed for process_constants
    for (_layer_num, decls_in_layer) in layered_declarations.iter() {
        for decl in decls_in_layer {
            match &decl.item {
                crate::declaration::DeclarationItem::Const(c) => constants.push(c.clone()),
                crate::declaration::DeclarationItem::Struct(s) => {
                    let struct_name = s.ident.to_string();
                    let layer = type_map.get(&struct_name).and_then(|info| info.layer).unwrap_or(0);
                    structs.entry(layer).or_insert_with(Vec::new).push(s.clone());
                },
                _ => {},
            }
        }
    }

    if let Err(e) = declaration_processing::process_constants(
        constants.clone(),
        &args,
        &project_root,
        all_numerical_constants,
        all_string_constants,
        &type_map,
    ).await {
        // For now, still collect anyhow errors from process_constants
        eprintln!("Error processing constants: {:?}", e);
    }


    println!("Total files processed: {}", total_files_processed);
    println!("Total constants extracted: {}", constants.len());
    println!("Total functions found: {}", total_fns);
    println!("Total structs found: {}", total_structs);
    println!("Total structs extracted per layer: {:?}", total_structs_per_layer);
    println!("Total enums found: {}", total_enums);
    println!("Total statics found: {}", total_statics);
    println!("Total other items found: {}", total_other_items);
    println!("---------------------------------------------");

    if !error_collection.errors.is_empty() {
        eprintln!(r"\n--- Summary of Errors Encountered During Macro Expansion/Parsing ---");
        for error in &error_collection.errors {
            eprintln!(r"File: {}, Type: {}, Message: {}", error.file_path.display(), error.error_type, error.error_message);
        }
        eprintln!(r"---------------------------------------------------------------------");
        let error_output_path = project_root.join("collected_errors.json");
        error_collection.write_to_file(&error_output_path).await?;
        eprintln!("Collected errors written to: {}", error_output_path.display());
    }

    Ok(())
}

pub async fn handle_extract_numerical_constants(
    _project_root: &PathBuf,
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
