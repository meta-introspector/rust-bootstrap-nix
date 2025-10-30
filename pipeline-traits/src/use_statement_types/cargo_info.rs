use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct CargoInfo {
    pub package_name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub enum CargoDetails {
    Info(CargoInfo),
    Error(String),
    Unknown,
}

pub trait CargoInfoTrait: Send + Sync + Debug {
    fn package_name(&self) -> Option<&str>;
    fn version(&self) -> Option<&str>;
}

impl CargoInfoTrait for CargoDetails {
    fn package_name(&self) -> Option<&str> {
        match self {
            CargoDetails::Info(info) => Some(&info.package_name),
            _ => None,
        }
    }
    fn version(&self) -> Option<&str> {
        match self {
            CargoDetails::Info(info) => Some(&info.version),
            _ => None,
        }
    }
}
