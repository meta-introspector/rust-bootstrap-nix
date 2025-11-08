use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    pub project_info: ProjectInfo,
    pub project_config: toml::Value, // Store the raw config.toml content
    pub generated_projects: HashMap<String, GeneratedProject>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub root_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedProject {
    pub path: PathBuf,
    pub modules: Vec<PathBuf>,
    pub declarations: Option<HashMap<String, Vec<String>>>, // e.g., "functions": ["fn_name1", "fn_name2"]
}
