use anyhow::{Context, Result};
use std::path::Path;
use sha2::{Sha256, Digest};
//use indoc::indoc;
use crate::use_extractor::rustc_info::RustcInfo;
use crate::error_collector::ErrorSample;
use chrono::Utc;
//use tokio::io::AsyncWriteExt;
use tokio::fs;

pub async fn expand_macros_and_parse(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), file_path: &Path, crate_root: &Path, rustc_info: &RustcInfo, cache_dir: &Path) -> Result<(syn::File, Option<ErrorSample>)> {
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

    // Calculate relative path for cargo expand
    let file_path_relative_to_crate_root = file_path.strip_prefix(crate_root)
        .with_context(|| format!("File path {} is not within crate root {}", file_path.display(), crate_root.display()))?;

    // Determine target triple (assuming host triple for now)
    let target_triple = rustc_info.host.clone();

    writer.write_all(format!("        -> Running cargo expand for: {} in crate {}", file_path.display(), crate_root.display()).as_bytes()).await?;
    let output = tokio::process::Command::new("cargo")
        .arg("expand")
        .arg("--ugly") // Output less pretty-printed code
        .arg("--output")
        .arg("-") // Print to stdout
        .arg("--color")
        .arg("never")
        .arg("--target")
        .arg(&target_triple)
        .arg("--") // Arguments after -- are passed to rustc
        .arg(file_path_relative_to_crate_root)
        .current_dir(crate_root) // Run cargo expand from the crate root
        .output().await?;


    writer.write_all(format!("        -> PATH environment variable: {:?}\n", std::env::var("PATH")).as_bytes()).await?;
    writer.write_all(format!("        -> Running cargo rustc -Zunpretty=expanded for: {}\n", file_path.display()).as_bytes()).await?;
//     let output = tokio::process::Command::new("cargo")
//         .arg("rustc")
//         .arg("--")
//         .arg("-Zunpretty=expanded")
//         .arg("--crate-type")
// //        .arg("lib")
// //        .current_dir(temp_crate_path)
//         .output().await?;

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
