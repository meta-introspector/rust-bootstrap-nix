extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, LitStr, Token};
use syn::braced;

struct ConfigInput {
    attrs: Vec<syn::Attribute>,
    ident: Ident,
    fields: syn::punctuated::Punctuated<ConfigField, Token![,]>, // Changed from ItemStruct to ConfigField
}

struct ConfigField {
    ident: Ident,
    ty: syn::Type,
    key: Option<LitStr>,
}

impl Parse for ConfigInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let struct_token = input.parse()?;
        let ident = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(ConfigField::parse, Token![,])?;
        Ok(ConfigInput {
            attrs,
            struct_token,
            ident,
            brace_token,
            fields,
        })
    }
}

impl Parse for ConfigField {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let ty = input.parse()?;
        let eq_token = input.parse().ok();
        let key = if eq_token.is_some() {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(ConfigField {
            ident,
            colon_token,
            ty,
            eq_token,
            key,
        })
    }
}

#[proc_macro]
pub fn define_config(input: TokenStream) -> TokenStream {
    let ConfigInput { attrs, ident, fields, .. } = parse_macro_input!(input as ConfigInput);

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_keys: Vec<_> = fields.iter().map(|f| {
        if let Some(key_lit) = &f.key {
            quote! { #key_lit }
        } else {
            let ident_str = LitStr::new(&f.ident.to_string(), f.ident.span());
            quote! { #ident_str }
        }
    }).collect();

    let expanded = quote! {
        #(#attrs)*
        pub struct #ident {
            #(pub #field_names: #field_types,)*
        }

        impl config_core::Merge for #ident {
            fn merge(&mut self, other: Self, replace: config_core::ReplaceOpt) {
                #( 
                    match replace {
                        config_core::ReplaceOpt::IgnoreDuplicate => {
                            if self.#field_names.is_none() {
                                self.#field_names = other.#field_names;
                            }
                        },
                        config_core::ReplaceOpt::Override => {
                            if other.#field_names.is_some() {
                                self.#field_names = other.#field_names;
                            }
                        }
                        config_core::ReplaceOpt::ErrorOnDuplicate => {
                            if other.#field_names.is_some() {
                                if self.#field_names.is_some() {
                                    if cfg!(test) {
                                        panic!("overriding existing option")
                                    } else {
                                        eprintln!("overriding existing option: `{}`", #field_keys);
                                        panic!("overriding existing option");
                                    }
                                }
                            } else {
                                self.#field_names = other.#field_names;
                            }
                        }
                    }
                )*
            }
        }
    };
    expanded.into()
}
