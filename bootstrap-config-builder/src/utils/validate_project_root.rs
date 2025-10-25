use crate::prelude::*


use anyhow::{Context, Result};
use std::{
    fs,
    path::PathBuf,
};

pub fn validate_project_root(project_root: &PathBuf) -> Result<PathBuf> {
    let canonicalized_root = fs::canonicalize(project_root)
        .with_context(|| format!("Failed to find absolute path for project root: {:?}", project_root))?;

    if !canonicalized_root.join("flake.nix").exists() {
        anyhow::bail!("flake.nix not found in the specified project root: {:?}", canonicalized_root);
    }
    Ok(canonicalized_root)
}