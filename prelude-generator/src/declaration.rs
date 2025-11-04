use syn::{ItemConst, ItemStruct, ItemEnum, ItemFn, ItemStatic};
use std::collections::HashSet;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResolvedDependency {
    pub id: String,
    pub dependency_type: String, // e.g., "type", "function", "macro"
    pub crate_name: String,
    pub module_path: String, // e.g., "std::collections"
    pub usage_count: usize,
}

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
    pub referenced_types: HashSet<ResolvedDependency>,
    pub referenced_functions: HashSet<ResolvedDependency>,
//    pub external_identifiers: HashSet<String>,
//    pub gem_identifiers: HashSet<String>,
    pub source_file: Option<PathBuf>,
    pub crate_name: Option<String>,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
    pub extern_crates: HashSet<String>,
    pub span_start: Option<(usize, usize)>,
    pub span_end: Option<(usize, usize)>,
    pub attributes: Vec<String>,
    pub doc_comments: Vec<String>,
}

impl Declaration {
    pub fn new(
        item: DeclarationItem,
        referenced_types: HashSet<ResolvedDependency>,
        referenced_functions: HashSet<ResolvedDependency>,
//        external_identifiers: HashSet<String>,
//        gem_identifiers: HashSet<String>,
        source_file: Option<PathBuf>,
        crate_name: Option<String>,
        is_proc_macro: bool,
        required_imports: HashSet<String>,
        extern_crates: HashSet<String>,
        span_start: Option<(usize, usize)>,
        span_end: Option<(usize, usize)>,
        attributes: Vec<String>,
        doc_comments: Vec<String>,
    ) -> Self {
        Declaration {
            item,
            referenced_types,
            referenced_functions,
//            external_identifiers,
//            gem_identifiers,
            source_file,
            crate_name,
            resolved_dependencies: HashSet::new(), // Will be populated later
            is_proc_macro,
            required_imports,
            direct_dependencies: HashSet::new(), // Will be populated later
            extern_crates,
            span_start,
            span_end,
            attributes,
            doc_comments,
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
    pub referenced_types: HashSet<ResolvedDependency>,
    pub referenced_functions: HashSet<ResolvedDependency>,
//    pub external_identifiers: HashSet<String>,
    pub source_file: Option<PathBuf>,
    pub crate_name: Option<String>,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
    pub extern_crates: HashSet<String>,
    pub span_start: Option<(usize, usize)>,
    pub span_end: Option<(usize, usize)>,
    pub attributes: Vec<String>,
    pub doc_comments: Vec<String>,
}

impl From<Declaration> for SerializableDeclaration {
    fn from(decl: Declaration) -> Self {
        let declaration_type = match decl.item {
            DeclarationItem::Const(_) => "const".to_string(),
            DeclarationItem::Struct(_) => "struct".to_string(),
            DeclarationItem::Enum(_) => "enum".to_string(),
            DeclarationItem::Fn(_) => "fn".to_string(),
            DeclarationItem::Static(_) => "static".to_string(),
            DeclarationItem::Other(_) => "other".to_string(),
            DeclarationItem::Macro(_) => "macro".to_string(),
            DeclarationItem::Mod(_) => "mod".to_string(),
            DeclarationItem::Trait(_) => "trait".to_string(),
            DeclarationItem::TraitAlias(_) => "trait_alias".to_string(),
            DeclarationItem::Type(_) => "type".to_string(),
            DeclarationItem::Union(_) => "union".to_string(),
        };

        SerializableDeclaration {
            identifier: decl.get_identifier(),
            declaration_type,
            referenced_types: decl.referenced_types,
            referenced_functions: decl.referenced_functions,
//            external_identifiers: decl.external_identifiers,
            source_file: decl.source_file,
            crate_name: decl.crate_name,
            resolved_dependencies: decl.resolved_dependencies,
            is_proc_macro: decl.is_proc_macro,
            required_imports: decl.required_imports,
            direct_dependencies: decl.direct_dependencies,
            extern_crates: decl.extern_crates,
            span_start: decl.span_start,
            span_end: decl.span_end,
            attributes: decl.attributes,
            doc_comments: decl.doc_comments,
        }
    }
}
