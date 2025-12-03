use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use split_expanded_lib::ErrorSample;
use chrono::Utc;
use tokio::io::AsyncWriteExt; // For writer.write_all
use crate::use_extractor::rustc_info::RustcInfo; // Assuming RustcInfo is defined here

pub struct RustcMacroExpander;

impl RustcMacroExpander {
    pub async fn expand_macro(
        writer: &mut (impl tokio::io::AsyncWriteExt + Unpin),
        file_path: &Path, // Original file path for error reporting
        temp_cargo_toml_path: &Path,
        rustc_info: &RustcInfo,
        original_file_content: &str, // Content of the original file for ErrorSample
    ) -> Result<(String, Option<ErrorSample>)> { // Returns expanded code and an optional error sample
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
                code_snippet: Some(original_file_content.to_string()),
                timestamp: Utc::now(),
                context: None,
            };
            return Ok((String::new(), Some(error_sample))); // Return empty string and the error sample
        }

        let expanded_code = String::from_utf8_lossy(&output.stdout).to_string();
        Ok((expanded_code, None))
    }
}