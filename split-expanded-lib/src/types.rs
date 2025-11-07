use std::path::PathBuf;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use syn::{self, ItemConst, ItemStruct, ItemEnum, ItemFn, ItemStatic, ItemMacro, ItemMod, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, Item};
use quote::ToTokens;
use anyhow::Context;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedManifest {
    pub rustc_version: String,
    pub rustc_host: String,
    pub project_root: PathBuf,
    pub expanded_files: Vec<ExpandedFileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedFileEntry {
    pub package_name: String,
    pub target_type: String,
    pub target_name: String,
    pub expanded_rs_path: PathBuf,
    pub cargo_expand_command: String,
    pub timestamp: u64,
    pub flake_lock_details: serde_json::Value,
    pub layer: u32,
}

#[derive(Debug, Default)]
pub struct FileMetadata {
    pub global_uses: HashSet<String>,
    pub feature_attributes: HashSet<String>,
    pub extern_crates: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeclarationItem {
    Const(String),
    Struct(String),
    Enum(String),
    Fn(String),
    Static(String),
    Macro(String),
    Mod(String),
    Trait(String),
    TraitAlias(String),
    Type(String),
    Union(String),
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declaration {
    pub item: DeclarationItem,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub source_file: PathBuf,
    pub crate_name: String,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
    pub extern_crates: HashSet<String>,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableDeclaration {
    pub identifier: String,
    pub declaration_type: String,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub source_file: PathBuf,
    pub crate_name: String,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
    pub extern_crates: HashSet<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicSymbol {
    pub identifier: String,
    pub declaration_type: String,
    pub signature: String,
    pub source_file: PathBuf,
    pub crate_name: String,
}

impl From<Declaration> for SerializableDeclaration {
    fn from(decl: Declaration) -> Self {
        SerializableDeclaration {
            identifier: decl.get_identifier(),
            declaration_type: match &decl.item {
                DeclarationItem::Const(_) => "const".to_string(),
                DeclarationItem::Struct(_) => "struct".to_string(),
                DeclarationItem::Enum(_) => "enum".to_string(),
                DeclarationItem::Fn(_) => "function".to_string(),
                DeclarationItem::Static(_) => "static".to_string(),
                DeclarationItem::Macro(_) => "macro".to_string(),
                DeclarationItem::Mod(_) => "module".to_string(),
                DeclarationItem::Trait(_) => "trait".to_string(),
                DeclarationItem::TraitAlias(_) => "trait_alias".to_string(),
                DeclarationItem::Type(_) => "type_alias".to_string(),
                DeclarationItem::Union(_) => "union".to_string(),
                DeclarationItem::Other(_) => "other".to_string(),
            },
            referenced_types: decl.referenced_types,
            referenced_functions: decl.referenced_functions,
            external_identifiers: decl.external_identifiers,
            source_file: decl.source_file,
            crate_name: decl.crate_name,
            resolved_dependencies: decl.resolved_dependencies,
            is_proc_macro: decl.is_proc_macro,
            required_imports: decl.required_imports,
            direct_dependencies: decl.direct_dependencies,
            extern_crates: decl.extern_crates,
            is_public: decl.is_public,
        }
    }
}

impl Declaration {
    pub fn new(
        item: DeclarationItem,
        referenced_types: HashSet<String>,
        referenced_functions: HashSet<String>,
        external_identifiers: HashSet<String>,
        source_file: PathBuf,
        crate_name: String,
        is_proc_macro: bool,
        required_imports: HashSet<String>,
        extern_crates: HashSet<String>,
        is_public: bool,
    ) -> Self {
        Declaration {
            item,
            referenced_types,
            referenced_functions,
            external_identifiers,
            source_file,
            crate_name,
            resolved_dependencies: HashSet::new(),
            is_proc_macro,
            required_imports,
            direct_dependencies: HashSet::new(),
            extern_crates,
            is_public,
        }
    }

    pub fn get_identifier(&self) -> String {
        match &self.item {
            DeclarationItem::Const(s) => syn::parse_str::<ItemConst>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_const".to_string()),
            DeclarationItem::Struct(s) => syn::parse_str::<ItemStruct>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_struct".to_string()),
            DeclarationItem::Enum(s) => syn::parse_str::<ItemEnum>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_enum".to_string()),
            DeclarationItem::Fn(s) => syn::parse_str::<ItemFn>(s).map(|item| item.sig.ident.to_string()).unwrap_or_else(|_| "unknown_fn".to_string()),
            DeclarationItem::Static(s) => syn::parse_str::<ItemStatic>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_static".to_string()),
            DeclarationItem::Macro(s) => syn::parse_str::<syn::ItemMacro>(s).map(|item| item.ident.as_ref().map_or_else(|| "unknown_macro".to_string(), |ident| ident.to_string())).unwrap_or_else(|_| "unknown_macro".to_string()),
            DeclarationItem::Mod(s) => syn::parse_str::<syn::ItemMod>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_mod".to_string()),
            DeclarationItem::Trait(s) => syn::parse_str::<syn::ItemTrait>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_trait".to_string()),
            DeclarationItem::TraitAlias(s) => syn::parse_str::<syn::ItemTraitAlias>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_trait_alias".to_string()),
            DeclarationItem::Type(s) => syn::parse_str::<syn::ItemType>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_type".to_string()),
            DeclarationItem::Union(s) => syn::parse_str::<syn::ItemUnion>(s).map(|item| item.ident.to_string()).unwrap_or_else(|_| "unknown_union".to_string()),
            DeclarationItem::Other(s) => syn::parse_str::<syn::Item>(s).map(|item| {
                match item {
                    syn::Item::Const(item_const) => item_const.ident.to_string(),
                    syn::Item::Enum(item_enum) => item_enum.ident.to_string(),
                    syn::Item::Fn(item_fn) => item_fn.sig.ident.to_string(),
                    syn::Item::Macro(item_macro) => item_macro.ident.as_ref().map_or_else(|| "unknown_macro".to_string(), |ident| ident.to_string()),
                    syn::Item::Mod(item_mod) => item_mod.ident.to_string(),
                    syn::Item::Static(item_static) => item_static.ident.to_string(),
                    syn::Item::Struct(item_struct) => item_struct.ident.to_string(),
                    syn::Item::Trait(item_trait) => item_trait.ident.to_string(),
                    syn::Item::TraitAlias(item_trait_alias) => item_trait_alias.ident.to_string(),
                    syn::Item::Type(item_type) => item_type.ident.to_string(),
                    syn::Item::Union(item_union) => item_union.ident.to_string(),
                    _ => "unknown_other_item".to_string(),
                }
            }).unwrap_or_else(|_| "unknown_other_item".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorSample {
    pub file_path: PathBuf,
    pub rustc_version: String,
    pub rustc_host: String,
    pub error_message: String,
    pub error_type: String,
    pub code_snippet: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustcInfo {
    pub version: String,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    pub id: String,
    pub dependency_type: String,
    pub crate_name: String,
    pub module_path: String,
    pub usage_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorCollection {
    pub errors: Vec<ErrorSample>,
}

impl ErrorCollection {
    pub fn new() -> Self {
        ErrorCollection {
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ErrorSample) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub async fn write_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let json_content = serde_json::to_string_pretty(&self.errors)
            .context("Failed to serialize error collection to JSON")?;
        tokio::fs::write(path, json_content).await
            .context(format!("Failed to write error collection to file: {:?}", path))?;
        Ok(())
    }
}