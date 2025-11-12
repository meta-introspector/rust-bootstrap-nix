use clap::Parser;
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;
use walkdir::WalkDir;
mod config_lock; // Declare the config_lock module
use crate::config_lock::{ConfigLock, StageStatus, calculate_file_hash};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(long)]
    config_file: PathBuf,

    /// Optional project root path for analysis
    #[arg(long)]
    project_root: PathBuf,

    /// Path to the config.lock file for caching and reproducibility
    #[arg(long)]
    config_lock_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("rust-system-builder started with args: {:?}", args);

    let project_root = args.project_root;
    let config_lock_path = args.config_lock_path;
    let config_file = args.config_file;

    // Load or initialize ConfigLock
    let mut config_lock: ConfigLock = if config_lock_path.exists() {
        ConfigLock::load(&config_lock_path).await?
    } else {
        ConfigLock::new()
    };

    // Calculate hash of the main config.toml
    let config_toml_hash = calculate_file_hash(&config_file).await?;
    config_lock.config_toml_hash = config_toml_hash;

    // Calculate hash of the rust-system-builder binary itself
    let current_exe_path = std::env::current_exe()?;
    let builder_binary_hash = calculate_file_hash(&current_exe_path).await?;
    config_lock.builder_binary_hash = builder_binary_hash;

    // Create a "lock_generation" stage
    let stage_name = "lock_generation";
    let mut stage_lock = config_lock.get_or_create_stage_lock(stage_name);

    let mut input_hashes: HashMap<String, String> = HashMap::new();

    // Scan project_root for .rs files and Cargo.toml
    for entry in WalkDir::new(&project_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "rs" || path.file_name().map_or(false, |name| name == "Cargo.toml") {
                    let relative_path = path.strip_prefix(&project_root)?.to_string_lossy().to_string();
                    let hash = calculate_file_hash(path).await?;
                    input_hashes.insert(relative_path, hash);
                }
            }
        }
    }

    stage_lock.input_hashes = input_hashes;
    stage_lock.status = StageStatus::Executed;
    config_lock.update_stage_lock(stage_lock);

    // Save the updated ConfigLock
    config_lock.save(&config_lock_path).await?;
    println!("ConfigLock saved to: {:?}", config_lock_path);

    Ok(())
}
    