use syn::{self, Item, spanned::Spanned};
use quote::ToTokens;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct Declaration {
    pub decl_type: DeclarationType,
    pub name: String,
    pub span_start: usize,
    pub span_end: usize,
}

pub fn parse_declarations(code: &str) -> (Vec<Declaration>, HashMap<DeclarationType, usize>) {
    let mut declarations = Vec::new();
    let mut counts: HashMap<DeclarationType, usize> = HashMap::new();
    let syntax_tree = syn::parse_file(code).expect("Failed to parse Rust code");

    for item in syntax_tree.items {
        let (decl_type, name, span_start, span_end) = match item {
            Item::Fn(item_fn) => (DeclarationType::Function, item_fn.sig.ident.to_string(), item_fn.span().start().line, item_fn.span().end().line),
            Item::Struct(item_struct) => (DeclarationType::Struct, item_struct.ident.to_string(), item_struct.span().start().line, item_struct.span().end().line),
            Item::Enum(item_enum) => (DeclarationType::Enum, item_enum.ident.to_string(), item_enum.span().start().line, item_enum.span().end().line),
            Item::Trait(item_trait) => (DeclarationType::Trait, item_trait.ident.to_string(), item_trait.span().start().line, item_trait.span().end().line),
            Item::Mod(item_mod) => (DeclarationType::Module, item_mod.ident.to_string(), item_mod.span().start().line, item_mod.span().end().line),
            Item::Const(item_const) => (DeclarationType::Constant, item_const.ident.to_string(), item_const.span().start().line, item_const.span().end().line),
            Item::Static(item_static) => (DeclarationType::Static, item_static.ident.to_string(), item_static.span().start().line, item_static.span().end().line),
            Item::Macro(item_macro) => (DeclarationType::Macro, item_macro.mac.path.segments.last().map_or("unknown".to_string(), |s| s.ident.to_string()), item_macro.span().start().line, item_macro.span().end().line),
            Item::Use(item_use) => (DeclarationType::Use, item_use.to_token_stream().to_string(), item_use.span().start().line, item_use.span().end().line),
            Item::Impl(item_impl) => {
                let mut name = String::from("impl ");
                if let Some((_, path, _)) = &item_impl.trait_ {
                    name.push_str(&format!("{} for ", path.to_token_stream().to_string()));
                }
                name.push_str(&item_impl.self_ty.to_token_stream().to_string());
                (DeclarationType::Impl, name, item_impl.span().start().line, item_impl.span().end().line)
            },
            Item::ForeignMod(item_foreign_mod) => (DeclarationType::ForeignMod, quote::quote!(#item_foreign_mod).to_string(), item_foreign_mod.span().start().line, item_foreign_mod.span().end().line),
            Item::Type(item_type) => (DeclarationType::TypeAlias, item_type.ident.to_string(), item_type.span().start().line, item_type.span().end().line),
            Item::Verbatim(item_verbatim) => {
                let verbatim_str = item_verbatim.to_token_stream().to_string();
                if verbatim_str.starts_with("#!") || verbatim_str.starts_with("#[") {
                    (DeclarationType::Other, format!("Attribute: {}", &verbatim_str[..verbatim_str.find('(').unwrap_or(verbatim_str.len())]), item_verbatim.span().start().line, item_verbatim.span().end().line)
                } else {
                    (DeclarationType::Other, verbatim_str, item_verbatim.span().start().line, item_verbatim.span().end().line)
                }
            },
            _ => (DeclarationType::Other, "unknown".to_string(), item.span().start().line, item.span().end().line),
        };
        *counts.entry(decl_type.clone()).or_insert(0) += 1;
        declarations.push(Declaration { decl_type, name, span_start, span_end });
    }

    (declarations, counts)
}