use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SynInfo {
    pub parsed_type: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub enum SynDetails {
    Info(SynInfo),
    Error(String),
    Unknown,
}

pub trait SynInfoTrait: Send + Sync + Debug {
    fn parsed_type(&self) -> Option<&str>;
    fn version(&self) -> Option<&str>;
}

impl SynInfoTrait for SynDetails {
    fn parsed_type(&self) -> Option<&str> {
        match self {
            SynDetails::Info(info) => Some(&info.parsed_type),
            _ => None,
        }
    }
    fn version(&self) -> Option<&str> {
        match self {
            SynDetails::Info(info) => Some(&info.version),
            _ => None,
        }
    }
}
