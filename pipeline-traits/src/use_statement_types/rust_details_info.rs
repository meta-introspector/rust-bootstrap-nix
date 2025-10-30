use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct RustDetailsInfo {
    pub version: String,
    pub crate_name: String,
    pub item_path: String,
}

#[derive(Debug, Clone)]
pub enum RustDetails {
    Info(RustDetailsInfo),
    Error(String),
    Unknown,
}

pub trait RustDetailsInfoTrait: Send + Sync + Debug {
    fn version(&self) -> Option<&str>;
    fn crate_name(&self) -> Option<&str>;
    fn item_path(&self) -> Option<&str>;
}

impl RustDetailsInfoTrait for RustDetails {
    fn version(&self) -> Option<&str> {
        match self {
            RustDetails::Info(info) => Some(&info.version),
            _ => None,
        }
    }
    fn crate_name(&self) -> Option<&str> {
        match self {
            RustDetails::Info(info) => Some(&info.crate_name),
            _ => None,
        }
    }
    fn item_path(&self) -> Option<&str> {
        match self {
            RustDetails::Info(info) => Some(&info.item_path),
            _ => None,
        }
    }
}