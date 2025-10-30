use std::fmt::Debug;

pub trait LlvmInfo: Send + Sync + Debug {}

#[derive(Debug, Clone)]
pub struct LlvmDetails {}

impl LlvmInfo for LlvmDetails {}
