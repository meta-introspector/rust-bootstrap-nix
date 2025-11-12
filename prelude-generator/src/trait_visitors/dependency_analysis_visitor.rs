use syn::{ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemType, ItemUnion, ItemConst, ItemStatic, ItemMacro, ItemMod, Signature, Block, Fields, Attribute, ReturnType, FnArg, Variant, TraitItem, Expr}; // Added Expr
use syn::visit::Visit;
use std::collections::HashSet;
use crate::trait_visitors::vernacular_walk::VernacularWalk;
use crate::trait_visitors::type_collector_visitor::TypeCollectorVisitor; // Added // Added

#[derive(Debug, Default)]
pub struct DependencyAnalysisVisitor {
    pub dependencies: HashSet<String>,
    pub types_used: HashSet<String>,
}

impl<'ast> Visit<'ast> for DependencyAnalysisVisitor {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        println!("Visiting function: {}", i.sig.ident);
        self.dependencies.insert(i.sig.ident.to_string()); // Add function name as a dependency
        self.visit_signature(&i.sig);
        self.visit_block(&i.block);
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
    }

    fn visit_signature(&mut self, i: &'ast Signature) {
        self.visit_return_type(&i.output);
        for input in &i.inputs {
            self.visit_fn_arg(input);
        }
        // TODO: Visit other components of Signature if needed
    }

    fn visit_return_type(&mut self, i: &'ast ReturnType) {
        if let ReturnType::Type(_, ty) = i {
            let mut type_collector = TypeCollectorVisitor::default();
            type_collector.visit_type(ty);
            self.types_used.extend(type_collector.collected_types);
        }
        self.walk_return_type(i);
    }

    fn visit_fn_arg(&mut self, i: &'ast FnArg) {
        if let FnArg::Typed(pat_type) = i {
            let mut type_collector = TypeCollectorVisitor::default();
            type_collector.visit_type(&pat_type.ty);
            self.types_used.extend(type_collector.collected_types);
        }
        self.walk_fn_arg(i);
    }

    fn visit_block(&mut self, i: &'ast Block) {
        self.walk_block(i);
    }

    fn visit_attribute(&mut self, i: &'ast Attribute) {
        self.walk_attribute(i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        println!("Visiting struct: {}", i.ident);
        self.dependencies.insert(i.ident.to_string()); // Add struct name as a dependency
        self.visit_fields(&i.fields);
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
    }

    fn visit_fields(&mut self, i: &'ast Fields) {
        self.walk_fields(i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        println!("Visiting enum: {}", i.ident);
        self.dependencies.insert(i.ident.to_string());
        for variant in &i.variants {
            self.visit_variant(variant);
        }
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_enum(i);
    }

    fn visit_variant(&mut self, i: &'ast Variant) { // Added
        self.visit_fields(&i.fields);
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_variant(i);
    }

    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        println!("Visiting trait: {}", i.ident);
        self.dependencies.insert(i.ident.to_string());
        for item in &i.items {
            self.visit_trait_item(item);
        }
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_trait(i);
    }

    fn visit_trait_item(&mut self, i: &'ast TraitItem) { // Added
        self.walk_trait_item(i);
    }

    fn visit_item_type(&mut self, i: &'ast ItemType) {
        println!("Visiting type alias: {}", i.ident);
        self.dependencies.insert(i.ident.to_string());
        self.visit_type(&i.ty);
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_type(i);
    }

    fn visit_item_union(&mut self, i: &'ast ItemUnion) {
        println!("Visiting union: {}", i.ident);
        self.dependencies.insert(i.ident.to_string());
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_union(i);
    }

    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        println!("Visiting const: {}", i.ident);
        self.dependencies.insert(i.ident.to_string());
        self.visit_type(&i.ty);
        self.visit_expr(&i.expr); // Added
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_const(i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        println!("Visiting static: {}", i.ident);
        self.dependencies.insert(i.ident.to_string());
        self.visit_type(&i.ty);
        self.visit_expr(&i.expr); // Added
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_static(i);
    }

    fn visit_expr(&mut self, i: &'ast Expr) { // Added
        self.walk_expr(i);
    }

    fn visit_item_macro(&mut self, i: &'ast ItemMacro) {
        println!("Visiting macro: {}", i.ident.as_ref().map_or("unnamed macro".to_string(), |ident| ident.to_string()));
        if let Some(ident) = i.ident.as_ref() {
            self.dependencies.insert(ident.to_string());
        }
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.walk_item_macro(i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        println!("Visiting module: {}", i.ident.to_string());
        self.dependencies.insert(i.ident.to_string());
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        if let Some((_, items)) = &i.content {
            for item in items {
                self.visit_item(item);
            }
        }
        self.walk_item_mod(i);
    }
}

impl<'ast> VernacularWalk<'ast> for DependencyAnalysisVisitor {
    fn walk_signature(&mut self, i: &'ast Signature) {
        syn::visit::visit_signature(self, i);
    }

    fn walk_block(&mut self, i: &'ast Block) {
        syn::visit::visit_block(self, i);
    }

    fn walk_attribute(&mut self, i: &'ast Attribute) {
        syn::visit::visit_attribute(self, i);
    }

    fn walk_fields(&mut self, i: &'ast Fields) {
        syn::visit::visit_fields(self, i);
    }

    fn walk_item_enum(&mut self, i: &'ast ItemEnum) {
        syn::visit::visit_item_enum(self, i);
    }

    fn walk_item_trait(&mut self, i: &'ast ItemTrait) {
        syn::visit::visit_item_trait(self, i);
    }

    fn walk_item_type(&mut self, i: &'ast ItemType) {
        syn::visit::visit_item_type(self, i);
    }

    fn walk_item_union(&mut self, i: &'ast ItemUnion) {
        syn::visit::visit_item_union(self, i);
    }

    fn walk_item_const(&mut self, i: &'ast ItemConst) {
        syn::visit::visit_item_const(self, i);
    }

    fn walk_item_static(&mut self, i: &'ast ItemStatic) {
        syn::visit::visit_item_static(self, i);
    }

    fn walk_item_macro(&mut self, i: &'ast ItemMacro) {
        syn::visit::visit_item_macro(self, i);
    }

    fn walk_item_mod(&mut self, i: &'ast ItemMod) {
        syn::visit::visit_item_mod(self, i);
    }
    fn walk_return_type(&mut self, i: &'ast ReturnType) {
        syn::visit::visit_return_type(self, i);
    }
    fn walk_fn_arg(&mut self, i: &'ast FnArg) {
        syn::visit::visit_fn_arg(self, i);
    }
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
    fn walk_variant(&mut self, i: &'ast syn::Variant) { // Added
        syn::visit::visit_variant(self, i);
    }
    fn walk_trait_item(&mut self, i: &'ast syn::TraitItem) { // Added
        syn::visit::visit_trait_item(self, i);
    }
    fn walk_expr(&mut self, i: &'ast syn::Expr) { // Added
        syn::visit::visit_expr(self, i);
    }
}

