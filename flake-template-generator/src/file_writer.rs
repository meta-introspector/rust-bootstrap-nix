use crate::prelude::*


use std::fs;
use std::path::PathBuf;

pub fn write_flake_and_config(
    absolute_output_dir: &PathBuf,
    flake_nix_content: &str,
    config_content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Write flake.nix to output directory
    let output_flake_nix_path = absolute_output_dir.join("flake.nix");
    fs::write(&output_flake_nix_path, flake_nix_content)?;

    // Copy config.toml to output directory
    let output_config_toml_path = absolute_output_dir.join("config.toml");
    fs::write(&output_config_toml_path, config_content)?;

    println!("Successfully generated flake in {:?}", absolute_output_dir);
    Ok(())
}
