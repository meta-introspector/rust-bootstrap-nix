use crate::prelude::*;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LlvmLibunwind {
    #[default]
    No,
    InTree,
    System,
}

impl FromStr for LlvmLibunwind {
    type Err = String;

fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "no" => Ok(Self::No),
            "in-tree" => Ok(Self::InTree),
            "system" => Ok(Self::System),
            invalid => Err(format!("Invalid value '{invalid}' for rust.llvm-libunwind config.")),
        }
    }
}
