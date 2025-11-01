use serde::{Deserialize, Deserializer};
use build_helper::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustOptimize {
    String(String),
    Int(u8),
    Bool(bool),
}
impl Default for RustOptimize {
    fn default() -> RustOptimize {
        RustOptimize::Bool(false)
    }
}
impl<'de> Deserialize<'de> for RustOptimize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(OptimizeVisitor)
    }
}
