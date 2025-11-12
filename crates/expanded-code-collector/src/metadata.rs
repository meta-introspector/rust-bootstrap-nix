use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoMetadata {
    pub packages: Vec<Package>,
    pub resolve: Resolve,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub id: PackageId,
    pub name: String,
    pub manifest_path: PathBuf,
    #[serde(default)]
    pub targets: Vec<Target>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PackageId {
    pub repr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub kind: Vec<String>,
    pub name: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resolve {
    pub nodes: Vec<Node>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: PackageId,
    pub dependencies: Vec<PackageId>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
