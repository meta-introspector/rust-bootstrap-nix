use std::collections::HashMap;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use crate::metadata::{CargoMetadata, Resolve, Node, PackageId};
use crate::manifest::ExpandedFileEntry;

fn calculate_package_layers(cargo_metadata: &CargoMetadata) -> HashMap<PackageId, u32> {
    let mut package_layers: HashMap<PackageId, u32> = HashMap::new();
    let mut package_id_to_name: HashMap<PackageId, String> = HashMap::new();

    for pkg in &cargo_metadata.packages {
        package_id_to_name.insert(pkg.id.clone(), pkg.name.clone());
        package_layers.insert(pkg.id.clone(), u32::MAX); // Initialize all to unassigned
    }

    let mut assigned_count = 0;
    let total_packages = cargo_metadata.packages.len();

    while assigned_count < total_packages {
        let mut newly_assigned_in_this_iteration = 0;
        for node in &cargo_metadata.resolve.nodes {
            if package_layers.get(&node.id).map_or(false, |&layer| layer != u32::MAX) {
                continue; // Already assigned
            }

            let mut all_deps_assigned = true;
            let mut max_dep_layer = 0;

            for dep_id in &node.dependencies {
                // Only consider dependencies that are part of the workspace (i.e., have an entry in package_layers)
                if package_id_to_name.contains_key(dep_id) {
                    if let Some(&dep_layer) = package_layers.get(dep_id) {
                        if dep_layer == u32::MAX {
                            all_deps_assigned = false;
                            break;
                        }
                        max_dep_layer = max_dep_layer.max(dep_layer + 1);
                    } else {
                        // This should not happen if package_id_to_name is correctly populated for all workspace packages
                        eprintln!("Warning: Dependency {} not found in package_id_to_name map.", dep_id.repr);
                    }
                }
            }

            if all_deps_assigned {
                package_layers.insert(node.id.clone(), max_dep_layer);
                newly_assigned_in_this_iteration += 1;
            }
        }

        if newly_assigned_in_this_iteration == 0 && assigned_count < total_packages {
            eprintln!("Warning: Could not assign layers to all packages. Possible circular dependencies or unresolvable graph.");
            break;
        }
        assigned_count += newly_assigned_in_this_iteration;
    }

    package_layers
}

pub async fn expand_code(
    metadata_path: &Path,
    output_dir: &Path,
    flake_lock_json: &serde_json::Value,
    layer: Option<u32>,
) -> Result<Vec<ExpandedFileEntry>> {
    let mut expanded_files_entries = Vec::new();

    // Read cargo metadata
    let metadata_content = fs::read_to_string(metadata_path)
        .context("Failed to read cargo metadata file")?;
    let cargo_metadata: CargoMetadata = serde_json::from_str(&metadata_content)
        .context("Failed to parse cargo metadata JSON")?;

    let package_layers = calculate_package_layers(&cargo_metadata);

    for package in cargo_metadata.packages {
        if let Some(requested_layer) = layer {
            if let Some(package_layer) = package_layers.get(&package.id) {
                if *package_layer != requested_layer {
                    println!("Skipping package {} (layer {}), not in requested layer {}.", package.name, package_layer, requested_layer);
                    continue; // Skip this package if it's not in the requested layer
                }
            } else {
                eprintln!("Warning: Package {} (ID: {}) not found in package_layers. Skipping.", package.name, package.id.repr);
                continue;
            }
        }

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
                    layer: *package_layers.get(&package.id).unwrap_or(&u32::MAX),
                });
                continue;
            }

            println!("Running cargo expand for {} ({})...", package.name, target_type);

            let cargo_expand_command = format!("nix develop --command bash -c \"cargo expand --manifest-path {} --{} {} --all-features --color=always --verbose\"",
                package.manifest_path.display(),
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
                    layer: *package_layers.get(&package.id).unwrap_or(&u32::MAX),
                });
            } else {
                eprintln!("Error expanding {} ({} {}):\nStdout: {}\nStderr: {}",
                    package.name,
                    target_type,
                    target.name,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    Ok(expanded_files_entries)
}
