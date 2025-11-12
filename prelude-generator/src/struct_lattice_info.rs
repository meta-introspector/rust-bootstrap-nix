use std::collections::{HashMap, BTreeSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StructLatticeInfo {
    pub struct_name: String,
    pub field_co_occurrences: HashMap<String, usize>,
    // Key: set of co-occurring field types, Value: count of co-occurrence
    pub total_expressions_analyzed: usize,
}

impl StructLatticeInfo {
    pub fn new(struct_name: String) -> Self {
        StructLatticeInfo {
            struct_name,
            field_co_occurrences: HashMap::new(),
            total_expressions_analyzed: 0,
        }
    }

    pub fn add_co_occurrence(&mut self, field_types: BTreeSet<String>) {
        let key = field_types.into_iter().collect::<Vec<String>>().join("::");
        *self.field_co_occurrences.entry(key).or_insert(0) += 1;
        self.total_expressions_analyzed += 1;
    }
}
