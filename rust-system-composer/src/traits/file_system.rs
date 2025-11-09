use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::process::Output;
use walkdir::WalkDir;

#[async_trait]
pub trait FileSystem: Send + Sync {
    async fn create_dir_all(&self, path: &Path) -> Result<()>;
    async fn write_file(&self, path: &Path, contents: &[u8]) -> Result<()>;
    async fn read_to_string(&self, path: &Path) -> Result<String>;
    fn exists(&self, path: &Path) -> bool;
    fn is_dir(&self, path: &Path) -> bool;
    fn current_dir(&self) -> Result<PathBuf>;
    // WalkDir is an iterator, so returning it directly might be problematic for trait objects.
    // For now, let's return a Vec of paths, or rethink this if performance is an issue.
    async fn read_dir_recursive(&self, path: &Path) -> Result<Vec<PathBuf>>;
}
