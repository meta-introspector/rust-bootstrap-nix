use build_helper::prelude::*;
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(untagged)]
pub enum StringOrInt {
    String(String),
    Int(i64),
}
