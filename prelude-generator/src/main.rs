use prelude_generator::{Level0DeclsVisitor, BagOfWordsVisitor};
use quote::quote;
use syn::visit::Visit;
use syn;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::fs;
use std::io::Write;
use crate::constant_storage;

// mod hf_dataset_reader;
mod config_parser;
mod parser;

async fn run_category_pipeline<W: tokio::io::AsyncWriteExt + Unpin + Send>(
    writer: &mut W,
    file_content: &str,
    file_path_str: &str,
    _args: &Args,
    _config: &Option<config_parser::Config>,
) -> anyhow::Result<()> {
    let raw_file = RawFile(file_path_str.to_string(), file_content.to_string());

    writer.write_all(format!("--- Stage 1: Parsing ---\n").as_bytes()).await?;
    let parse_functor = ParseFunctor;
    let parsed_file = parse_functor.map(writer, raw_file).await.context("Parsing failed")?;
    writer.write_all(format!("  -> Parsed file successfully.\n").as_bytes()).await?;

    writer.write_all(format!("--- Stage 2: Extracting Use Statements ---\n").as_bytes()).await?;
    let extract_uses_functor = ExtractUsesFunctor;
    let use_statements = extract_uses_functor.map(writer, parsed_file.clone()).await.context("Extracting use statements failed")?;
    writer.write_all(format!("  -> Extracted {} use statements.\n", use_statements.0.len()).as_bytes()).await?;

    writer.write_all(format!("--- Stage 3: Classifying Use Statements ---\n").as_bytes()).await?;
    let classify_uses_functor = ClassifyUsesFunctor;
    let classified_uses = classify_uses_functor.map(writer, use_statements).await.context("Classifying use statements failed")?;
    writer.write_all(format!("  -> Classified use statements:\n").as_bytes()).await?;
    writer.write_all(format!("{:#?}\n", classified_uses).as_bytes()).await?;

    writer.write_all(format!("--- Stage 4: AST Reconstruction from Hugging Face Dataset ---\n").as_bytes()).await?;
    let ast_reconstruction_functor = AstReconstructionFunctor;
    let validated_file = ValidatedFile(parsed_file.0.clone(), parsed_file.1.clone());
    let reconstructed_code = PipelineFunctor::map(&ast_reconstruction_functor, writer, validated_file.clone()).await.context("AST Reconstruction failed")?;
    writer.write_all(format!("  -> AST Reconstruction completed successfully.\n").as_bytes()).await?;

    // Write generated code to a file
    let output_file_path = PathBuf::from("generated/self_generated_code.rs"); // Define output path
    tokio::fs::create_dir_all(output_file_path.parent().unwrap()).await?;
    tokio::fs::write(&output_file_path, reconstructed_code.as_bytes()).await
        .context(format!("Failed to write generated code to {:?}", output_file_path))?;
    writer.write_all(format!("  -> Generated code written to {:?}\n", output_file_path).as_bytes()).await?;

    // Validate the generated code
    writer.write_all(format!("--- Validating Generated Code ---\n").as_bytes()).await?;
    validate_rust_code(&output_file_path).await
        .context(format!("Generated code validation failed for {:?}", output_file_path))?;
    writer.write_all(format!("  -> Generated code validated successfully.\n").as_bytes()).await?;

    let collected_metrics = measurement::get_collected_metrics();
    let json_metrics = serde_json::to_string_pretty(&collected_metrics).context("Failed to serialize metrics to JSON")?;
    writer.write_all(format!("--- METRICS_START ---\n{}\n--- METRICS_END ---\n", json_metrics).as_bytes()).await?;

    Ok(())
}

async fn format_rust_code(file_path: &PathBuf) -> anyhow::Result<()> {
    use tokio::process::Command;

    let output = Command::new("rustfmt")
        .arg(file_path)
        .arg("--edition=2021") // Specify the Rust edition
        .arg("--emit=files") // Emit changes directly to the file
        .output()
        .await
        .context("Failed to execute rustfmt")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rustfmt failed for file {:?}:\n{}", file_path, stderr);
    }

    Ok(())
}

async fn validate_rust_code(file_path: &PathBuf) -> anyhow::Result<()> {
    use tokio::process::Command;

    let output = Command::new("rustc")
        .arg("--emit=metadata") // Only check for errors, don't produce artifacts
        .arg("--crate-type=lib") // Treat as a library crate
        .arg(file_path)
        .output()
        .await
        .context("Failed to execute rustc")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rustc check failed for file {:?}:\n{}", file_path, stderr);
    }

    Ok(())
}

fn parse_arguments_and_config() -> anyhow::Result<(Args, Option<Config>)> {

    let args = Args::parse();

    // Determine the project root. If args.path is ".", resolve it to the actual current directory.
    // Then, find the parent directory that contains the Cargo.toml for the workspace.
    // For now, we'll assume args.path is the project root if it's explicitly set,
    // otherwise, we'll try to find the workspace root from the current executable's directory.
    let project_root = if args.path == PathBuf::from(".") {
        // If path is ".", it means the current directory of the prelude-generator executable.
        // The actual project root is its parent.
        std::env::current_dir()?.parent().unwrap().to_path_buf()
    } else {
        args.path.clone()
    };


    let config = if let Some(config_path) = &args.config_file_path {
        Some(config_parser::read_config(config_path, &project_root)?)
    } else {
        // If config_file_path is not provided, try to read from the default location
        let default_config_path = project_root.join("config.toml");
        if default_config_path.exists() {
            Some(config_parser::read_config(&default_config_path, &project_root)?)
        } else {
            None
        }
    };
    Ok((args, config))
}

fn get_required_uses(decl_name: &str) -> String {
    match decl_name {
        "Args" => "use clap::{Parser, Args, Command};\nuse std::path::PathBuf;\n".to_string(),
        "TestInfo" => "use serde::{Serialize, Deserialize};\nuse std::path::PathBuf;\n".to_string(),
        "BinsConfig" => "use serde::Deserialize;\nuse std::collections::HashMap;\nuse std::path::PathBuf;\n".to_string(),
        "PipelineState" | "StageSummary" | "UseStatement" => "use serde::{Serialize, Deserialize};\n".to_string(),
        "Level0DeclsVisitor" => "use syn::{ItemConst, ItemStruct};\nuse syn::visit::Visit;\n".to_string(),
        _ => "".to_string(),
    }
}

fn get_required_uses_for_item_const(_constant: &syn::ItemConst) -> String {
    // For Level 0 constants, typically no special uses are needed unless they use complex types.
    // For now, return empty string.
    "".to_string()
}

fn get_required_uses_for_item_struct(structure: &syn::ItemStruct) -> String {
    let mut uses = Vec::new();

    // Check for clap attributes
    if structure.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                meta_list.tokens.to_string().contains("Parser")
            } else { false }
        } else { false }
    }) {
        uses.push("use clap::{Parser, Args, Command};\n");
        uses.push("use std::path::PathBuf;\n"); // Args often uses PathBuf
    }

    // Check for serde attributes
    if structure.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                meta_list.tokens.to_string().contains("Serialize") || meta_list.tokens.to_string().contains("Deserialize")
            } else { false }
        } else { false }
    }) {
        uses.push("use serde::{Serialize, Deserialize};\n");
    }

    // Check for HashMap (used in BinsConfig)
    if structure.ident.to_string() == "BinsConfig" {
        uses.push("use std::collections::HashMap;\n");
        uses.push("use std::path::PathBuf;\n");
    }

    // Check for PathBuf (used in TestInfo)
    if structure.ident.to_string() == "TestInfo" {
        uses.push("use std::path::PathBuf;\n");
    }

    // Check for Level0DeclsVisitor specific uses
    if structure.ident.to_string() == "Level0DeclsVisitor" {
        uses.push("use syn::{ItemConst, ItemStruct};\n");
        uses.push("use syn::visit::Visit;\n");
    }

    uses.join("")
}

fn parse_arguments_and_config() -> anyhow::Result<(Args, Option<Config>)> {

    let args = Args::parse();

    // Determine the project root. If args.path is ".", resolve it to the actual current directory.
    // Then, find the parent directory that contains the Cargo.toml for the workspace.
    // For now, we'll assume args.path is the project root if it's explicitly set,
    // otherwise, we'll try to find the workspace root from the current executable's directory.
    let project_root = if args.path == PathBuf::from(".") {
        // If path is ".", it means the current directory of the prelude-generator executable.
        // The actual project root is its parent.
        std::env::current_dir()?.parent().unwrap().to_path_buf()
    } else {
        args.path.clone()
    };


    let config = if let Some(config_path) = &args.config_file_path {
        Some(config_parser::read_config(config_path, &project_root)?)
    } else {
        // If config_file_path is not provided, try to read from the default location
        let default_config_path = project_root.join("config.toml");
        if default_config_path.exists() {
            Some(config_parser::read_config(&default_config_path, &project_root)?)
        } else {
            None
        }
    };
    Ok((args, config))
}

fn get_required_uses(decl_name: &str) -> String {
    match decl_name {
        "Args" => "use clap::{Parser, Args, Command};\nuse std::path::PathBuf;\n".to_string(),
        "TestInfo" => "use serde::{Serialize, Deserialize};\nuse std::path::PathBuf;\n".to_string(),
        "BinsConfig" => "use serde::Deserialize;\nuse std::collections::HashMap;\nuse std::path::PathBuf;\n".to_string(),
        "PipelineState" | "StageSummary" | "UseStatement" => "use serde::{Serialize, Deserialize};\n".to_string(),
        "Level0DeclsVisitor" => "use syn::{ItemConst, ItemStruct};\nuse syn::visit::Visit;\n".to_string(),
        _ => "".to_string(),
    }
}

fn get_required_uses_for_item_const(_constant: &syn::ItemConst) -> String {
    // For Level 0 constants, typically no special uses are needed unless they use complex types.
    // For now, return empty string.
    "".to_string()
}

fn get_required_uses_for_item_struct(structure: &syn::ItemStruct) -> String {
    let mut uses = Vec::new();

    // Check for clap attributes
    if structure.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                meta_list.tokens.to_string().contains("Parser")
            } else { false }
        } else { false }
    }) {
        uses.push("use clap::{Parser, Args, Command};\n");
        uses.push("use std::path::PathBuf;\n"); // Args often uses PathBuf
    }

    // Check for serde attributes
    if structure.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                meta_list.tokens.to_string().contains("Serialize") || meta_list.tokens.to_string().contains("Deserialize")
            } else { false }
        } else { false }
    }) {
        uses.push("use serde::{Serialize, Deserialize};\n");
    }

    // Check for HashMap (used in BinsConfig)
    if structure.ident.to_string() == "BinsConfig" {
        uses.push("use std::collections::HashMap;\n");
        uses.push("use std::path::PathBuf;\n");
    }

    // Check for PathBuf (used in TestInfo)
    if structure.ident.to_string() == "TestInfo" {
        uses.push("use std::path::PathBuf;\n");
    }

    // Check for Level0DeclsVisitor specific uses
    if structure.ident.to_string() == "Level0DeclsVisitor" {
        uses.push("use syn::{ItemConst, ItemStruct};\n");
        uses.push("use syn::visit::Visit;\n");
    }

    uses.join("")
}

const MAX_FILE_SIZE: usize = 4 * 1024; // 4KB

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (args, config) = parse_arguments_and_config()?;

    if args.analyze_ast {
        let path = args.ast_analysis_path.ok_or_else(|| anyhow::anyhow!("ast_analysis_path is required when analyze_ast is true"))?;
        println!("Analyzing AST for project: {}", path.display());
        return Ok(()); // Exit after AST analysis if requested
    }

    if args.generate_test_report {
        let output_file = args.test_report_output_file.clone().unwrap_or_else(|| PathBuf::from("test_report.json"));
        // generate_test_report_json(&args.path)?;
    }

    if args.compile_tests {
        let input_file = args.test_report_input_file.clone().ok_or_else(|| anyhow::anyhow!("test_report_input_file is required when compile_tests is true"))?;
        let output_dir = args.test_verification_output_dir.clone().ok_or_else(|| anyhow::anyhow!("test_verification_output_dir is required when compile_tests is true"))?;
        // generate_test_verification_script_and_report(&input_file)?;
    }

    if args.extract_use_statements {
        let output_dir = args.use_statements_output_dir.clone().ok_or_else(|| anyhow::anyhow!("use_statements_output_dir is required when extract_use_statements is true"))?;
        // TODO: Implement actual use statement extraction logic here
        println!("Extracting use statements to: {}", output_dir.display());
    }

    if args.collect_and_process_use_statements {
        // TODO: Implement logic for collecting and processing use statements
        println!("Collecting and processing use statements...");
    }

    if args.generate_aggregated_test_file {
        // TODO: Implement logic for generating aggregated test file
        println!("Generating aggregated test file...");
    }

    if args.run_pipeline {
        println!("Running main pipeline...");
        let mut stdout = tokio::io::stdout();
        let dummy_content = "fn main() { println!(\"Hello, world!\"); }".to_string();
        let dummy_path = "dummy_file.rs".to_string();

        run_category_pipeline(
            &mut stdout,
            &dummy_content,
            &dummy_path,
            &args,
            &config,
        ).await?;
    }

    if args.verify_config {
        // TODO: Implement config verification logic
        println!("Verifying configuration...");
    }

    if args.extract_global_level0_decls {
        println!("Extracting global Level 0 declarations...");
        let mut all_constants: Vec<syn::ItemConst> = Vec::new();
        let mut all_layer0_structs: Vec<syn::ItemStruct> = Vec::new();
        let mut total_files_processed = 0;
        let mut total_fns = 0;
        let mut total_structs = 0;
        let mut total_enums = 0;
        let mut total_statics = 0;
        let mut total_other_items = 0;
        let mut total_layer0_structs = 0;

        let project_root = if args.path == PathBuf::from(".") {
            std::env::current_dir()?.parent().unwrap().to_path_buf()
        } else {
            args.path.clone()
        };
        let src_dir = project_root.join("prelude-generator/src");

        for entry in std::fs::read_dir(&src_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                total_files_processed += 1;
                println!("  Processing file: {}", path.display());
                let content = std::fs::read_to_string(&path)?;
                let file = syn::parse_file(&content)?;

                let mut visitor = Level0DeclsVisitor::new();
                visitor.visit_file(&file);
                let current_layer0_structs_len = visitor.layer0_structs.len();
                all_constants.extend(visitor.constants);
                all_layer0_structs.extend(visitor.layer0_structs);

                total_fns += visitor.fn_count;
                total_structs += visitor.struct_count;
                total_enums += visitor.enum_count;
                total_statics += visitor.static_count;
                total_other_items += visitor.other_item_count;
                total_layer0_structs += current_layer0_structs_len;
            }
        }

        let generated_decls_output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| {
            project_root.join("generated/level0_decls")
        });

        let consts_output_dir = generated_decls_output_dir.join("const");
        let structs_output_dir = generated_decls_output_dir.join("struct");

        tokio::fs::create_dir_all(&consts_output_dir).await
            .context(format!("Failed to create output directory {:?}", consts_output_dir))?;
        tokio::fs::create_dir_all(&structs_output_dir).await
            .context(format!("Failed to create output directory {:?}", structs_output_dir))?;

        println!("  -> Generated constants will be written to: {:?}", consts_output_dir);
        println!("  -> Generated structs will be written to: {:?}", structs_output_dir);

        let mut errors: Vec<anyhow::Error> = Vec::new();

        // Process constants
        for constant in &all_constants {
            let const_name = constant.ident.to_string();
            let file_name = format!("{}.rs", const_name);
            let output_path = consts_output_dir.join(&file_name);

            let result = async {
                let tokens = quote! { #constant };
                let mut code = tokens.to_string();

                let required_uses = get_required_uses_for_item_const(&constant);
                code = format!("{}{}", required_uses, code);

                tokio::fs::write(&output_path, code.as_bytes()).await
                    .context(format!("Failed to write constant {:?} to {:?}", const_name, output_path))?;
                println!("  -> Wrote constant {:?} to {:?}", const_name, output_path);

                // Format the generated code
                format_rust_code(&output_path).await
                    .context(format!("Constant {:?} formatting failed for {:?}", const_name, output_path))?;
                println!("  -> Constant {:?} formatted successfully.", const_name);

                // Validate the generated code
                validate_rust_code(&output_path).await
                    .context(format!("Constant {:?} validation failed for {:?}", const_name, output_path))?;
                println!("  -> Constant {:?} validated successfully.\n", const_name);
                Ok(())
            }.await;

            if let Err(e) = result {
                eprintln!("Error processing constant {}: {:?}\n", const_name, e);
                errors.push(e);
            }
        }

        // Process structs
        for structure in &all_layer0_structs {
            let struct_name = structure.ident.to_string();
            let file_name = format!("{}.rs", struct_name);
            let output_path = structs_output_dir.join(&file_name);

            let result = async {
                let tokens = quote! { #structure };
                let mut code = tokens.to_string();

                let required_uses = get_required_uses_for_item_struct(&structure);
                code = format!("{}{}", required_uses, code);

                tokio::fs::write(&output_path, code.as_bytes()).await
                    .context(format!("Failed to write struct {:?} to {:?}", struct_name, output_path))?;
                println!("  -> Wrote struct {:?} to {:?}", struct_name, output_path);

                // Format the generated code
                format_rust_code(&output_path).await
                    .context(format!("Struct {:?} formatting failed for {:?}", struct_name, output_path))?;
                println!("  -> Struct {:?} formatted successfully.\n", struct_name);

                // Validate the generated code
                validate_rust_code(&output_path).await
                    .context(format!("Struct {:?} validation failed for {:?}", struct_name, output_path))?;
                println!("  -> Struct {:?} validated successfully.\n", struct_name);
                Ok(())
            }.await;

            if let Err(e) = result {
                eprintln!("Error processing struct {}: {:?}\n", struct_name, e);
                errors.push(e);
            }
        }

        println!("Total files processed: {}", total_files_processed);
        println!("Total constants extracted: {}", all_constants.len());
        println!("Total functions found: {}", total_fns);
        println!("Total structs found: {}", total_structs);
        println!("Total Layer 0 structs extracted: {}", all_layer0_structs.len());
        println!("Total enums found: {}", total_enums);
        println!("Total statics found: {}", total_statics);
        println!("Total other items found: {}", total_other_items);
        println!("---------------------------------------------");

        if !errors.is_empty() {
            eprintln!("\n--- Errors Encountered ---");
            for error in errors {
                eprintln!("{:?}", error);
            }
            eprintln!("--------------------------");
            println!("Declaration processing completed with errors.");
        } else {
            println!("Declaration processing completed successfully.");
        }
    }

    // Process numerical constants
    if args.extract_numerical_constants {
        let numerical_output_dir = project_root.join("generated/numerical_constants");
        tokio::fs::create_dir_all(&numerical_output_dir).await
            .context(format!("Failed to create output directory {:?}", numerical_output_dir))?;
                    constant_storage::numerical_constants::write_numerical_constants_to_hierarchical_structure(&all_numerical_constants, &numerical_output_dir).await?;        println!("  -> Numerical constants will be written to: {:?}", numerical_output_dir);
        println!("  -> Total numerical constants extracted: {}", all_numerical_constants.len());
    }

    // Process string constants
    if args.extract_string_constants {
        let string_output_dir = project_root.join("generated/string_constants");
        tokio::fs::create_dir_all(&string_output_dir).await
            .context(format!("Failed to create output directory {:?}", string_output_dir))?;
                    constant_storage::string_constants::write_string_constants_to_hierarchical_structure(&all_string_constants, &string_output_dir).await?;        println!("  -> String constants will be written to: {:?}", string_output_dir);
        println!("  -> Total string constants extracted: {}", all_string_constants.len());
    }

    if args.analyze_bag_of_words {
        println!("Analyzing bag of words...");
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
            .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let path = entry.path();
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(file) = syn::parse_file(&content) {
                    bag_of_words_visitor.visit_file(&file);
                    files_processed_for_bow += 1;
                } else {
                    eprintln!("Warning: Could not parse file for bag of words analysis: {}", path.display());
                }
            } else {
                eprintln!("Warning: Could not read file for bag of words analysis: {}", path.display());
            }
        }

        println!("Processed {} files for bag of words analysis.", files_processed_for_bow);
        println!("Top 20 most common terms:");

        let mut sorted_terms: Vec<(&String, &usize)> = bag_of_words_visitor.bag_of_words.iter().collect();
        sorted_terms.sort_by(|a, b| b.1.cmp(a.1));

        for (term, count) in sorted_terms.iter().take(20) {
            println!("  - {}: {}", term, count);
        }
    }

    // If no specific command was executed, print help or a default message
    if !args.analyze_ast && !args.generate_test_report && !args.compile_tests && !args.extract_use_statements && !args.collect_and_process_use_statements && !args.generate_aggregated_test_file && !args.run_pipeline && !args.verify_config && !args.extract_global_level0_decls && !args.analyze_bag_of_words && !args.extract_numerical_constants && !args.extract_string_constants {
        println!("No specific command executed. Use --help for options.");
    }

    Ok(())
}

async fn handle_pipeline_result(result: anyhow::Result<()>) -> anyhow::Result<()> {
    if let Err(ref e) = result {
        let mut stderr = tokio::io::stderr();
        stderr.write_all(format!("Pipeline failed: {:?}\n", e).as_bytes()).await?;
    } else {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(b"Pipeline completed successfully.\n").await?;
    }
    result
}

fn generate_ast_statistics_code(stats: &pipeline_traits::AstStatistics) -> String {
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

