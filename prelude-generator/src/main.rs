use prelude_generator::args::Args;
use clap::Parser;

use prelude_generator::command_handlers;
use prelude_generator::type_usage_analyzer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Before Args::parse()");
    let args = Args::parse();
    let mut warnings: Vec<String> = Vec::new();

    if args.run_decl_splitter {
        let project_root = args.path.clone();
        let rustc_info = prelude_generator::use_extractor::rustc_info::get_rustc_info()?;
        command_handlers::handle_run_decl_splitter(&args, &project_root, &rustc_info, &mut warnings).await?;
    } else if args.analyze_type_usage {
        type_usage_analyzer::analyze_type_usage(&args).await?;
    } else if args.extract_global_level0_decls {
        let project_root = args.path.clone();
        let rustc_info = prelude_generator::use_extractor::rustc_info::get_rustc_info()?;
        let mut all_numerical_constants = Vec::new();
        let mut all_string_constants = Vec::new();
        let cache_dir = args.cache_dir.clone().unwrap_or_else(|| project_root.join(".prelude_cache"));
        let crate_name = args.crate_name.clone().unwrap_or_else(|| "unknown_crate".to_string());

        command_handlers::handle_extract_global_level0_decls(
            &project_root,
            &args,
            &mut all_numerical_constants,
            &mut all_string_constants,
            &rustc_info,
            &cache_dir,
            &crate_name,
            &mut warnings,
        ).await?;

        command_handlers::handle_extract_numerical_constants(&project_root, &args, &all_numerical_constants).await?;
        command_handlers::handle_extract_string_constants(&project_root, &args, &all_string_constants).await?;
    } else if args.split_expanded_bin {
        command_handlers::handle_split_expanded_bin(&args, &mut warnings).await?;
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
