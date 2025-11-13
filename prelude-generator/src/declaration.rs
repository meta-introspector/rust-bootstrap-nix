use syn::{ItemConst, ItemStruct, ItemEnum, ItemFn, ItemStatic};
use std::collections::HashSet;

#[derive(Debug)]
pub enum DeclarationItem {
    Const(ItemConst),
    Struct(ItemStruct),
    Enum(ItemEnum),
    Fn(ItemFn),
    Static(ItemStatic),
    Other(syn::Item),
}

#[derive(Debug)]
pub struct Declaration {
    pub item: DeclarationItem,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub gem_identifiers: HashSet<String>,
    // Add other metadata as needed, e.g., source file, layer, etc.
}

impl Declaration {
    pub fn new(
        item: DeclarationItem,
        referenced_types: HashSet<String>,
        referenced_functions: HashSet<String>,
        external_identifiers: HashSet<String>,
        gem_identifiers: HashSet<String>,
    ) -> Self {
        Declaration {
            item,
            referenced_types,
            referenced_functions,
            external_identifiers,
            gem_identifiers,
        }
    }

    pub fn get_identifier(&self) -> String {
        match &self.item {
            DeclarationItem::Const(item_const) => item_const.ident.to_string(),
            DeclarationItem::Struct(item_struct) => item_struct.ident.to_string(),
            DeclarationItem::Enum(item_enum) => item_enum.ident.to_string(),
            DeclarationItem::Fn(item_fn) => item_fn.sig.ident.to_string(),
            DeclarationItem::Static(item_static) => item_static.ident.to_string(),
            DeclarationItem::Other(item) => {
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
                    _ => "unknown_declaration".to_string(),
                }
            }
        }
    }
}
