use crate::core::builder::Compiler;
use crate::core::config::TargetSelection;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommonTestFields {
    pub compiler: Compiler,
    pub target: TargetSelection,
    pub host: TargetSelection,
    pub stage: u32,
}
