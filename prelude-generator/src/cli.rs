
use std::path::PathBuf;
use clap::Parser;
use crate::args::Args;
use crate::config_parser::Config;

pub fn parse_arguments_and_config() -> anyhow::Result<(Args, Option<Config>)> {

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


    let config = if let Some(config_file_path) = &args.config_file_path {
        Some(crate::config_parser::read_config(config_file_path, &project_root)?)
    } else {
        // If config_file_path is not provided, try to read from the default location
        let default_config_path = project_root.join("config.toml");
        if default_config_path.exists() {
            Some(crate::config_parser::read_config(&default_config_path, &project_root)?)
        } else {
            None
        }
    };
    Ok((args, config))
}
