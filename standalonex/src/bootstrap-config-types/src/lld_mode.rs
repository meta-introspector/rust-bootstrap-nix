use std::fmt;
use serde::{Deserialize, Deserializer};
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub enum LldMode {
    /// Do not use LLD
    #[default]
    Unused,
    /// Use `rust-lld` from the compiler's sysroot
    SelfContained,
    /// Use an externally provided `lld` binary.
    /// Note that the linker name cannot be overridden, the binary has to be named `lld` and it has
    /// to be in $PATH.
    External,
}
impl LldMode {
    pub fn is_used(&self) -> bool {
        match self {
            LldMode::SelfContained | LldMode::External => true,
            LldMode::Unused => false,
        }
    }
}
impl<'de> Deserialize<'de> for LldMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LldModeVisitor;
        impl serde::de::Visitor<'_> for LldModeVisitor {
            type Value = LldMode;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("one of true, 'self-contained' or 'external'")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(if v { LldMode::External } else { LldMode::Unused })
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "external" => Ok(LldMode::External),
                    "self-contained" => Ok(LldMode::SelfContained),
                    _ => Err(E::custom(&format!("unknown mode {}", v))),
                }
            }
        }
        deserializer.deserialize_any(LldModeVisitor)
    }
}
