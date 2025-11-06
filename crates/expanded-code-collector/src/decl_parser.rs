#![allow(unused_imports)]
use syn::{spanned::Spanned, Attribute};
use quote::ToTokens;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use syn::visit::{Visit, visit_type, visit_item_mod, visit_item_fn, visit_item_struct, visit_item_enum, visit_item_trait, visit_item_const, visit_item_static, visit_item_macro, visit_item_use, visit_item_impl, visit_item_foreign_mod, visit_item_type};

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Clone)]
pub enum DeclarationType {
    Function,
    Struct,
    Enum,
    Trait,
    Module,
    Constant,
    Static,
    Macro,
    Use,
    Impl,
    ForeignMod,
    TypeAlias,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Declaration {
    pub decl_type: DeclarationType,
    pub name: String,
    pub span_start: usize,
    pub span_end: usize,
    pub level: usize,
    pub attributes: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TypeUsage {
    pub type_name: String,
    pub function_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub trait_count: usize,
    pub module_count: usize,
    pub constant_count: usize,
    pub static_count: usize,
    pub macro_count: usize,
    pub use_count: usize,
    pub impl_count: usize,
    pub foreign_mod_count: usize,
    pub type_alias_count: usize,
    pub other_count: usize,
}

#[derive(Debug)]
struct DeclarationCollector {
    pub declarations: Vec<Declaration>,
    pub type_usages: HashMap<String, TypeUsage>,
    pub nesting_matrix: HashMap<(DeclarationType, DeclarationType, usize), usize>, // New field
    current_level: usize,
    current_decl_type: Option<DeclarationType>,
    current_attributes: Vec<String>,
}

impl Default for DeclarationCollector {
    fn default() -> Self {
        DeclarationCollector {
            declarations: Vec::new(),
            type_usages: HashMap::new(),
            nesting_matrix: HashMap::new(),
            current_level: 0,
            current_decl_type: None,
            current_attributes: Vec::new(),
        }
    }
}

impl Visit<'_> for DeclarationCollector {
    fn visit_item_mod(&mut self, i: &'_ syn::ItemMod) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_level += 1;
        self.current_decl_type = Some(DeclarationType::Module);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Module,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level - 1, // Module itself is at current_level - 1
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Module, self.current_level - 1)).or_insert(0) += 1;
        }
        visit_item_mod(self, i);
        self.current_level -= 1;
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_fn(&mut self, i: &'_ syn::ItemFn) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Function);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Function,
            name: i.sig.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Function, self.current_level)).or_insert(0) += 1;
        }
        visit_item_fn(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_struct(&mut self, i: &'_ syn::ItemStruct) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Struct);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Struct,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Struct, self.current_level)).or_insert(0) += 1;
        }
        visit_item_struct(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_enum(&mut self, i: &'_ syn::ItemEnum) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Enum);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Enum,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Enum, self.current_level)).or_insert(0) += 1;
        }
        visit_item_enum(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_trait(&mut self, i: &'_ syn::ItemTrait) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Trait);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Trait,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Trait, self.current_level)).or_insert(0) += 1;
        }
        visit_item_trait(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_const(&mut self, i: &'_ syn::ItemConst) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Constant);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Constant,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Constant, self.current_level)).or_insert(0) += 1;
        }
        visit_item_const(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_static(&mut self, i: &'_ syn::ItemStatic) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Static);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Static,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Static, self.current_level)).or_insert(0) += 1;
        }
        visit_item_static(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_macro(&mut self, i: &'_ syn::ItemMacro) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Macro);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Macro,
            name: i.mac.path.segments.last().map_or("unknown".to_string(), |s| s.ident.to_string()),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Macro, self.current_level)).or_insert(0) += 1;
        }
        visit_item_macro(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_use(&mut self, i: &'_ syn::ItemUse) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Use);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Use,
            name: i.to_token_stream().to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Use, self.current_level)).or_insert(0) += 1;
        }
        visit_item_use(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_impl(&mut self, i: &'_ syn::ItemImpl) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::Impl);
        let mut name = String::from("impl ");
        if let Some((_, path, _)) = &i.trait_ {
            name.push_str(&format!("{} for ", path.to_token_stream().to_string()));
        }
        name.push_str(&i.self_ty.to_token_stream().to_string());
        self.declarations.push(Declaration {
            decl_type: DeclarationType::Impl,
            name,
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::Impl, self.current_level)).or_insert(0) += 1;
        }
        visit_item_impl(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_foreign_mod(&mut self, i: &'_ syn::ItemForeignMod) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::ForeignMod);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::ForeignMod,
            name: quote::quote!(#i).to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::ForeignMod, self.current_level)).or_insert(0) += 1;
        }
        visit_item_foreign_mod(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_item_type(&mut self, i: &'_ syn::ItemType) {
        let parent_decl_type = self.current_decl_type.clone();
        let item_attributes = extract_attributes(&i.attrs);
        self.current_attributes.extend(item_attributes.clone());

        self.current_decl_type = Some(DeclarationType::TypeAlias);
        self.declarations.push(Declaration {
            decl_type: DeclarationType::TypeAlias,
            name: i.ident.to_string(),
            span_start: i.span().start().line,
            span_end: i.span().end().line,
            level: self.current_level,
            attributes: self.current_attributes.clone(),
        });
        if let Some(p_type) = parent_decl_type {
            *self.nesting_matrix.entry((p_type, DeclarationType::TypeAlias, self.current_level)).or_insert(0) += 1;
        }
        visit_item_type(self, i);
        self.current_decl_type = None;
        self.current_attributes.retain(|attr| !item_attributes.contains(attr));
    }

    fn visit_type(&mut self, i: &'_ syn::Type) {
        let type_name = quote::quote!(#i).to_string();
        let entry = self.type_usages.entry(type_name.clone()).or_insert_with(|| TypeUsage { type_name, ..Default::default() });

        if let Some(decl_type) = &self.current_decl_type {
            match decl_type {
                DeclarationType::Function => entry.function_count += 1,
                DeclarationType::Struct => entry.struct_count += 1,
                DeclarationType::Enum => entry.enum_count += 1,
                DeclarationType::Trait => entry.trait_count += 1,
                DeclarationType::Module => entry.module_count += 1,
                DeclarationType::Constant => entry.constant_count += 1,
                DeclarationType::Static => entry.static_count += 1,
                DeclarationType::Macro => entry.macro_count += 1,
                DeclarationType::Use => entry.use_count += 1,
                DeclarationType::Impl => entry.impl_count += 1,
                DeclarationType::ForeignMod => entry.foreign_mod_count += 1,
                DeclarationType::TypeAlias => entry.type_alias_count += 1,
                DeclarationType::Other => entry.other_count += 1,
            }
        }
        visit_type(self, i);
    }
}

fn extract_attributes(attrs: &[syn::Attribute]) -> Vec<String> {
    attrs.iter()
        .map(|attr| attr.path().to_token_stream().to_string())
        .collect()
}

pub fn parse_declarations(code: &str) -> (Vec<Declaration>, HashMap<DeclarationType, usize>, HashMap<String, TypeUsage>, HashMap<(DeclarationType, DeclarationType, usize), usize>) {
    // Debug print the code that is being parsed
    eprintln!("Attempting to parse code:\n---\n{}\n---", code);
    let syntax_tree = match syn::parse_file(code) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Error parsing Rust code: {}", e);
            eprintln!("Problematic code:\n---\n{}\n---", code);
            return (Vec::new(), HashMap::new(), HashMap::new(), HashMap::new());
        }
    };

    let crate_attributes: Vec<String> = syntax_tree.attrs.iter()
        .filter(|attr: &&syn::Attribute| matches!(attr.style, syn::AttrStyle::Inner(_)))
        .map(|attr| attr.path().to_token_stream().to_string())
        .collect();

    let mut collector = DeclarationCollector {
        current_attributes: crate_attributes,
        ..Default::default()
    };
    collector.visit_file(&syntax_tree);

    let mut counts: HashMap<DeclarationType, usize> = HashMap::new();
    for decl in &collector.declarations {
        *counts.entry(decl.decl_type.clone()).or_insert(0) += 1;
    }

    (collector.declarations, counts, collector.type_usages, collector.nesting_matrix)
}