use prelude_generator::args::Args;
use clap::Parser;
use std::path::PathBuf;
use prelude_generator::command_handlers::handle_run_decl_splitter;
use prelude_generator::command_handlers;
//use command_handlers::{self, handle_run_decl_splitter};
use prelude_generator::type_usage_analyzer;
use prelude_generator::config_parser;
use prelude_generator::use_extractor::rustc_info;
use prelude_generator::cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Before Args::parse()");
    let (args, config_option) = cli::parse_arguments_and_config().await?;

    let config = config_option.ok_or_else(|| anyhow::anyhow!("Configuration not loaded."))?;

    if args.plan {
        command_handlers::handle_plan_mode(&args, &config).await?;
    } else if let Some(task_id) = args.commit {
        command_handlers::handle_commit_task(&args, &task_id).await?;
    } else if args.run_decl_splitter {
        let project_root = args.path.clone();
        let rustc_info = rustc_info::get_rustc_info()?;
        handle_run_decl_splitter(&args, &project_root, &rustc_info).await?;
    } else if args.analyze_type_usage {
        type_usage_analyzer::analyze_type_usage(&args).await?;
    } else {
        // Default behavior or error if no specific command flag is set
        println!("No specific command flag set. Use --help for options.");
    }

    Ok(())
}
