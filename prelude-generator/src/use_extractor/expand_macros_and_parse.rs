use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::Path;
//use indoc::indoc;
use crate::use_extractor::rustc_info::RustcInfo;
use chrono::Utc;
use split_expanded_lib::ErrorSample;
//use tokio::io::AsyncWriteExt;
use super::cache_manager::CacheManager;
use super::rustc_macro_expander::RustcMacroExpander;
use super::syn_file_parser::SynFileParser;
use super::temp_crate_builder::TempCrateBuilder;
use tempfile;
use tokio::fs; // Add this line

pub async fn expand_macros_and_parse(
    writer: &mut (impl tokio::io::AsyncWriteExt + Unpin),
    file_path: &Path,
    crate_root: &Path,
    manifest_path: &Path,
    rustc_info: &RustcInfo,
    cache_dir: &Path,
) -> Result<(syn::File, Option<ErrorSample>)> {
    // Read the content of the file once, and calculate its hash
    let content = tokio::fs::read_to_string(file_path)
        .await
        .with_context(|| {
            format!(
                "Failed to read file content for hashing: {}",
                file_path.display()
            )
        })?;
    let mut hasher = Sha256::new();
    hasher.update(file_path.to_string_lossy().as_bytes());
    hasher.update(crate_root.to_string_lossy().as_bytes());
    hasher.update(content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    if let Some(cached_code) =
        CacheManager::get_cached_expanded_code(writer, file_path, crate_root, rustc_info, cache_dir)
            .await?
    {
        return Ok((
            syn::parse_file(&cached_code).with_context(|| {
                format!(
                    "Failed to parse cached expanded code for {}",
                    file_path.display()
                )
            })?,
            None,
        ));
    }

    let (temp_crate_dir, temp_cargo_toml_path) =
        TempCrateBuilder::build_temp_crate(file_path, manifest_path).await?;
    let temp_crate_path = temp_crate_dir.path(); // Keep temp_crate_path for current_dir in Command

    // Create a dummy main.rs if the original was a binary, or if it's a lib, ensure it's a lib
    // For simplicity, we'll always treat the temporary crate as a library.
    // The original file's content is now in temp_lib_rs_path.

    let (expanded_code, error_sample_macro_expansion) = RustcMacroExpander::expand_macro(
        writer,
        file_path,
        &temp_cargo_toml_path,
        rustc_info,
        &content,
    )
    .await?;

    if let Some(error_sample) = error_sample_macro_expansion {
        return Ok((syn::parse_file("").unwrap(), Some(error_sample)));
    }

    // For now, assume the entire expanded_code is relevant.
    // This might need refinement if rustc -Zunpretty=expanded output
    // contains other artifacts.
    let relevant_expanded_code = expanded_code.to_string();

    CacheManager::cache_expanded_code(
        writer,
        file_path,
        crate_root,
        rustc_info,
        cache_dir,
        &relevant_expanded_code,
    )
    .await?;

    // --- REPLACED CODE BLOCK (Syn Parsing) ---
    SynFileParser::parse_expanded_code(writer, file_path, relevant_expanded_code, rustc_info).await
    // --- END REPLACED CODE BLOCK ---
}
