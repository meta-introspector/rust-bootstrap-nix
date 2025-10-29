use anyhow::{Context, Result};
use std::path::Path;
use sha2::{Sha256, Digest};
use indoc::indoc;
use crate::use_extractor::rustc_info::RustcInfo;
use tokio::io::AsyncWriteExt;
use tokio::fs;

pub async fn expand_macros_and_parse(writer: &mut (impl tokio::io::AsyncWriteExt + Unpin), file_path: &Path, content: &str, rustc_info: &RustcInfo, cache_dir: &Path) -> Result<syn::File> {
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
        return syn::parse_file(&expanded_code).with_context(|| format!("Failed to parse cached expanded code for {}", file_path.display()));
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

    // Write the original content to a file within the temporary crate
    let temp_rs_file_name = file_path.file_name().unwrap_or_else(|| "temp_file.rs".as_ref());
    let temp_rs_file_path = temp_src_dir.join(temp_rs_file_name);
    tokio::fs::write(&temp_rs_file_path, content).await?;

    // Create lib.rs that includes the target file
    let lib_rs_content = format!(
        "#![allow(unused_imports)]\n#![allow(dead_code)]\ninclude!(\"{}\");\n",
        temp_rs_file_name.to_string_lossy() // Pass the full file name
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
        writer.write_all(format!("        -> cargo rustc stderr for {}: {}\n", file_path.display(), String::from_utf8_lossy(&output.stderr)).as_bytes()).await.context("Failed to write rustc error to writer")?;
        return Err(anyhow::anyhow!("cargo rustc macro expansion failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let expanded_code = String::from_utf8_lossy(&output.stdout).to_string();

    // Extract the relevant expanded code for the specific file
    // This is a heuristic and might need refinement.
    let search_string = format!("// {}
", temp_rs_file_name.to_string_lossy());
    let start_index = expanded_code.find(&search_string).unwrap_or(0);
    let end_index = expanded_code[start_index..].find("// ").map_or(expanded_code.len(), |i| start_index + i);
    let relevant_expanded_code = expanded_code[start_index..end_index].to_string();

    writer.write_all(format!("        -> Writing expanded code to cache for: {}\n", file_path.display()).as_bytes()).await?;
    // Cache the expanded code
    tokio::fs::write(&cached_file_path, &relevant_expanded_code).await
        .with_context(|| format!("Failed to write expanded code to cache for {}", file_path.display()))?;
    writer.write_all(format!("      -> Wrote expanded code to cache: {}\n", cached_file_path.display()).as_bytes()).await?;

    writer.write_all(format!("        -> Parsing expanded code for: {}\n", file_path.display()).as_bytes()).await?;
    return syn::parse_file(&relevant_expanded_code).with_context(|| format!("Failed to parse expanded code for {}", file_path.display()));
}
