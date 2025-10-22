use std::path::PathBuf;
use crate::target_selection::TargetSelection;

pub struct LocalFlags {
    pub set: Vec<String>,
    pub jobs: Option<u32>,
    pub build_dir: Option<PathBuf>,
    pub skip_stage0_validation: bool,
    pub host: Vec<TargetSelection>,
    pub target: Vec<TargetSelection>,
    pub src: Option<PathBuf>,
    pub config: Option<PathBuf>,
}
