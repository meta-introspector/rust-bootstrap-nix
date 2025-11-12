use std::path::PathBuf;
use std::fs;
use serde::Deserialize;

use bootstrap::src::core::generate_steps::git_modules::create_branch::create_and_push_branch;

#[derive(Debug, Deserialize)]
struct GitConfig {
    base_branch: String,
    new_flake_branch_prefix: String,
    component: String,
    arch: String,
    phase: String,
    step: String,
    output_dir_prefix: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    git: GitConfig,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // standalonex/src/bootstrap/src/bin
        .unwrap()
        .parent() // standalonex/src/bootstrap/src
        .unwrap()
        .parent() // standalonex/src/bootstrap
        .unwrap()
        .parent() // standalonex/src
        .unwrap()
        .parent() // standalonex
        .unwrap()
        .to_path_buf(); // rust-bootstrap-nix root

    let config_path = repo_root.join("config.toml");
    let config_content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_content)?;

    let base_branch_name = config.git.base_branch;
    let branch_name = format!(
        "{}/{}/{}/{}/{}",
        config.git.new_flake_branch_prefix,
        config.git.component,
        config.git.arch,
        config.git.phase,
        config.git.step
    );
    let output_dir = repo_root.join(format!(
        "{}/{}/{}/{}/{}",
        config.git.output_dir_prefix,
        config.git.component,
        config.git.arch,
        config.git.phase,
        config.git.step
    ));

    create_and_push_branch(
        &repo_root,
        &branch_name,
        &base_branch_name,
        &output_dir,
        false, // dry_run
    )?;

    Ok(())
}
