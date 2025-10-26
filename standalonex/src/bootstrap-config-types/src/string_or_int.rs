use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum StringOrInt {
    String(String),
    Int(i64),
}
