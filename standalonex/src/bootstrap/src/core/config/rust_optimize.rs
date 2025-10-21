use crate::prelude::*;
use serde::Deserializer;
use crate::core::config::config_part6::OptimizeVisitor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustOptimize {
    String(String),
    Int(u8),
    Bool(bool),
}

impl Default for RustOptimize {
pub fn default() -> RustOptimize {
        RustOptimize::Bool(false)
    }
}

impl<'de> Deserialize<'de> for RustOptimize {
pub fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(OptimizeVisitor)
    }
}
