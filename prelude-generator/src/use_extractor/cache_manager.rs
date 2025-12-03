use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use tokio::fs;
use crate::use_extractor::rustc_info::RustcInfo; // Assuming RustcInfo is defined here

pub struct CacheManager;

impl CacheManager {
    /// Calculates a content hash for the given file and crate root.
    async fn calculate_content_hash(file_path: &Path, crate_root: &Path) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(file_path.to_string_lossy().as_bytes());
        hasher.update(crate_root.to_string_lossy().as_bytes());
        let content = fs::read_to_string(file_path).await
            .with_context(|| format!("Failed to read file content for hashing: {}", file_path.display()))?;
        hasher.update(content.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Constructs a cache key for expanded code.
    fn construct_cache_key(
        content_hash: &str,
        rustc_info: &RustcInfo,
        cargo_expand_flag: Option<&str>,
        edition: &str,
    ) -> String {
        match cargo_expand_flag {
            Some(flag) => format!(
                "expanded_{}_{}_{}_{}_{}",
                content_hash, rustc_info.version, rustc_info.host, flag, edition
            ),
            None => format!(
                "expanded_{}_{}_{}_{}",
                content_hash, rustc_info.version, rustc_info.host, edition
            ),
        }
    }

    /// Checks if the expanded code is cached and returns it if found.
    pub async fn get_cached_expanded_code(
        writer: &mut (impl tokio::io::AsyncWriteExt + Unpin),
        file_path: &Path,
        crate_root: &Path,
        rustc_info: &RustcInfo,
        cache_dir: &Path,
    ) -> Result<Option<String>> {
        let content_hash = Self::calculate_content_hash(file_path, crate_root).await?;

        // Check for cargo_expand cache key
        let cache_key_cargo_expand = Self::construct_cache_key(&content_hash, rustc_info, Some("cargo_expand"), "2021");
        let cached_file_path_cargo_expand = cache_dir.join(&cache_key_cargo_expand);

        if cached_file_path_cargo_expand.exists() {
            writer.write_all(format!("      -> Using cached expanded code for: {}
", file_path.display()).as_bytes()).await?;
            let expanded_code = fs::read_to_string(&cached_file_path_cargo_expand).await
                .with_context(|| format!("Failed to read cached expanded code for {}", file_path.display()))?;
            return Ok(Some(expanded_code));
        }

        // Check for rustc flags cache key (without cargo_expand)
        let cache_key_rustc_flags = Self::construct_cache_key(&content_hash, rustc_info, None, "2021");
        let cached_file_path_rustc_flags = cache_dir.join(&cache_key_rustc_flags);

        if cached_file_path_rustc_flags.exists() {
            writer.write_all(format!("      -> Using cached expanded code for: {}
", file_path.display()).as_bytes()).await?;
            let expanded_code = fs::read_to_string(&cached_file_path_rustc_flags).await
                .with_context(|| format!("Failed to read cached expanded code for {}", file_path.display()))?;
            return Ok(Some(expanded_code));
        }

        Ok(None)
    }

    /// Writes the expanded code to the cache.
    pub async fn cache_expanded_code(
        writer: &mut (impl tokio::io::AsyncWriteExt + Unpin),
        file_path: &Path,
        crate_root: &Path,
        rustc_info: &RustcInfo,
        cache_dir: &Path,
        expanded_code: &str,
    ) -> Result<()> {
        let content_hash = Self::calculate_content_hash(file_path, crate_root).await?;
        // Use the cargo_expand flag for caching the result of the expansion
        let cache_key = Self::construct_cache_key(&content_hash, rustc_info, Some("cargo_expand"), "2021");
        let cached_file_path = cache_dir.join(&cache_key);

        writer.write_all(format!("        -> Writing expanded code to cache for: {}
", file_path.display()).as_bytes()).await?;
        fs::create_dir_all(cached_file_path.parent().unwrap()).await.context("Failed to create parent directories for cache file")?;
        fs::write(&cached_file_path, expanded_code).await
            .with_context(|| format!("Failed to write expanded code to cache for {}", file_path.display()))?;
        writer.write_all(format!("      -> Wrote expanded code to cache: {}
", cached_file_path.display()).as_bytes()).await?;
        Ok(())
    }
}