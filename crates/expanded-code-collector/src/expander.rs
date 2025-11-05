use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tokio::process::Command;

use crate::metadata::CargoMetadata;
use crate::expanded_metadata::ExpandedMetadata;

pub async fn expand_code(
    metadata_path: &Path,
    output_dir: &Path,
    flake_lock_json: &serde_json::Value,
) -> Result<()> {
    // Read cargo metadata
    let metadata_content = fs::read_to_string(metadata_path)
        .context("Failed to read cargo metadata file")?;
    let cargo_metadata: CargoMetadata = serde_json::from_str(&metadata_content)
        .context("Failed to parse cargo metadata JSON")?;

    for package in cargo_metadata.packages {
        // Skip packages that are not part of the workspace (e.g., registry dependencies)
        if package.extra.contains_key("source") {
            continue;
        }

        for target in package.targets {
            let target_type = if target.kind.contains(&"lib".to_string()) {
                "lib"
            } else if target.kind.contains(&"bin".to_string()) {
                "bin"
            } else {
                continue; // Skip other target types for now
            };

            let output_file_prefix = format!("{}/.expand_output_{}_{}",
                output_dir.display(),
                package.name.replace("-", "_"),
                target.name.replace("-", "_")
            );
            let expanded_rs_path = format!("{}_{}.rs", output_file_prefix, target_type);
            let expanded_json_path = format!("{}_{}.json", output_file_prefix, target_type);

            // Check if expanded file already exists and is up to date (simple check for now)
            if Path::new(&expanded_rs_path).exists() {
                println!("Expanded file for {} ({}) is up to date.", package.name, target_type);
                continue;
            }

            println!("Running cargo expand for {} ({})\n...", package.name, target_type);

            let cargo_expand_command = format!("nix develop --command bash -c \"cargo expand -p {} --{} {}\"",
                package.name,
                target_type,
                target.name
            );

            let output = Command::new("bash")
                .arg("-c")
                .arg(&cargo_expand_command)
                .output()
                .await
                .context(format!("Failed to execute cargo expand for {} ({})", package.name, target.name))?;

            if output.status.success() {
                fs::write(&expanded_rs_path, &output.stdout)
                    .context(format!("Failed to write expanded RS file for {} ({})", package.name, target.name))?;

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();

                let expanded_metadata = ExpandedMetadata {
                    package_name: package.name.clone(),
                    target_type: target_type.to_string(),
                    target_name: target.name.clone(),
                    cargo_expand_command: cargo_expand_command.clone(),
                    timestamp,
                    flake_lock_details: flake_lock_json.clone(),
                };

                let metadata_json = serde_json::to_string_pretty(&expanded_metadata)
                    .context("Failed to serialize expanded metadata")?;
                fs::write(&expanded_json_path, metadata_json)
                    .context(format!("Failed to write expanded JSON file for {} ({})", package.name, target.name))?;
            } else {
                eprintln!("Error expanding {} ({}) {}: {}",
                    package.name,
                    target_type,
                    target.name,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    Ok(())
}
