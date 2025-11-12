use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct LlvmInfo {
    pub ir_version: String,
    pub target_triple: String,
}

#[derive(Debug, Clone)]
pub enum LlvmDetails {
    Info(LlvmInfo),
    Error(String),
    Unknown,
}

pub trait LlvmInfoTrait: Send + Sync + Debug {
    fn ir_version(&self) -> Option<&str>;
    fn target_triple(&self) -> Option<&str>;
}

impl LlvmInfoTrait for LlvmDetails {
    fn ir_version(&self) -> Option<&str> {
        match self {
            LlvmDetails::Info(info) => Some(&info.ir_version),
            _ => None,
        }
    }
    fn target_triple(&self) -> Option<&str> {
        match self {
            LlvmDetails::Info(info) => Some(&info.target_triple),
            _ => None,
        }
    }
}
