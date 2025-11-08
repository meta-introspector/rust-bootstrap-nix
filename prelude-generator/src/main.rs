use prelude_generator::args::Args;
use clap::Parser;

use prelude_generator::command_handlers;
use prelude_generator::type_usage_analyzer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Before Args::parse()");
    let args = Args::parse();
    let mut warnings: Vec<String> = Vec::new();

    let project_root = args.path.clone();
    let config = if let Some(config_path) = &args.config_file_path {
        Some(prelude_generator::config_parser::read_config(config_path, &project_root)?)
    } else {
        None
    };

    let mut args_with_config = args.clone();
    if args_with_config.generated_decls_output_dir.is_none() {
        if let Some(cfg) = &config {
            if let Some(generated_output_dir) = &cfg.generated_output_dir {
                args_with_config.generated_decls_output_dir = Some(generated_output_dir.clone());
            }
        }
    }
    // Ensure cache_dir and crate_name are also propagated if not set by CLI
    if args_with_config.cache_dir.is_none() {
        args_with_config.cache_dir = Some(project_root.join(".prelude_cache"));
    }
    if args_with_config.crate_name.is_none() {
        args_with_config.crate_name = Some("unknown_crate".to_string());
    }


    if args_with_config.run_decl_splitter {
        let rustc_info = prelude_generator::use_extractor::rustc_info::get_rustc_info()?;
        command_handlers::handle_run_decl_splitter(&args_with_config, &project_root, &rustc_info, &mut warnings).await?;
    } else if args_with_config.analyze_type_usage {
        type_usage_analyzer::analyze_type_usage(&args_with_config).await?;
    } else if args_with_config.extract_global_level0_decls {
        let rustc_info = prelude_generator::use_extractor::rustc_info::get_rustc_info()?;
        let mut all_numerical_constants = Vec::new();
        let mut all_string_constants = Vec::new();
        let cache_dir = args_with_config.cache_dir.clone().unwrap(); // Now guaranteed to be Some
        let crate_name = args_with_config.crate_name.clone().unwrap(); // Now guaranteed to be Some

        command_handlers::handle_extract_global_level0_decls(
            &project_root,
            &args_with_config,
            &mut all_numerical_constants,
            &mut all_string_constants,
            &rustc_info,
            &cache_dir,
            &crate_name,
            &mut warnings,
        ).await?;

        command_handlers::handle_extract_numerical_constants(&project_root, &args_with_config, &all_numerical_constants).await?;
        command_handlers::handle_extract_string_constants(&project_root, &args_with_config, &all_string_constants).await?;
    } else if args_with_config.split_expanded_bin {
        let rustc_info = prelude_generator::use_extractor::rustc_info::get_rustc_info()?;
        let main_project_root = std::env::current_dir()?.parent().unwrap().to_path_buf();
        let canonical_output_root = main_project_root.join("generated");
        tokio::fs::create_dir_all(&canonical_output_root)
            .await
            .context(format!("Failed to create canonical output root directory: {}", canonical_output_root.display()))?;

        let inputs = prelude_generator::types::SplitExpandedBinInputs {
            files_to_process: args_with_config.split_expanded_files.clone(),
            project_root: args_with_config.split_expanded_project_root.clone().unwrap_or_else(|| PathBuf::from("generated_workspace")),
            rustc_version: rustc_info.version.clone(),
            rustc_host: rustc_info.host.clone(),
            verbose: args_with_config.verbose,
            output_global_toml: args_with_config.split_expanded_output_global_toml.clone(),
            output_symbol_map: args_with_config.output_symbol_map.clone(),
            warnings: &mut warnings,
            canonical_output_root: &canonical_output_root,
        };
        command_handlers::handle_split_expanded_bin(inputs).await?;
    }
    else {
        // Default behavior or error if no specific command flag is set
        println!("No specific command flag set. Use --help for options.");
    }

    if !warnings.is_empty() {
        eprintln!("\n--- Warnings ---");
        for warning in warnings {
            eprintln!("{}", warning);
        }
        eprintln!("----------------");
    }

    Ok(())
}
