use anyhow::{Context, Result};
use std::path::Path;
use sha2::{Sha256, Digest};
use indoc::indoc;
use crate::use_extractor::rustc_info::RustcInfo;
use crate::error_collector::ErrorSample;
use chrono::Utc;
//use tokio::io::AsyncWriteExt;
use tokio::fs;

pub async fn expand_macros_and_parse(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), file_path: &Path, content: &str, rustc_info: &RustcInfo, cache_dir: &Path) -> Result<(syn::File, Option<ErrorSample>)> {
    // Calculate content hash
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    // Create a unique cache key based on file hash, rustc info, and rustc flags
    let cache_key = format!(
        "expanded_{}_{}_{}_{}_{}",
        content_hash,
        rustc_info.version,
        rustc_info.host,
        "lib", // --crate-type
        "2021" // --edition
    );
    let cached_file_path = cache_dir.join(cache_key);

    // Check if expanded code is already cached
    if cached_file_path.exists() {
        writer.write_all(format!("      -> Using cached expanded code for: {}\n", file_path.display()).as_bytes()).await?;
    let expanded_code = fs::read_to_string(&cached_file_path).await
        .with_context(|| format!("Failed to read cached expanded code for {}", file_path.display()))?;
        return Ok((syn::parse_file(&expanded_code).with_context(|| format!("Failed to parse cached expanded code for {}", file_path.display()))?, None));
    }

    // If not cached, perform expansion by creating a temporary crate
    let temp_crate_dir = tempfile::tempdir()?;
    let temp_crate_path = temp_crate_dir.path();

    // Create Cargo.toml for the temporary crate
    let cargo_toml_content = indoc! {
        r#"[package]
        name = "temp_crate"
        version = "0.1.0"
        edition = "2021"

        [dependencies.serde]
        version = "1.0"
        features = ["derive"]

        [dependencies.serde_json]
        version = "1.0"

        [dependencies.anyhow]
        version = "1.0"
        "#
    };
    tokio::fs::write(temp_crate_path.join("Cargo.toml"), cargo_toml_content).await?;

    // Create src directory
    let temp_src_dir = temp_crate_path.join("src");
    tokio::fs::create_dir(&temp_src_dir).await?;

    // Write the original content directly to lib.rs of the temporary crate
    let lib_rs_content = format!(
        "#![allow(unused_imports)]\n#![allow(dead_code)]\n{}",
        content
    );
    tokio::fs::write(temp_src_dir.join("lib.rs"), lib_rs_content).await?;

    writer.write_all(format!("        -> PATH environment variable: {:?}\n", std::env::var("PATH")).as_bytes()).await?;
    writer.write_all(format!("        -> Running cargo rustc -Zunpretty=expanded for: {}\n", file_path.display()).as_bytes()).await?;
    let output = tokio::process::Command::new("cargo")
        .arg("rustc")
        .arg("--")
        .arg("-Zunpretty=expanded")
        .arg("--crate-type")
        .arg("lib")
        .current_dir(temp_crate_path)
        .output().await?;

    writer.write_all(format!("        -> cargo rustc status for {}: {}\n", file_path.display(), output.status).as_bytes()).await?;
    writer.write_all(format!("        -> cargo rustc stdout for {}: {}\n", file_path.display(), String::from_utf8_lossy(&output.stdout)).as_bytes()).await?;
    if !output.status.success() {
        let error_message = format!("cargo rustc macro expansion failed: {}\nStdout: {}\nStderr: {}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        writer.write_all(format!("        -> {}\n", error_message).as_bytes()).await.context("Failed to write rustc error to writer")?;
        let error_sample = ErrorSample {
            file_path: file_path.to_path_buf(),
            rustc_version: rustc_info.version.clone(),
            rustc_host: rustc_info.host.clone(),
            error_message: error_message.clone(),
            error_type: "MacroExpansionFailed".to_string(),
            code_snippet: Some(content.to_string()),
            timestamp: Utc::now(),
            context: None,
        };
        return Ok((syn::parse_file("").unwrap(), Some(error_sample))); // Return a dummy syn::File and the error sample
    }

    let expanded_code = String::from_utf8_lossy(&output.stdout).to_string();

    // For now, assume the entire expanded_code is relevant.
    // This might need refinement if cargo rustc -Zunpretty=expanded output
    // contains other artifacts.
    let relevant_expanded_code = expanded_code.to_string();

    writer.write_all(format!("        -> Writing expanded code to cache for: {}\n", file_path.display()).as_bytes()).await?;
    // Cache the expanded code
    tokio::fs::write(&cached_file_path, &relevant_expanded_code).await
        .with_context(|| format!("Failed to write expanded code to cache for {}", file_path.display()))?;
    writer.write_all(format!("      -> Wrote expanded code to cache: {}\n", cached_file_path.display()).as_bytes()).await?;

    writer.write_all(format!("        -> Parsing expanded code for: {}\n", file_path.display()).as_bytes()).await?;
    match syn::parse_file(&relevant_expanded_code) {
        Ok(file) => Ok((file, None)),
        Err(e) => {
            let error_message = format!("Failed to parse expanded code for {}: {}", file_path.display(), e);
            writer.write_all(format!("        -> {}\n", error_message).as_bytes()).await.context("Failed to write parsing error to writer")?;
            let error_sample = ErrorSample {
                file_path: file_path.to_path_buf(),
                rustc_version: rustc_info.version.clone(),
                rustc_host: rustc_info.host.clone(),
                error_message: error_message.clone(),
                error_type: "SynParsingFailed".to_string(),
                code_snippet: Some(relevant_expanded_code.to_string()),
                timestamp: Utc::now(),
                context: None,
            };
            Ok((syn::parse_file("").unwrap(), Some(error_sample))) // Return a dummy syn::File and the error sample
        }
    }
}
