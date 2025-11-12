use std::collections::{HashMap, BTreeSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnumLatticeInfo {
    pub enum_name: String,
    pub variant_type_co_occurrences: HashMap<String, usize>,
    // Key: set of co-occurring variant types, Value: count of co-occurrence
    pub total_expressions_analyzed: usize,
}

impl EnumLatticeInfo {
    pub fn new(enum_name: String) -> Self {
        EnumLatticeInfo {
            enum_name,
            variant_type_co_occurrences: HashMap::new(),
            total_expressions_analyzed: 0,
        }
    }

    pub fn add_co_occurrence(&mut self, variant_types: BTreeSet<String>) {
        let key = variant_types.into_iter().collect::<Vec<String>>().join("::");
        *self.variant_type_co_occurrences.entry(key).or_insert(0) += 1;
        self.total_expressions_analyzed += 1;
    }
}
