use anyhow::Context;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn format_rust_code(file_path: &PathBuf) -> anyhow::Result<()> {
    let output = Command::new("rustfmt")
        .arg(file_path)
        .arg("--edition=2021") // Specify the Rust edition
        .arg("--emit=files") // Emit changes directly to the file
        .output()
        .await
        .context("Failed to execute rustfmt")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rustfmt failed for file {:?}:\n{}", file_path, stderr);
    }

    Ok(())
}

pub async fn validate_rust_code(file_path: &PathBuf) -> anyhow::Result<()> {
    let output = Command::new("rustc")
        .arg("--emit=metadata") // Only check for errors, don't produce artifacts
        .arg("--crate-type=lib") // Treat as a library crate
        .arg(file_path)
        .output()
        .await
        .context("Failed to execute rustc")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Rustc check failed for file {:?}:\n{}", file_path, stderr);
    }

    Ok(())
}

