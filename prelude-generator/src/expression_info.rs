use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionInfo {
    pub expression_str: String,
    pub depth: usize,
    pub used_types: HashSet<String>,
    pub other_types_count: usize,
    pub node_type: String,
}
