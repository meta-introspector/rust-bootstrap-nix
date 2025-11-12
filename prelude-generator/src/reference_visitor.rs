

use syn::visit;

use split_expanded_lib::{Declaration};
use crate::symbol_map::SymbolMap;

pub struct ReferenceVisitor<'a> {
    pub symbol_map: &'a mut SymbolMap,
    pub declarations: &'a mut Vec<Declaration>,
    pub crate_name: String,
    pub module_path: String,
    pub verbose: u8,
}

impl<'a> ReferenceVisitor<'a> {
    pub fn new(symbol_map: &'a mut SymbolMap, declarations: &'a mut Vec<Declaration>, crate_name: String, module_path: String, verbose: u8) -> Self {
        ReferenceVisitor {
            symbol_map,
            declarations,
            crate_name,
            module_path,
            verbose,
        }
    }
}

impl<'ast, 'a> visit::Visit<'ast> for ReferenceVisitor<'a> {
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*i.func {
            if let Some(segment) = expr_path.path.segments.last() {
                let resolved_dep = self.symbol_map.resolve_and_increment_usage(
                    segment.ident.to_string(),
                    "function".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );
                if self.verbose > 0 {
                    println!("Resolved Function Reference: id={}, type={}, crate={}, module={}, usage={}",
                             resolved_dep.id,
                             resolved_dep.dependency_type,
                             resolved_dep.crate_name,
                             resolved_dep.module_path,
                             resolved_dep.usage_count);
                }
                // TODO: Update the corresponding Declaration with this resolved_dep
            }
        }
        syn::visit::visit_expr_call(self, i);
    }

    fn visit_path(&mut self, i: &'ast syn::Path) {
        let ident_str = i.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
        // Basic heuristic to avoid primitive types
        if !ident_str.is_empty() && !matches!(ident_str.as_str(), "bool" | "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "f32" | "f64" | "char" | "str" | "usize" | "isize") {
            let resolved_dep = self.symbol_map.resolve_and_increment_usage(
                ident_str.clone(),
                "type".to_string(),
                self.crate_name.clone(),
                self.module_path.clone(),
            );
            if self.verbose > 0 {
                println!("Resolved Type Reference: id={}, type={}, crate={}, module={}, usage={}",
                         resolved_dep.id,
                         resolved_dep.dependency_type,
                         resolved_dep.crate_name,
                         resolved_dep.module_path,
                         resolved_dep.usage_count);
            }
            // TODO: Update the corresponding Declaration with this resolved_dep
        }
        syn::visit::visit_path(self, i);
    }
}
