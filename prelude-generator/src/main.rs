use prelude_generator::args::Args;
use clap::Parser;

use prelude_generator::command_handlers;
use prelude_generator::type_usage_analyzer;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Before Args::parse()");
    let args = Args::parse();

    if args.run_decl_splitter {
        command_handlers::handle_run_decl_splitter(&args).await?;
    } else if args.analyze_type_usage {
        type_usage_analyzer::analyze_type_usage(&args).await?;
    } else {
        // Default behavior or error if no specific command flag is set
        println!("No specific command flag set. Use --help for options.");
    }

    Ok(())
}
