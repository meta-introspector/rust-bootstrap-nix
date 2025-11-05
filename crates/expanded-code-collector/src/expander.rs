use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use crate::metadata::CargoMetadata;
use crate::manifest::ExpandedFileEntry;

pub async fn expand_code(
    metadata_path: &Path,
    output_dir: &Path,
    flake_lock_json: &serde_json::Value,
) -> Result<Vec<ExpandedFileEntry>> {
    let mut expanded_files_entries = Vec::new();

    // Read cargo metadata
    let metadata_content = fs::read_to_string(metadata_path)
        .context("Failed to read cargo metadata file")?;
    let cargo_metadata: CargoMetadata = serde_json::from_str(&metadata_content)
        .context("Failed to parse cargo metadata JSON")?;

    for package in cargo_metadata.packages {
        println!("Processing package: {}", package.name);


        for target in package.targets {
            println!("  Processing target: {} (kind: {:?})", target.name, target.kind);
            let target_type = if target.kind.contains(&"lib".to_string()) {
                "lib"
            } else if target.kind.contains(&"bin".to_string()) {
                "bin"
            } else {
                println!("    Skipping target {} (unsupported type: {:?}).", target.name, target.kind);
                continue; // Skip other target types for now
            };

            let output_file_prefix = format!("{}/.expand_output_{}_{}",
                output_dir.display(),
                package.name.replace("-", "_"),
                target.name.replace("-", "_")
            );
            let expanded_rs_path = PathBuf::from(format!("{}_{}.rs", output_file_prefix, target_type));

            // Check if expanded file already exists and is up to date (simple check for now)
            if expanded_rs_path.exists() {
                println!("Expanded file for {} ({}) is up to date.", package.name, target_type);
                let timestamp = expanded_rs_path.metadata()?.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();
                expanded_files_entries.push(ExpandedFileEntry {
                    package_name: package.name.clone(),
                    target_type: target_type.to_string(),
                    target_name: target.name.clone(),
                    expanded_rs_path: expanded_rs_path.clone(),
                    cargo_expand_command: "".to_string(), // Command not stored if skipped
                    timestamp,
                    flake_lock_details: flake_lock_json.clone(),
                });
                continue;
            }

            println!("Running cargo expand for {} ({})...", package.name, target_type);

            let cargo_expand_command = format!("nix develop --command bash -c \"cargo expand -p {} --{} {}\"",
                package.name,
                target_type,
                target.name
            );
            println!("  Command: {}", cargo_expand_command);

            let output = Command::new("bash")
                .arg("-c")
                .arg(&cargo_expand_command)
                .output()
                .await
                .context(format!("Failed to execute cargo expand for {} ({})", package.name, target.name))?;

            if output.status.success() {
                println!("  cargo expand successful for {} ({}).", package.name, target_type);
                fs::write(&expanded_rs_path, &output.stdout)
                    .context(format!("Failed to write expanded RS file for {} ({})", package.name, target.name))?;

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();

                expanded_files_entries.push(ExpandedFileEntry {
                    package_name: package.name.clone(),
                    target_type: target_type.to_string(),
                    target_name: target.name.clone(),
                    expanded_rs_path: expanded_rs_path.clone(),
                    cargo_expand_command: cargo_expand_command.clone(),
                    timestamp,
                    flake_lock_details: flake_lock_json.clone(),
                });
            } else {
                eprintln!("Error expanding {} ({} {}): {}",
                    package.name,
                    target_type,
                    target.name,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    Ok(expanded_files_entries)
}
