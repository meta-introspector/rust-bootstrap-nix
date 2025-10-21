use crate::prelude::*;
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

impl Default for StringOrBool {
fn default() -> StringOrBool {
        StringOrBool::Bool(false)
    }
}

impl StringOrBool {
pub fn is_string_or_true(&self) -> bool {
        matches!(self, Self::String(_) | Self::Bool(true))
    }
}
