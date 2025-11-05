use std::collections::{HashMap, BTreeSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StructLatticeInfo {
    pub struct_name: String,
    pub field_co_occurrences: HashMap<BTreeSet<String>, usize>,
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
        *self.field_co_occurrences.entry(field_types).or_insert(0) += 1;
        self.total_expressions_analyzed += 1;
    }
}
