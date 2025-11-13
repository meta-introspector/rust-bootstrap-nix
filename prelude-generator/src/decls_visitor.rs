use syn::{visit::Visit, ItemConst, File, ItemFn, ItemStruct, ItemEnum, ItemStatic};
use std::collections::{HashSet, HashMap};
use crate::declaration::{Declaration, DeclarationItem};
use crate::gem_parser::GemConfig;

pub struct DeclsVisitor {
    pub declarations: Vec<Declaration>,
    pub fn_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub static_count: usize,
    pub other_item_count: usize,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub current_gem_identifiers: HashSet<String>, // Temporarily stores gem identifiers for the current declaration
    pub gem_identifiers: HashMap<String, String>, // Maps identifier to gem name
}

impl DeclsVisitor {
    pub fn new(gem_config: &GemConfig) -> Self {
        DeclsVisitor {
            declarations: Vec::new(),
            fn_count: 0,
            struct_count: 0,
            enum_count: 0,
            static_count: 0,
            other_item_count: 0,
            referenced_types: HashSet::new(),
            referenced_functions: HashSet::new(),
            external_identifiers: HashSet::new(),
            current_gem_identifiers: HashSet::new(),
            gem_identifiers: gem_config.get_identifier_to_gem_map(),
        }
    }

    pub fn extract_from_file(file: &File, gem_config: &GemConfig) -> Self {
        let mut visitor = Self::new(gem_config);
        visitor.visit_file(file);
        visitor
    }

    fn is_primitive_type(s: &str) -> bool {
        matches!(s, "bool" | "char" | "f32" | "f64" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize")
    }
}

impl<'ast> Visit<'ast> for DeclsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        let declaration = Declaration::new(
            DeclarationItem::Const(i.clone()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
        );
        self.declarations.push(declaration);
        // Clear collected identifiers for the next declaration
        self.referenced_types.clear();
        self.referenced_functions.clear();
        self.external_identifiers.clear();
        self.current_gem_identifiers.clear();
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.fn_count += 1;
        let declaration = Declaration::new(
            DeclarationItem::Fn(i.clone()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
        );
        self.declarations.push(declaration);
        // Clear collected identifiers for the next declaration
        self.referenced_types.clear();
        self.referenced_functions.clear();
        self.external_identifiers.clear();
        self.current_gem_identifiers.clear();
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*i.func {
            if let Some(segment) = expr_path.path.segments.last() {
                self.referenced_functions.insert(segment.ident.to_string());
            }
        }
        syn::visit::visit_expr_call(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.struct_count += 1;
        let declaration = Declaration::new(
            DeclarationItem::Struct(i.clone()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
        );
        self.declarations.push(declaration);
        // Clear collected identifiers for the next declaration
        self.referenced_types.clear();
        self.referenced_functions.clear();
        self.external_identifiers.clear();
        self.current_gem_identifiers.clear();
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.enum_count += 1;
        let declaration = Declaration::new(
            DeclarationItem::Enum(i.clone()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
        );
        self.declarations.push(declaration);
        // Clear collected identifiers for the next declaration
        self.referenced_types.clear();
        self.referenced_functions.clear();
        self.external_identifiers.clear();
        self.current_gem_identifiers.clear();
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.static_count += 1;
        let declaration = Declaration::new(
            DeclarationItem::Static(i.clone()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
        );
        self.declarations.push(declaration);
        // Clear collected identifiers for the next declaration
        self.referenced_types.clear();
        self.referenced_functions.clear();
        self.external_identifiers.clear();
        self.current_gem_identifiers.clear();
        syn::visit::visit_item_static(self, i);
    }

    fn visit_path(&mut self, i: &'ast syn::Path) {
        if let Some(segment) = i.segments.last() {
            let ident_str = segment.ident.to_string();
            // Heuristic: if it's not a primitive type, consider it a referenced type
            // This will need refinement with actual type resolution
            if !Self::is_primitive_type(&ident_str) {
                self.referenced_types.insert(ident_str.clone());
            }
            // Check if the identifier belongs to a gem
            if self.gem_identifiers.contains_key(&ident_str) {
                // We don't add gem identifiers to external_identifiers directly
                // They will be handled separately based on their gem category
            } else {
                // For now, all other paths are considered external identifiers until we have proper scope resolution
                self.external_identifiers.insert(ident_str);
            }
        }
        syn::visit::visit_path(self, i);
    }

    // Catch-all for other items not explicitly handled
    fn visit_item(&mut self, i: &'ast syn::Item) {
        match i {
            syn::Item::Const(_) | syn::Item::Fn(_) | syn::Item::Struct(_) | syn::Item::Enum(_) | syn::Item::Static(_) => {},
            _ => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Other(i.clone()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                );
                self.declarations.push(declaration);
                // Clear collected identifiers for the next declaration
                self.referenced_types.clear();
                self.referenced_functions.clear();
                self.external_identifiers.clear();
                self.current_gem_identifiers.clear();
            }
        }
        syn::visit::visit_item(self, i);
    }
}

