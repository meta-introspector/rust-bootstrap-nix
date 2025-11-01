use build_helper::prelude::*;
use crate::RustOptimize;
pub struct OptimizeVisitor;
impl serde::de::Visitor<'_> for OptimizeVisitor {
    type Value = RustOptimize;
    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(r#"one of: 0, 1, 2, 3, "s", "z", true, false"#)
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if matches!(value, "s" | "z") {
            Ok(RustOptimize::String(value.to_string()))
        } else {
            Err(serde::de::Error::custom(format_optimize_error_msg(value)))
        }
    }
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if matches!(value, 0..= 3) {
            Ok(RustOptimize::Int(value as u8))
        } else {
            Err(serde::de::Error::custom(format_optimize_error_msg(value)))
        }
    }
    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RustOptimize::Bool(value))
    }
}
pub fn format_optimize_error_msg(v: impl std::fmt::Display) -> String {
    format!(
        r#"unrecognized option for rust optimize: "{v}", expected one of 0, 1, 2, 3, "s", "z", true, false"#
    )
}
impl RustOptimize {
    pub(crate) fn is_release(&self) -> bool {
        match &self {
            RustOptimize::Bool(true) | RustOptimize::String(_) => true,
            RustOptimize::Int(i) => *i > 0,
            RustOptimize::Bool(false) => false,
        }
    }
    pub(crate) fn get_opt_level(&self) -> Option<String> {
        match &self {
            RustOptimize::String(s) => Some(s.clone()),
            RustOptimize::Int(i) => Some(i.to_string()),
            RustOptimize::Bool(_) => None,
        }
    }
}
