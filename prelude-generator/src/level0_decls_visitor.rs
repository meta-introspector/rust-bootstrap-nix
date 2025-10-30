use syn::{visit::Visit, ItemConst, File};
use quote::quote;

pub struct Level0DeclsVisitor {
    pub constants: Vec<ItemConst>,
}

impl Level0DeclsVisitor {
    pub fn new() -> Self {
        Level0DeclsVisitor {
            constants: Vec::new(),
        }
    }

    pub fn extract_from_file(file: &File) -> Self {
        let mut visitor = Self::new();
        visitor.visit_file(file);
        visitor
    }
}

impl<'ast> Visit<'ast> for Level0DeclsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.constants.push(i.clone());
        // Continue traversal to find nested consts if any (though not typical for ItemConst)
        syn::visit::visit_item_const(self, i);
    }
}

// Example usage (for testing/demonstration purposes)
pub fn generate_constants_module(constants: &[ItemConst]) -> String {
    let generated_code = constants.iter().map(|c| {
        quote! { #c }
    }).collect::<Vec<_>>();

    quote! {
        // This module contains extracted Level 0 constant declarations.
        // It is automatically generated.

        #(#generated_code)*
    }.to_string()
}
