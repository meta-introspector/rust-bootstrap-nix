use anyhow::{Context, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;

use crate::traits::file_system::FileSystem;

pub struct RealFileSystem;

#[async_trait]
impl FileSystem for RealFileSystem {
    async fn create_dir_all(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .await
            .context(format!("Failed to create directory: {}", path.display()))
    }

    async fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()> {
        fs::write(path, contents)
            .await
            .context(format!("Failed to write file: {}", path.display()))
    }

    async fn read_to_string(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .await
            .context(format!("Failed to read file: {}", path.display()))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn current_dir(&self) -> Result<PathBuf> {
        std::env::current_dir().context("Failed to get current directory")
    }

    async fn read_dir_recursive(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                files.push(entry.path().to_path_buf());
            }
        }
        Ok(files)
    }
}
