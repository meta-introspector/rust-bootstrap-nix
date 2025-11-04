use prelude_generator::command_handlers;
use std::path::PathBuf;
use syn;
use tokio;
use prelude_generator::use_extractor::get_rustc_info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (args, config) = prelude_generator::cli::parse_arguments_and_config()?;

    let project_root = if args.path == PathBuf::from(".") {
        std::env::current_dir()?
    } else {
        args.path.clone()
    };

    let rustc_info = get_rustc_info()?;
    let cache_dir = project_root.join(".prelude_cache");
    tokio::fs::create_dir_all(&cache_dir).await?;

    let mut all_numerical_constants: Vec<syn::ItemConst> = Vec::new();
    let mut all_string_constants: Vec<syn::ItemConst> = Vec::new();

    if args.analyze_ast {
        crate::command_handlers::handle_analyze_ast(&args)?;
        return Ok(()); // Exit after AST analysis if requested
    }

    if args.generate_test_report {
        crate::command_handlers::handle_generate_test_report(&args)?;
    }

    if args.compile_tests {
        crate::command_handlers::handle_compile_tests(&args)?;
    }

    if args.extract_use_statements {
        crate::command_handlers::handle_extract_use_statements(&args)?;
    }

    if args.collect_and_process_use_statements {
        crate::command_handlers::handle_collect_and_process_use_statements();
    }

    if args.generate_aggregated_test_file {
        crate::command_handlers::handle_generate_aggregated_test_file();
    }

    if args.run_pipeline {
        crate::command_handlers::handle_run_pipeline(&args, config.as_ref().unwrap()).await?;
    }

    if args.verify_config {
        crate::command_handlers::handle_verify_config();
    }

    if args.extract_global_level0_decls {
        crate::command_handlers::handle_extract_global_level0_decls(
            &project_root,
            &args,
            &mut all_numerical_constants,
            &mut all_string_constants,
            &rustc_info,
            &cache_dir,
        ).await?;
    }

    // Process numerical constants
    if args.extract_numerical_constants {
        crate::command_handlers::handle_extract_numerical_constants(&project_root, &args, &all_numerical_constants).await?;
    }

    // Process string constants
    if args.extract_string_constants {
        crate::command_handlers::handle_extract_string_constants(&project_root, &args, &all_string_constants).await?;
    }

    if args.analyze_bag_of_words {
        crate::command_handlers::handle_analyze_bag_of_words(&project_root, &args)?;
    }

    if args.calculate_layers {
        crate::command_handlers::handle_calculate_layers(&project_root, &args).await?;
    }

    // New conditional block for split-expanded-bin functionality
    if args.run_split_expanded_bin {
        crate::command_handlers::handle_split_expanded_bin(&args).await?;
    }

    // If no specific command was executed, print help or a default message
    if !args.analyze_ast && !args.generate_test_report && !args.compile_tests && !args.extract_use_statements && !args.collect_and_process_use_statements && !args.generate_aggregated_test_file && !args.run_pipeline && !args.verify_config && !args.extract_global_level0_decls && !args.analyze_bag_of_words && !args.extract_numerical_constants && !args.extract_string_constants && !args.calculate_layers && !args.run_split_expanded_bin {
        println!(r"No specific command executed. Use --help for options.");
    }

    Ok(())
}
