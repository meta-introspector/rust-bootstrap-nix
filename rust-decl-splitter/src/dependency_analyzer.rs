use syn::{
    visit::{self, Visit},
    Ident,
    Path,
    Macro,
    Item,
};
use std::collections::HashSet;

pub struct DependencyCollector {
    pub dependencies: HashSet<String>,
}

impl DependencyCollector {
    pub fn new() -> Self {
        DependencyCollector {
            dependencies: HashSet::new(),
        }
    }

    fn add_dependency(&mut self, ident: &Ident) {
        self.dependencies.insert(ident.to_string());
    }
}

impl<'ast> Visit<'ast> for DependencyCollector {
    // Visit identifiers in paths (e.g., `std::collections::HashMap`)
    fn visit_path(&mut self, i: &'ast Path) {
        for segment in &i.segments {
            self.add_dependency(&segment.ident);
        }
        visit::visit_path(self, i);
    }

    // Visit macro calls (e.g., `println!`) and their paths
    fn visit_macro(&mut self, i: &'ast Macro) {
        for segment in &i.path.segments {
            self.add_dependency(&segment.ident);
        }
        visit::visit_macro(self, i);
    }

    // Visit all identifiers. This is a broad approach to catch all uses.
    fn visit_ident(&mut self, i: &'ast Ident) {
        self.add_dependency(i);
        // No need to call visit::visit_ident, as it's a leaf node for Ident
    }

    // We need to visit items to ensure we capture dependencies within them
    fn visit_item(&mut self, i: &'ast Item) {
        // This will recursively visit all parts of the item
        visit::visit_item(self, i);
    }
}

pub fn count_dependencies(item: &Item) -> usize {
    let mut collector = DependencyCollector::new();
    collector.visit_item(item);
    collector.dependencies.len()
}
