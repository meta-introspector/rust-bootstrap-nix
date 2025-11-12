use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashSet;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Metadata {
    packages: Vec<Package>,
    workspace_members: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    id: String,
    source: Option<String>,
}

fn main() -> Result<()> {
    // 1. Execute `cargo metadata`
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .context("Failed to execute cargo metadata")?;

    if !output.status.success() {
        anyhow::bail!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // 2. Parse the JSON output
    let metadata: Metadata = serde_json::from_slice(&output.stdout)
        .context("Failed to parse cargo metadata JSON output")?;

    // Collect workspace member IDs for quick lookup
    let workspace_member_ids: HashSet<&str> = metadata
        .workspace_members
        .iter()
        .map(|s| s.as_str())
        .collect();

    println!("External Dependencies:");

    // 3. Identify external dependencies
    for package in metadata.packages {
        // A package is external if it's not a workspace member and has a source (i.e., not a local path dependency)
        if !workspace_member_ids.contains(package.id.as_str()) && package.source.is_some() {
            println!("- {} (source: {})", package.name, package.source.unwrap());
        }
    }

    Ok(())
}