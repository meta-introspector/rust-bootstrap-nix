//use crate::external_interfaces::IoInterface;
use std::path::PathBuf;
use clap::Parser;
use crate::args::Args;
use crate::config_parser::Config;
//use crate::external_interfaces::ExternalInterfaceGateway;
use crate::external_interfaces::ExternalInterfaceGateway;

pub async fn parse_arguments_and_config() -> anyhow::Result<(Args, Option<Config>)> {

    let args = Args::parse();

    // Determine the project root. If args.path is ".", resolve it to the actual current directory.
    // Then, find the parent directory that contains the Cargo.toml for the workspace.
    // For now, we'll assume args.path is the project root if it's explicitly set,
    // otherwise, we'll try to find the workspace root from the current executable's directory.
    let project_root = if args.path == PathBuf::from(".") {
        std::env::current_dir()?.parent().unwrap().to_path_buf()
    } else {
        PathBuf::from(&args.path)
    };

    let gateway = ExternalInterfaceGateway::default();

        let config = if let Some(config_file_path) = config_file_path {
            Some(crate::config_parser::read_config(config_file_path, &project_root, &gateway.iointerface).await?)
        } else if gateway.iointerface.path_exists(&default_config_path).await {
            Some(crate::config_parser::read_config(&default_config_path, &project_root, &gateway.iointerface).await?)
        } else {
            None
        };
    Ok((args, config))
}
