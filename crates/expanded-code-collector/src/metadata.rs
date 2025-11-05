use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoMetadata {
    pub packages: Vec<Package>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    #[serde(default)]
    pub targets: Vec<Target>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    pub name: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
