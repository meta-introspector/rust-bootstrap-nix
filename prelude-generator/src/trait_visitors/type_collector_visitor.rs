use syn::visit::Visit;
use std::collections::HashSet;
use crate::trait_visitors::vernacular_walk::VernacularWalk;

#[derive(Debug, Default)]
pub struct TypeCollectorVisitor {
    pub collected_types: HashSet<String>,
}

impl<'ast> Visit<'ast> for TypeCollectorVisitor {
    fn visit_path(&mut self, i: &'ast syn::Path) {
        // Collect path segments as types
        self.collected_types.insert(quote::quote!(#i).to_string());
        self.walk_path(i);
    }

    fn visit_type(&mut self, i: &'ast syn::Type) {
        self.collected_types.insert(quote::quote!(#i).to_string());
        match i {
            syn::Type::Array(type_array) => {
                self.visit_type(&type_array.elem);
                self.walk_type(i);
            }
            syn::Type::BareFn(type_bare_fn) => {
                self.walk_bare_fn(type_bare_fn); // Changed
                self.walk_type(i);
            }
            syn::Type::Group(type_group) => {
                self.visit_type(&type_group.elem);
                self.walk_type(i);
            }
            syn::Type::ImplTrait(type_impl_trait) => {
                for bound in &type_impl_trait.bounds {
                    self.walk_type_param_bound(bound); // Changed
                }
                self.walk_type(i);
            }
            syn::Type::Infer(_) => {
                self.walk_type(i);
            }
            syn::Type::Macro(type_macro) => {
                self.walk_macro(&type_macro.mac); // Changed
                self.walk_type(i);
            }
            syn::Type::Never(_) => {
                self.walk_type(i);
            }
            syn::Type::Paren(type_paren) => {
                self.visit_type(&type_paren.elem);
                self.walk_type(i);
            }
            syn::Type::Path(type_path) => {
                self.walk_type_path(type_path); // Changed
                self.walk_type(i);
            }
            syn::Type::Ptr(type_ptr) => {
                self.visit_type(&type_ptr.elem);
                self.walk_type(i);
            }
            syn::Type::Reference(type_reference) => {
                self.visit_type(&type_reference.elem);
                self.walk_type(i);
            }
            syn::Type::Slice(type_slice) => {
                self.visit_type(&type_slice.elem);
                self.walk_type(i);
            }
            syn::Type::TraitObject(type_trait_object) => {
                for bound in &type_trait_object.bounds {
                    self.walk_type_param_bound(bound); // Changed
                }
                self.walk_type(i);
            }
            syn::Type::Tuple(type_tuple) => {
                for elem in &type_tuple.elems {
                    self.visit_type(elem);
                }
                self.walk_type(i);
            }
            syn::Type::Verbatim(_) => {
                self.walk_type(i);
            }
            _ => {
                // Handle unknown or custom types
                self.walk_type(i);
            }
        }
    }
}

impl<'ast> VernacularWalk<'ast> for TypeCollectorVisitor {
    fn walk_signature(&mut self, _i: &'ast syn::Signature) {}
    fn walk_block(&mut self, _i: &'ast syn::Block) {}
    fn walk_attribute(&mut self, _i: &'ast syn::Attribute) {}
    fn walk_fields(&mut self, _i: &'ast syn::Fields) {}
    fn walk_item_enum(&mut self, _i: &'ast syn::ItemEnum) {}
    fn walk_item_trait(&mut self, _i: &'ast syn::ItemTrait) {}
    fn walk_item_type(&mut self, _i: &'ast syn::ItemType) {}
    fn walk_item_union(&mut self, _i: &'ast syn::ItemUnion) {}
    fn walk_item_const(&mut self, _i: &'ast syn::ItemConst) {}
    fn walk_item_static(&mut self, _i: &'ast syn::ItemStatic) {}
    fn walk_item_macro(&mut self, _i: &'ast syn::ItemMacro) {}
    fn walk_item_mod(&mut self, _i: &'ast syn::ItemMod) {}
    fn walk_return_type(&mut self, _i: &'ast syn::ReturnType) {}
    fn walk_fn_arg(&mut self, _i: &'ast syn::FnArg) {}
    fn walk_path(&mut self, i: &'ast syn::Path) {
        syn::visit::visit_path(self, i);
    }
    fn walk_type(&mut self, i: &'ast syn::Type) {
        syn::visit::visit_type(self, i);
    }
    fn walk_bare_fn(&mut self, i: &'ast syn::TypeBareFn) { // Added
        syn::visit::visit_type_bare_fn(self, i);
    }
    fn walk_macro(&mut self, i: &'ast syn::Macro) { // Added
        syn::visit::visit_macro(self, i);
    }
    fn walk_type_path(&mut self, i: &'ast syn::TypePath) { // Added
        syn::visit::visit_type_path(self, i);
    }
    fn walk_type_param_bound(&mut self, i: &'ast syn::TypeParamBound) { // Added
        syn::visit::visit_type_param_bound(self, i);
    }
    fn walk_variant(&mut self, _i: &'ast syn::Variant) {} // Added
    fn walk_trait_item(&mut self, _i: &'ast syn::TraitItem) {} // Added
    fn walk_expr(&mut self, _i: &'ast syn::Expr) {} // Added
}
