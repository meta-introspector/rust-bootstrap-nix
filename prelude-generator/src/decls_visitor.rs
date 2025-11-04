use syn::{visit::Visit, ItemConst, File, ItemFn, ItemStruct, ItemEnum, ItemStatic, Item};
use std::collections::{HashSet, HashMap};
use std::path::PathBuf; // Added for PathBuf
use crate::declaration::{Declaration, DeclarationItem};
use crate::gem_parser::GemConfig;
use quote::quote; // Added for converting syn::Item to String

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
    pub source_file: Option<PathBuf>, // New field
    pub crate_name: Option<String>,   // New field
    pub current_required_imports: HashSet<String>, // New field for imports specific to current declaration
    pub current_extern_crates: HashSet<String>, // New field for extern crates specific to current declaration
}

impl DeclsVisitor {
    pub fn new(gem_config: &GemConfig, source_file: Option<PathBuf>, crate_name: Option<String>) -> Self {
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
            source_file,
            crate_name,
            current_required_imports: HashSet::new(),
            current_extern_crates: HashSet::new(),
        }
    }

    pub fn extract_from_file(file: &File, gem_config: &GemConfig, source_file: Option<PathBuf>, crate_name: Option<String>) -> Self {
        let mut visitor = Self::new(gem_config, source_file, crate_name);
        visitor.visit_file(file);
        visitor
    }

    fn is_primitive_type(s: &str) -> bool {
        matches!(s, "bool" | "char" | "f32" | "f64" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize")
    }

    // Helper to clear temporary sets after a declaration is processed
    fn clear_current_declaration_data(&mut self) {
        self.referenced_types.clear();
        self.referenced_functions.clear();
        self.external_identifiers.clear();
        self.current_gem_identifiers.clear();
        self.current_required_imports.clear();
        self.current_extern_crates.clear();
    }
}

impl<'ast> Visit<'ast> for DeclsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        let declaration = Declaration::new(
            DeclarationItem::Const(quote!{#i}.to_string()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false, // is_proc_macro
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.fn_count += 1;
        let is_proc_macro = i.attrs.iter().any(|attr| attr.path().is_ident("proc_macro") || attr.path().is_ident("proc_macro_attribute") || attr.path().is_ident("proc_macro_derive"));
        let declaration = Declaration::new(
            DeclarationItem::Fn(quote!{#i}.to_string()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
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
        let is_proc_macro = i.attrs.iter().any(|attr| attr.path().is_ident("proc_macro") || attr.path().is_ident("proc_macro_attribute") || attr.path().is_ident("proc_macro_derive"));
        let declaration = Declaration::new(
            DeclarationItem::Struct(quote!{#i}.to_string()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.enum_count += 1;
        let declaration = Declaration::new(
            DeclarationItem::Enum(quote!{#i}.to_string()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false, // is_proc_macro
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.static_count += 1;
        let declaration = Declaration::new(
            DeclarationItem::Static(quote!{#i}.to_string()),
            self.referenced_types.clone(),
            self.referenced_functions.clone(),
            self.external_identifiers.clone(),
            self.current_gem_identifiers.clone(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false, // is_proc_macro
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_static(self, i);
    }

    fn visit_path(&mut self, i: &'ast syn::Path) {
        if let Some(segment) = i.segments.last() {
            let ident_str = segment.ident.to_string();
            if !Self::is_primitive_type(&ident_str) {
                self.referenced_types.insert(ident_str.clone());
            }
            if self.gem_identifiers.contains_key(&ident_str) {
                // Handled by gem_identifiers
            } else {
                self.external_identifiers.insert(ident_str);
            }
        }
        syn::visit::visit_path(self, i);
    }

    fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
        self.current_required_imports.insert(quote!{#i}.to_string());
        syn::visit::visit_item_use(self, i);
    }

    fn visit_item_extern_crate(&mut self, i: &'ast syn::ItemExternCrate) {
        self.current_extern_crates.insert(i.ident.to_string());
        syn::visit::visit_item_extern_crate(self, i);
    }

    // Catch-all for other items not explicitly handled
    fn visit_item(&mut self, i: &'ast syn::Item) {
        match i {
            Item::Const(_) | Item::Fn(_) | Item::Struct(_) | Item::Enum(_) | Item::Static(_) => {
                // These are handled by their specific visit methods
            },
            Item::Macro(item_macro) => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Macro(quote!{#item_macro}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    true, // Assuming all Item::Macro are proc_macros for now
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Mod(item_mod) => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Mod(quote!{#item_mod}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    false,
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Trait(item_trait) => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Trait(quote!{#item_trait}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    false,
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::TraitAlias(item_trait_alias) => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::TraitAlias(quote!{#item_trait_alias}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    false,
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Type(item_type) => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Type(quote!{#item_type}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    false,
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Union(item_union) => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Union(quote!{#item_union}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    false,
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            _ => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Other(quote!{#i}.to_string()),
                    self.referenced_types.clone(),
                    self.referenced_functions.clone(),
                    self.external_identifiers.clone(),
                    self.current_gem_identifiers.clone(),
                    self.source_file.clone(),
                    self.crate_name.clone(),
                    false,
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            }
        }
        syn::visit::visit_item(self, i);
    }
}

