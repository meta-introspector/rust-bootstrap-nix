use anyhow::{Context, Result};
use std::path::Path;
use sha2::{Sha256, Digest};
//use indoc::indoc;
use crate::use_extractor::rustc_info::RustcInfo;
use crate::error_collector::ErrorSample;
use chrono::Utc;
//use tokio::io::AsyncWriteExt;
use tokio::fs;
use tempfile;

pub async fn expand_macros_and_parse(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), file_path: &Path, crate_root: &Path, manifest_path: &Path, rustc_info: &RustcInfo, cache_dir: &Path) -> Result<(syn::File, Option<ErrorSample>)> {
    // Calculate content hash based on file_path and crate_root
    let mut hasher = Sha256::new();
    hasher.update(file_path.to_string_lossy().as_bytes());
    hasher.update(crate_root.to_string_lossy().as_bytes());
    // Also hash the content of the file itself, as it might change without file_path changing
    let content = tokio::fs::read_to_string(file_path).await
        .with_context(|| format!("Failed to read file content for hashing: {}", file_path.display()))?;
    hasher.update(content.as_bytes());

    let content_hash = format!("{:x}", hasher.finalize());

    // Create a unique cache key based on file hash, crate root, rustc info, and cargo expand flags
    let cache_key = format!(
        "expanded_{}_{}_{}_{}_{}",
        content_hash,
        rustc_info.version,
        rustc_info.host,
        "cargo_expand", // Indicate cargo expand was used
        "2021" // Edition
    );
    let cached_file_path = cache_dir.join(cache_key);

    // Check if expanded code is already cached
    if cached_file_path.exists() {
        writer.write_all(format!("      -> Using cached expanded code for: {}\n", file_path.display()).as_bytes()).await?;
    let expanded_code = fs::read_to_string(&cached_file_path).await
        .with_context(|| format!("Failed to read cached expanded code for {}", file_path.display()))?;
        return Ok((syn::parse_file(&expanded_code).with_context(|| format!("Failed to parse cached expanded code for {}", file_path.display()))?, None));
    }

    // Create a unique cache key based on file hash, rustc info, and rustc flags
    let cache_key = format!(
        "expanded_{}_{}_{}_{}",
        content_hash,
        rustc_info.version,
        rustc_info.host,
        "2021" // Edition
    );
    let cached_file_path = cache_dir.join(cache_key);

    // Check if expanded code is already cached
    if cached_file_path.exists() {
        writer.write_all(format!("      -> Using cached expanded code for: {}\n", file_path.display()).as_bytes()).await?;
    let expanded_code = fs::read_to_string(&cached_file_path).await
        .with_context(|| format!("Failed to read cached expanded code for {}", file_path.display()))?;
        return Ok((syn::parse_file(&expanded_code).with_context(|| format!("Failed to parse cached expanded code for {}", file_path.display()))?, None));
    }

    let temp_crate_dir = tempfile::tempdir().context("Failed to create temporary directory")?;
    let temp_crate_path = temp_crate_dir.path();

    // Copy the original Cargo.toml to the temporary crate directory
    let original_cargo_toml_path = manifest_path;
    let temp_cargo_toml_path = temp_crate_path.join("Cargo.toml");
    fs::copy(&original_cargo_toml_path, &temp_cargo_toml_path).await.context("Failed to copy Cargo.toml to temporary directory")?;

    // Create src directory in the temporary crate
    let temp_src_dir = temp_crate_path.join("src");
    fs::create_dir(&temp_src_dir).await.context("Failed to create temporary src directory")?;

    // Copy the file to be expanded into the temporary src directory, renaming it to lib.rs
    let original_file_stem = file_path.file_stem().unwrap_or_else(|| "temp_file".as_ref());
    let temp_lib_rs_path = temp_src_dir.join("lib.rs");
    fs::copy(file_path, &temp_lib_rs_path).await.context("Failed to copy source file to temporary lib.rs")?;

    // Create a dummy main.rs if the original was a binary, or if it's a lib, ensure it's a lib
    // For simplicity, we'll always treat the temporary crate as a library.
    // The original file's content is now in temp_lib_rs_path.

    writer.write_all(format!("        -> PATH environment variable: {:?}\n", std::env::var("PATH")).as_bytes()).await?;
    writer.write_all(format!("        -> Running cargo rustc -Zunpretty=expanded --lib for: {}\n", file_path.display()).as_bytes()).await?;
    let output = tokio::process::Command::new("cargo")
        .arg("rustc")
        .arg("--manifest-path")
        .arg(&temp_cargo_toml_path)
        .arg("--lib") // Explicitly compile as a library
        .arg("--")
        .arg("-Zunpretty=expanded")
        .output().await?;

    writer.write_all(format!("        -> rustc status for {}: {}\n", file_path.display(), output.status).as_bytes()).await?;
    writer.write_all(format!("        -> rustc stdout for {}: {}\n", file_path.display(), String::from_utf8_lossy(&output.stdout)).as_bytes()).await?;
    if !output.status.success() {
        let error_message = format!("rustc macro expansion failed: {}\nStdout: {}\nStderr: {}",
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
    // This might need refinement if rustc -Zunpretty=expanded output
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
