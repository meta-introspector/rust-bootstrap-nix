use std::collections::HashSet;
use syn::{self, visit::{self, Visit}, Type};
use quote::ToTokens;

pub struct TypeCollector<'a> {
    pub types: &'a mut HashSet<String>,
}

impl<'ast, 'a> Visit<'ast> for TypeCollector<'a> {
    fn visit_type(&mut self, i: &'ast Type) {
        let type_str = i.to_token_stream().to_string();

        // Heuristic to filter out primitive types and common standard library types
        let primitive_types = [
            "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize",
            "u8", "u16", "u32", "u64", "u128", "usize", "str", "String",
            // Common std types that are often not considered 'user-defined' for this analysis
            "Vec", "Option", "Result", "HashMap", "HashSet", "Box", "Arc", "Rc",
        ];

        // Only consider types that are not in our primitive/common std list
        if !primitive_types.contains(&type_str.as_str()) {
            match i {
                Type::Path(type_path) => {
                    // Extract the last segment of the path as the type name
                    if let Some(segment) = type_path.path.segments.last() {
                        self.types.insert(segment.ident.to_string());
                    }
                },
                // For other type variants, we might need more sophisticated logic
                // For now, we'll just insert the full string representation if it's not a primitive
                // This might include types like `&str`, `[T]`, `(A, B)`, etc.
                _ => {
                    self.types.insert(type_str);
                }
            }
        }
        visit::visit_type(self, i);
    }
}
