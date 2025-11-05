use anyhow::{Context, Result};
use serde_json;
use std::fs;

pub async fn get_flake_lock_json() -> Result<serde_json::Value> {
    let flake_lock_content = fs::read_to_string("flake.lock")
        .context("Failed to read flake.lock file")?;
    serde_json::from_str(&flake_lock_content)
        .context("Failed to parse flake.lock JSON")
}
