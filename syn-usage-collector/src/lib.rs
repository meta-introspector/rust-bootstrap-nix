use syn::{visit::{self, Visit}, File, Type};
use std::collections::{HashMap, HashSet};
use anyhow::Result;

#[derive(Debug, Default)]
pub struct TypeUsageCollector {
    /// Stores usage of types: TypeA -> AST Node Type -> Count of TypeB -> Set of TypeB groups
    pub type_usage: HashMap<String, HashMap<String, HashMap<usize, HashSet<HashSet<String>>>>>, 
    pub all_types: HashSet<String>,
}

impl<'ast> Visit<'ast> for TypeUsageCollector {
    fn visit_type(&mut self, i: &'ast Type) {
        // Collect all types encountered
        if let Some(type_name) = type_to_string(i) {
            self.all_types.insert(type_name);
        }
        visit::visit_type(self, i);
    }

    // TODO: Implement visit methods for other AST nodes (e.g., visit_expr, visit_item_fn, etc.)
    // to capture usage context and co-occurring types.
}

/// Helper function to convert a syn::Type to a String representation.
fn type_to_string(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(type_path) => type_path.path.segments.last().map(|segment| segment.ident.to_string()),
        // Add other Type variants as needed
        _ => None,
    }
}

pub fn analyze_file(file: &File) -> Result<TypeUsageCollector> {
    let mut collector = TypeUsageCollector::default();
    collector.visit_file(file);
    Ok(collector)
}