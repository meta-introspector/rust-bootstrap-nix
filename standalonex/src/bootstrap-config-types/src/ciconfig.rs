use crate::prelude::*;
/// Configuration for CI-related paths and settings.
#[derive(Debug, Default, Clone)]
pub struct CiConfig {
    pub channel_file: PathBuf,
    pub version_file: PathBuf,
    pub tools_dir: PathBuf,
    pub llvm_project_dir: PathBuf,
    pub gcc_dir: PathBuf,
}
