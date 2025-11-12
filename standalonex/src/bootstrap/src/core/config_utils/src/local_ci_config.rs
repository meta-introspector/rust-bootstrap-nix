use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[derive(Clone)]
pub struct LocalCiConfig {
    pub channel_file: Option<std::path::PathBuf>,
    pub version_file: Option<std::path::PathBuf>,
    pub tools_dir: Option<std::path::PathBuf>,
    pub llvm_project_dir: Option<std::path::PathBuf>,
    pub gcc_dir: Option<std::path::PathBuf>,
}
