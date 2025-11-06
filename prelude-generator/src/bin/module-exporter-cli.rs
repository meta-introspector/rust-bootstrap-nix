use anyhow::Result;
use std::path::PathBuf;
use prelude_generator::config_parser;
use prelude_generator::module_exporter;
use prelude_generator::module_verifier;
use clap::Parser;
use prelude_generator::external_interfaces::ExternalInterfaceGateway;

#[derive(Parser, Debug, Clone, Default)]
#[command(author, version, about, long_about = None)]
pub struct ModuleExporterArgs {
    /// Run in dry-run mode, printing changes without modifying files.
    #[arg(long)]
    pub dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = ModuleExporterArgs::parse();

    let project_root = PathBuf::from(std::env::current_dir()?);
    let config_path = project_root.join("config.toml");

    let gateway = ExternalInterfaceGateway::default();
    let config = config_parser::read_config(&config_path, &project_root, &gateway.io_interface).await?;

    let generated_exports = module_exporter::generate_module_exports(&config);

    if args.dry_run {
        println!("\n--- Dry Run: Generated Module Exports (no files modified) ---");
    } else {
        println!("\n--- Generated Module Exports ---");
    }
    println!("{}", generated_exports);
    println!("--------------------------------");

    module_verifier::verify_module_exports(&generated_exports, &config, &gateway)?;

    Ok(())
}

