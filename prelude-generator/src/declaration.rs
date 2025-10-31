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
    // Add other metadata as needed, e.g., source file, layer, etc.
}

impl Declaration {
    pub fn new(
        item: DeclarationItem,
        referenced_types: HashSet<String>,
        referenced_functions: HashSet<String>,
        external_identifiers: HashSet<String>,
    ) -> Self {
        Declaration {
            item,
            referenced_types,
            referenced_functions,
            external_identifiers,
        }
    }
}
