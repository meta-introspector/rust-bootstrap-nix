use prelude_generator::args::Args;
use clap::Parser;
use std::path::PathBuf;

use prelude_generator::command_handlers;
use prelude_generator::type_usage_analyzer;
use prelude_generator::use_extractor::rustc_info::RustcInfo;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Before Args::parse()");
    let args = Args::parse();

    if args.run_decl_splitter {
        let project_root = args.path.clone();
        let rustc_info = prelude_generator::use_extractor::rustc_info::get_rustc_info()?;
        command_handlers::handle_run_decl_splitter(&args, &project_root, &rustc_info).await?;
    } else if args.analyze_type_usage {
        type_usage_analyzer::analyze_type_usage(&args).await?;
    } else {
        // Default behavior or error if no specific command flag is set
        println!("No specific command flag set. Use --help for options.");
    }

    Ok(())
}
