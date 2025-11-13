use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct LinuxInfo {
    pub kernel_version: String,
    pub architecture: String,
}

#[derive(Debug, Clone)]
pub enum LinuxDetails {
    Info(LinuxInfo),
    Error(String),
    Unknown,
}

pub trait LinuxInfoTrait: Send + Sync + Debug {
    fn kernel_version(&self) -> Option<&str>;
    fn architecture(&self) -> Option<&str>;
}

impl LinuxInfoTrait for LinuxDetails {
    fn kernel_version(&self) -> Option<&str> {
        match self {
            LinuxDetails::Info(info) => Some(&info.kernel_version),
            _ => None,
        }
    }
    fn architecture(&self) -> Option<&str> {
        match self {
            LinuxDetails::Info(info) => Some(&info.architecture),
            _ => None,
        }
    }
}
