use std::collections::HashMap;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use crate::metadata::{CargoMetadata, PackageId};
use crate::manifest::ExpandedFileEntry;

fn calculate_package_layers(cargo_metadata: &CargoMetadata) -> HashMap<PackageId, u32> {
    let mut package_layers: HashMap<PackageId, u32> = HashMap::new();
    let mut package_id_to_name: HashMap<PackageId, String> = HashMap::new();

    // Initialize all packages with an unassigned layer and populate id-to-name map
    for pkg in &cargo_metadata.packages {
        package_id_to_name.insert(pkg.id.clone(), pkg.name.clone());
        package_layers.insert(pkg.id.clone(), u32::MAX); // u32::MAX signifies unassigned
    }

    let mut assigned_count = 0;
    let total_packages = cargo_metadata.packages.len();
    let mut current_layer = 0;

    // First pass: assign layer 0 to packages with no dependencies
    for node in &cargo_metadata.resolve.nodes {
        if node.dependencies.is_empty() {
            if let Some(layer_entry) = package_layers.get_mut(&node.id) {
                if *layer_entry == u32::MAX { // Only assign if not already assigned
                    *layer_entry = 0;
                    assigned_count += 1;
                }
            }
        }
    }

    // Iteratively assign layers
    while assigned_count < total_packages {
        let mut newly_assigned_in_this_iteration = 0;
        let mut made_progress = false;

        for node in &cargo_metadata.resolve.nodes {
            if package_layers.get(&node.id).map_or(false, |&layer| layer != u32::MAX) {
                continue; // Already assigned
            }

            let mut all_deps_resolved = true;
            let mut max_dep_layer = 0;

            for dep_id in &node.dependencies {
                if let Some(&dep_layer) = package_layers.get(dep_id) {
                    if dep_layer == u32::MAX {
                        all_deps_resolved = false;
                        break;
                    }
                    max_dep_layer = max_dep_layer.max(dep_layer);
                } else {
                    // Dependency is not in the workspace, assume it's a base dependency (layer -1 conceptually)
                    // or handle as already resolved if it's an external crate that cargo metadata resolved.
                    // For simplicity, if it's not in our package_layers map, we consider it resolved
                    // and not contributing to a higher layer for *our* packages, unless it's a direct dependency
                    // that needs to be expanded. For layer calculation, we treat external dependencies as resolved
                    // and potentially contributing to the current package's layer if they are not layer 0.
                    // This part needs careful consideration based on how 'layer' is truly defined.
                    // For now, let's assume external dependencies are 'resolved' and don't block layer assignment.
                    // If an external dependency is truly layer 0, it should have no dependencies itself.
                }
            }

            if all_deps_resolved {
                if let Some(layer_entry) = package_layers.get_mut(&node.id) {
                    if *layer_entry == u32::MAX { // Only assign if not already assigned
                        *layer_entry = max_dep_layer + 1;
                        newly_assigned_in_this_iteration += 1;
                        made_progress = true;
                    }
                }
            }
        }

        if newly_assigned_in_this_iteration == 0 && assigned_count < total_packages {
            eprintln!("Warning: Could not assign layers to all packages. Possible circular dependencies or unresolvable graph. Unassigned packages: ");
            for (pkg_id, layer) in &package_layers {
                if *layer == u32::MAX {
                    if let Some(pkg_name) = package_id_to_name.get(pkg_id) {
                        eprintln!("- {}", pkg_name);
                    }
                }
            }
            break;
        }
        if !made_progress && assigned_count < total_packages {
            eprintln!("Warning: No progress made in assigning layers in this iteration. Possible circular dependencies or unresolvable graph. Unassigned packages: ");
            for (pkg_id, layer) in &package_layers {
                if *layer == u32::MAX {
                    if let Some(pkg_name) = package_id_to_name.get(pkg_id) {
                        eprintln!("- {}", pkg_name);
                    }
                }
            }
            break;
        }
        assigned_count += newly_assigned_in_this_iteration;
        current_layer += 1;
    }

    // Ensure all packages are assigned a layer. If not, assign remaining to a high layer or error.
    for (pkg_id, layer) in package_layers.iter_mut() {
        if *layer == u32::MAX {
            // If a package remains unassigned, it means it has unresolved dependencies
            // or is part of a circular dependency. For now, assign it to a very high layer
            // so it's unlikely to be in a requested low layer.
            *layer = u32::MAX - 1; // Assign a high layer to unresolvable packages
            if let Some(pkg_name) = package_id_to_name.get(pkg_id) {
                eprintln!("Warning: Package {} (ID: {}) could not be assigned a proper layer due to unresolved dependencies or circularity. Assigned to layer {}.\n", pkg_name, pkg_id.repr, u32::MAX - 1);
            }
        }
    }

    // Debug print assigned layers
    for (package_id, layer) in &package_layers {
        if *layer != u32::MAX {
            if let Some(package_name) = package_id_to_name.get(package_id) {
                println!("Assigned layer {} to package {}", layer, package_name);
            }
        }
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

            let cargo_expand_command = format!("cargo expand --manifest-path {} --{} {} --all-features --color=always --verbose",
                package.manifest_path.display(),
                target_type,
                target.name
            );
            println!("  Command: {}", cargo_expand_command);

            let output = Command::new("cargo")
                .arg("expand")
                .arg("--manifest-path")
                .arg(&package.manifest_path)
                .arg(format!("--{}", target_type))
                .arg(&target.name)
                .arg("--all-features")
                .arg("--color=always")
                .arg("--verbose")
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
