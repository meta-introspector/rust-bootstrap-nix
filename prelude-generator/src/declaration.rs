use syn::{ItemConst, ItemStruct, ItemEnum, ItemFn, ItemStatic};
use std::collections::HashSet;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use quote::quote; // For converting syn::Item to String

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeclarationItem {
    Const(String),
    Struct(String),
    Enum(String),
    Fn(String),
    Static(String),
    Other(String),
    Macro(String),
    Mod(String),
    Trait(String),
    TraitAlias(String),
    Type(String),
    Union(String),
}

impl DeclarationItem {
    pub fn get_name(&self) -> String {
        match self {
            DeclarationItem::Const(s) => syn::parse_str::<ItemConst>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "UnknownConst".to_string()),
            DeclarationItem::Struct(s) => syn::parse_str::<ItemStruct>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "UnknownStruct".to_string()),
            DeclarationItem::Enum(s) => syn::parse_str::<ItemEnum>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "UnknownEnum".to_string()),
            DeclarationItem::Fn(s) => syn::parse_str::<ItemFn>(s).map(|item| item.sig.ident.to_string()).unwrap_or_else(|_| "UnknownFn".to_string()),
            DeclarationItem::Static(s) => syn::parse_str::<ItemStatic>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "UnknownStatic".to_string()),
            DeclarationItem::Other(s) => s.clone(),
            DeclarationItem::Macro(s) => s.clone(),
            DeclarationItem::Mod(s) => s.clone(),
            DeclarationItem::Trait(s) => s.clone(),
            DeclarationItem::TraitAlias(s) => s.clone(),
            DeclarationItem::Type(s) => s.clone(),
            DeclarationItem::Union(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declaration {
    pub item: DeclarationItem,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub gem_identifiers: HashSet<String>,
    pub source_file: Option<PathBuf>,
    pub crate_name: Option<String>,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
    pub extern_crates: HashSet<String>,
}

impl Declaration {
    pub fn new(
        item: DeclarationItem,
        referenced_types: HashSet<String>,
        referenced_functions: HashSet<String>,
        external_identifiers: HashSet<String>,
        gem_identifiers: HashSet<String>,
        source_file: Option<PathBuf>,
        crate_name: Option<String>,
        is_proc_macro: bool,
        required_imports: HashSet<String>,
        extern_crates: HashSet<String>,
    ) -> Self {
        Declaration {
            item,
            referenced_types,
            referenced_functions,
            external_identifiers,
            gem_identifiers,
            source_file,
            crate_name,
            resolved_dependencies: HashSet::new(), // Will be populated later
            is_proc_macro,
            required_imports,
            direct_dependencies: HashSet::new(), // Will be populated later
            extern_crates,
        }
    }

    pub fn get_identifier(&self) -> String {
        self.item.get_name()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableDeclaration {
    pub identifier: String,
    pub declaration_type: String,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub source_file: Option<PathBuf>,
    pub crate_name: Option<String>,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
    pub extern_crates: HashSet<String>,
}
