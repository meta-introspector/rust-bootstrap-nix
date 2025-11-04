use syn::{visit::Visit, ItemConst, ItemFn, ItemStruct, ItemEnum, ItemStatic, Item};
use std::collections::HashSet;
use std::path::PathBuf;
use crate::declaration::{Declaration, DeclarationItem};

use crate::symbol_map::SymbolMap;
use quote::quote;
use syn::spanned::Spanned;

pub struct DeclsVisitor<'a> {
    pub declarations: Vec<Declaration>,
    pub fn_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub static_count: usize,
    pub other_item_count: usize,
    // pub gem_identifiers: HashMap<String, String>,
    pub source_file: Option<PathBuf>,
    pub crate_name: String,
    pub module_path: String,
    pub current_required_imports: HashSet<String>,
    pub current_extern_crates: HashSet<String>,
    pub current_attributes: Vec<String>,
    pub current_doc_comments: Vec<String>,
    pub symbol_map: &'a mut SymbolMap,
    pub verbose: u8,
}

impl<'a> DeclsVisitor<'a> {
    pub fn new(source_file: Option<PathBuf>, crate_name: String, module_path: String, symbol_map: &'a mut SymbolMap, verbose: u8) -> Self {
        DeclsVisitor {
            declarations: Vec::new(),
            fn_count: 0,
            struct_count: 0,
            enum_count: 0,
            static_count: 0,
            other_item_count: 0,
            // gem_identifiers: gem_config.get_identifier_to_gem_map(),
            source_file,
            crate_name,
            module_path,
            current_required_imports: HashSet::new(),
            current_extern_crates: HashSet::new(),
            current_attributes: Vec::new(),
            current_doc_comments: Vec::new(),
            symbol_map,
            verbose,
        }
    }


    fn is_primitive_type(s: &str) -> bool {
        matches!(s, "bool" | "char" | "f32" | "f64" | "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize")
    }

    // Helper to clear temporary sets after a declaration is processed
    fn clear_current_declaration_data(&mut self) {
        self.current_required_imports.clear();
        self.current_extern_crates.clear();
        self.current_attributes.clear();
        self.current_doc_comments.clear();
    }

    pub fn extract_span(span: &proc_macro2::Span) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        let start = span.start();
        let end = span.end();
        (Some((start.line, start.column)), Some((end.line, end.column)))
    }

    pub fn extract_attributes(attrs: &[syn::Attribute]) -> Vec<String> {
        attrs.iter().map(|attr| quote!{#attr}.to_string()).collect()
    }

    pub fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
        attrs.iter()
            .filter_map(|attr| {
                if attr.path().is_ident("doc") {
                    if let Ok(lit_str) = attr.parse_args::<syn::LitStr>() {
                        return Some(lit_str.value());
                    }
                }
                None
            })
            .collect()
    }
}

impl<'ast, 'a> Visit<'ast> for DeclsVisitor<'a> {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        let span_start = Some((i.span().start().line, i.span().start().column));
        let span_end = Some((i.span().end().line, i.span().end().column));
        let attributes = Self::extract_attributes(&i.attrs);
        let doc_comments = Self::extract_doc_comments(&i.attrs);

        self.symbol_map.add_declaration(
            i.ident.to_string(),
            "const".to_string(),
            self.crate_name.clone(),
            self.module_path.clone(),
        );

        let declaration = Declaration::new(
            DeclarationItem::Const(quote!{#i}.to_string()),
            HashSet::new(), // Referenced types will be populated in Pass 2
            HashSet::new(), // Referenced functions will be populated in Pass 2
            self.source_file.clone(),
            Some(self.crate_name.clone()),
            false, // is_proc_macro
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
            span_start,
            span_end,
            attributes,
            doc_comments,
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.fn_count += 1;
        let is_proc_macro = i.attrs.iter().any(|attr| attr.path().is_ident("proc_macro") || attr.path().is_ident("proc_macro_attribute") || attr.path().is_ident("proc_macro_derive"));
        let span_start = Some((i.span().start().line, i.span().start().column));
        let span_end = Some((i.span().end().line, i.span().end().column));
        let attributes = Self::extract_attributes(&i.attrs);
        let doc_comments = Self::extract_doc_comments(&i.attrs);

        self.symbol_map.add_declaration(
            i.sig.ident.to_string(),
            "function".to_string(),
            self.crate_name.clone(),
            self.module_path.clone(),
        );

        let declaration = Declaration::new(
            DeclarationItem::Fn(quote!{#i}.to_string()),
            HashSet::new(), // Referenced types will be populated in Pass 2
            HashSet::new(), // Referenced functions will be populated in Pass 2
            self.source_file.clone(),
            Some(self.crate_name.clone()),
            is_proc_macro,
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
            span_start,
            span_end,
            attributes,
            doc_comments,
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*i.func {
            if let Some(segment) = expr_path.path.segments.last() {
                if self.verbose > 0 {
                    println!("Resolved Function: id={}, type={}, crate={}, module={}",
                             segment.ident.to_string(),
                             "function",
                             self.crate_name,
                             self.module_path);
                }
            }
        }
        syn::visit::visit_expr_call(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.struct_count += 1;
        let is_proc_macro = i.attrs.iter().any(|attr| attr.path().is_ident("proc_macro") || attr.path().is_ident("proc_macro_attribute") || attr.path().is_ident("proc_macro_derive"));
        let span_start = Some((i.span().start().line, i.span().start().column));
        let span_end = Some((i.span().end().line, i.span().end().column));
        let attributes = Self::extract_attributes(&i.attrs);
        let doc_comments = Self::extract_doc_comments(&i.attrs);

        self.symbol_map.add_declaration(
            i.ident.to_string(),
            "struct".to_string(),
            self.crate_name.clone(),
            self.module_path.clone(),
        );

        let declaration = Declaration::new(
                            DeclarationItem::Struct(quote!{#i}.to_string()),
                            HashSet::new(), // Referenced types will be populated in Pass 2
                            HashSet::new(), // Referenced functions will be populated in Pass 2
                            self.source_file.clone(),            Some(self.crate_name.clone()),
            is_proc_macro,
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
            span_start,
            span_end,
            attributes,
            doc_comments,
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.enum_count += 1;
        let span_start = Some((i.span().start().line, i.span().start().column));
        let span_end = Some((i.span().end().line, i.span().end().column));
        let attributes = Self::extract_attributes(&i.attrs);
        let doc_comments = Self::extract_doc_comments(&i.attrs);

        self.symbol_map.add_declaration(
            i.ident.to_string(),
            "enum".to_string(),
            self.crate_name.clone(),
            self.module_path.clone(),
        );

        let declaration = Declaration::new(
            DeclarationItem::Enum(quote!{#i}.to_string()),
            HashSet::new(), // Referenced types will be populated in Pass 2
            HashSet::new(), // Referenced functions will be populated in Pass 2
            self.source_file.clone(),
            Some(self.crate_name.clone()),
            false, // is_proc_macro
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
            span_start,
            span_end,
            attributes,
            doc_comments,
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.static_count += 1;
        let span_start = Some((i.span().start().line, i.span().start().column));
        let span_end = Some((i.span().end().line, i.span().end().column));
        let attributes = Self::extract_attributes(&i.attrs);
        let doc_comments = Self::extract_doc_comments(&i.attrs);

        self.symbol_map.add_declaration(
            i.ident.to_string(),
            "static".to_string(),
            self.crate_name.clone(),
            self.module_path.clone(),
        );

        let declaration = Declaration::new(
            DeclarationItem::Static(quote!{#i}.to_string()),
            HashSet::new(), // Referenced types will be populated in Pass 2
            HashSet::new(), // Referenced functions will be populated in Pass 2
            self.source_file.clone(),
            Some(self.crate_name.clone()),
            false, // is_proc_macro
            self.current_required_imports.clone(),
            self.current_extern_crates.clone(),
            span_start,
            span_end,
            attributes,
            doc_comments,
        );
        self.declarations.push(declaration);
        self.clear_current_declaration_data();
        syn::visit::visit_item_static(self, i);
    }

    fn visit_path(&mut self, i: &'ast syn::Path) {
        if let Some(segment) = i.segments.last() {
            let ident_str = segment.ident.to_string();
            if !Self::is_primitive_type(&ident_str) {
                if self.verbose > 0 {
                    println!("Resolved Type: id={}, type={}, crate={}, module={}",
                             ident_str,
                             "type",
                             self.crate_name,
                             self.module_path);
                }
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
        let span_start = Some((i.span().start().line, i.span().start().column));
        let span_end = Some((i.span().end().line, i.span().end().column));
        let _attributes = Self::extract_attributes(match i {
            Item::Const(item) => &item.attrs,
            Item::Fn(item) => &item.attrs,
            Item::Struct(item) => &item.attrs,
            Item::Enum(item) => &item.attrs,
            Item::Static(item) => &item.attrs,
            Item::Macro(item) => &item.attrs,
            Item::Mod(item) => &item.attrs,
            Item::Trait(item) => &item.attrs,
            Item::TraitAlias(item) => &item.attrs,
            Item::Type(item) => &item.attrs,
            Item::Union(item) => &item.attrs,
            _ => &[], // Handle other cases if necessary
        });
        let _doc_comments = Self::extract_doc_comments(match i {
            Item::Const(item) => &item.attrs,
            Item::Fn(item) => &item.attrs,
            Item::Struct(item) => &item.attrs,
            Item::Enum(item) => &item.attrs,
            Item::Static(item) => &item.attrs,
            Item::Macro(item) => &item.attrs,
            Item::Mod(item) => &item.attrs,
            Item::Trait(item) => &item.attrs,
            Item::TraitAlias(item) => &item.attrs,
            Item::Type(item) => &item.attrs,
            Item::Union(item) => &item.attrs,
            _ => &[], // Handle other cases if necessary
        });

        match i {
            Item::Const(_) | Item::Fn(_) | Item::Struct(_) | Item::Enum(_) | Item::Static(_) => {
                // These are handled by their specific visit methods
            },
            Item::Macro(item_macro) => {
                self.other_item_count += 1;
                self.symbol_map.add_declaration(
                    item_macro.ident.as_ref().map_or_else(|| "unknown_macro".to_string(), |i| i.to_string()),
                    "macro".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );

                let declaration = Declaration::new(
                    DeclarationItem::Macro(quote!{#item_macro}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
//                    HashSet::new(), // external_identifiers
//                    HashSet::new(), // gem_identifiers
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Mod(item_mod) => {
                self.other_item_count += 1;
                self.symbol_map.add_declaration(
                    item_mod.ident.to_string(),
                    "module".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );

                let declaration = Declaration::new(
                    DeclarationItem::Mod(quote!{#item_mod}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
//                    HashSet::new(), // external_identifiers
//                    HashSet::new(), // gem_identifiers
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Trait(item_trait) => {
                self.other_item_count += 1;
                self.symbol_map.add_declaration(
                    item_trait.ident.to_string(),
                    "trait".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );

                let declaration = Declaration::new(
                    DeclarationItem::Trait(quote!{#item_trait}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
//                    HashSet::new(), // external_identifiers
//                    HashSet::new(), // gem_identifiers
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::TraitAlias(item_trait_alias) => {
                self.other_item_count += 1;
                self.symbol_map.add_declaration(
                    item_trait_alias.ident.to_string(),
                    "trait_alias".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );

                let declaration = Declaration::new(
                    DeclarationItem::TraitAlias(quote!{#item_trait_alias}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Type(item_type) => {
                self.other_item_count += 1;
                self.symbol_map.add_declaration(
                    item_type.ident.to_string(),
                    "type_alias".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );

                let declaration = Declaration::new(
                    DeclarationItem::Type(quote!{#item_type}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            Item::Union(item_union) => {
                self.symbol_map.add_declaration(
                    item_union.ident.to_string(),
                    "union".to_string(),
                    self.crate_name.clone(),
                    self.module_path.clone(),
                );

                let declaration = Declaration::new(
                    DeclarationItem::Union(quote!{#item_union}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            },
            _ => {
                self.other_item_count += 1;
                let declaration = Declaration::new(
                    DeclarationItem::Other(quote!{#i}.to_string()),
                    HashSet::new(), // Referenced types will be populated in Pass 2
                    HashSet::new(), // Referenced functions will be populated in Pass 2
                    self.source_file.clone(),
                    Some(self.crate_name.clone()),
                    false, // is_proc_macro
                    self.current_required_imports.clone(),
                    self.current_extern_crates.clone(),
                    span_start,
                    span_end,
                    self.current_attributes.clone(),
                    self.current_doc_comments.clone(),
                );
                self.declarations.push(declaration);
                self.clear_current_declaration_data();
            }
        }
        syn::visit::visit_item(self, i);
    }
}

