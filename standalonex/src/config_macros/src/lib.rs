//pub mod prelude;
//use crate::prelude::*;
//use syn::Token;
use proc_macro :: TokenStream ;
use quote :: quote ;
use syn :: braced ;
use syn :: parse :: { Parse , ParseStream , Result } ;
use syn :: { parse_macro_input , Ident , LitStr , Token } ;

extern crate proc_macro;
struct ConfigInput {
    attrs: Vec<syn::Attribute>,
    ident: Ident,
    fields: syn::punctuated::Punctuated<ConfigField, Token![,]>,
}
struct ConfigField {
    ident: Ident,
    ty: syn::Type,
    key: Option<LitStr>,
}
impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<Token![struct]>()?;
        let ident = input.parse()?;
        let content;
        let _brace_token = braced!(content in input);
        let fields = content.parse_terminated(ConfigField::parse, Token![,])?;
        Ok(ConfigInput {
            attrs,
            ident,
            fields,
        })
    }
}
impl Parse for ConfigField {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;
        let key = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(ConfigField { ident, ty, key })
    }
}
#[proc_macro]
pub fn define_config(input: TokenStream) -> TokenStream {
    let ConfigInput { attrs, ident, fields, .. } = parse_macro_input!(
        input as ConfigInput
    );
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_keys: Vec<_> = fields
        .iter()
        .map(|f| {
            if let Some(key_lit) = &f.key {
                quote! {
                    # key_lit
                }
            } else {
                let ident_str = LitStr::new(&f.ident.to_string(), f.ident.span());
                quote! {
                    # ident_str
                }
            }
        })
        .collect();
    let expanded = quote! {
        # (# attrs) * pub struct # ident { # (pub # field_names : # field_types,) * }
        impl config_core::Merge for # ident { fn merge(& mut self, other : Self, replace
        : config_core::ReplaceOpt) { # (match replace {
        config_core::ReplaceOpt::IgnoreDuplicate => { if self.# field_names.is_none() {
        self.# field_names = other.# field_names; } }, config_core::ReplaceOpt::Override
        => { if other.# field_names.is_some() { self.# field_names = other.# field_names;
        } } config_core::ReplaceOpt::ErrorOnDuplicate => { if other.# field_names
        .is_some() { if self.# field_names.is_some() { if cfg!(test) {
        panic!("overriding existing option") } else {
        eprintln!("overriding existing option: `{}`", # field_keys);
        panic!("overriding existing option"); } } } else { self.# field_names = other.#
        field_names; } } }) * } }
    };
    expanded.into()
}
