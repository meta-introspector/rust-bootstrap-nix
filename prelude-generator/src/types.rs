use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use split_expanded_lib::{Declaration};
use std::collections::HashMap;
use crate::{struct_lattice_info::StructLatticeInfo, enum_lattice_info::EnumLatticeInfo, impl_lattice_info::ImplLatticeInfo};
use crate::expression_info::ExpressionInfo;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectedProjectInfo {
    pub declarations: Vec<Declaration>,
    pub types: HashMap<String, split_expanded_lib::ResolvedDependency>,
    pub modules: HashMap<String, split_expanded_lib::ResolvedDependency>,
    pub crates: HashMap<String, split_expanded_lib::ResolvedDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectedAnalysisData {
    pub expressions: HashMap<String, ExpressionInfo>,
    pub struct_lattices: HashMap<String, StructLatticeInfo>,
    pub enum_lattices: HashMap<String, EnumLatticeInfo>,
    pub impl_lattices: HashMap<String, ImplLatticeInfo>,
}