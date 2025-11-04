use serde::Serialize;
use serde::Deserialize;
use std::path::PathBuf;
#[derive(Serialize, Deserialize, Debug)]
pub enum FileProcessingStatus {
    Success,
    Skipped { reason: String },
    Failed { error: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileProcessingResult {
    pub path: PathBuf,
    pub status: FileProcessingStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectedPreludeInfo {
    pub crate_name: String,
    pub crate_root: PathBuf,
    pub prelude_content: String,
    pub modified_files: Vec<PathBuf>,
    pub crate_root_modified: bool,
    pub file_processing_results: Vec<FileProcessingResult>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FileMetadata {
    pub global_uses: Vec<String>,
    pub feature_attributes: Vec<String>,
    pub extern_crates: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RustcInfo {
    pub version: String,
    pub host: String,
}

#[derive(Debug)]
pub struct DeclarationExtractionArgs {
    pub file_path: PathBuf,
    pub rustc_info: RustcInfo,
    pub crate_name: Option<String>,
}
