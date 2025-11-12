use std::collections::{HashMap, BTreeSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImplLatticeInfo {
    pub impl_for_type: String,
    pub method_co_occurrences: HashMap<String, usize>,
    // Key: set of co-occurring method names, Value: count of co-occurrence
    pub total_expressions_analyzed: usize,
}

impl ImplLatticeInfo {
    pub fn new(impl_for_type: String) -> Self {
        ImplLatticeInfo {
            impl_for_type,
            method_co_occurrences: HashMap::new(),
            total_expressions_analyzed: 0,
        }
    }

    pub fn add_co_occurrence(&mut self, method_names: BTreeSet<String>) {
        let key = method_names.into_iter().collect::<Vec<String>>().join("::");
        *self.method_co_occurrences.entry(key).or_insert(0) += 1;
        self.total_expressions_analyzed += 1;
    }
}
