use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident; // Added

#[derive(Debug)]
pub struct GeneratedTrait {
    pub name: String,
    pub generics: Option<TokenStream>,
    pub where_clause: Option<TokenStream>,
    pub visibility: Option<TokenStream>,
    pub methods: Vec<GeneratedTraitMethod>,
    pub associated_types: Vec<GeneratedAssociatedType>,
    pub supertraits: Vec<TokenStream>,
    // ... other trait components
}

#[derive(Debug)]
pub struct GeneratedTraitMethod {
    pub name: String,
    pub signature: String, // Using string for simplicity for now
    pub generics: Option<TokenStream>,
    pub where_clause: Option<TokenStream>,
    pub visibility: Option<TokenStream>,
    // ... other method components
}

#[derive(Debug)]
pub struct GeneratedAssociatedType {
    pub name: String,
    pub bounds: Vec<TokenStream>,
    pub default: Option<TokenStream>,
}

impl ToTokens for GeneratedTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(&self.name, proc_macro2::Span::call_site());
        let generics = &self.generics;
        let where_clause = &self.where_clause;
        let visibility = self.visibility.as_ref().map_or_else(|| quote!{}, |v| quote!{#v});

        let methods = self.methods.iter().map(|m| {
            let sig: TokenStream = m.signature.parse().expect("Invalid method signature");
            quote! { #sig }
        });

        let associated_types = self.associated_types.iter().map(|at| {
            let at_name = Ident::new(&at.name, proc_macro2::Span::call_site());
            let bounds = &at.bounds;
            let default = &at.default;
            quote! {
                type #at_name: #(#bounds)* #default;
            }
        });

        let supertraits = &self.supertraits;
        let supertraits_tokens = if supertraits.is_empty() {
            quote!{}
        } else {
            quote! { : #(#supertraits),* }
        };

        tokens.extend(quote! {
            #visibility trait #name #generics #supertraits_tokens #where_clause {
                #(#associated_types)*
                #(#methods)*
            }
        });
    }
}

impl ToTokens for GeneratedTraitMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sig: TokenStream = self.signature.parse().expect("Invalid method signature");
        tokens.extend(quote! { #sig });
    }
}

pub mod generator;
pub use generator::generate_traits;
pub mod writer;
pub use writer::write_trait_to_file;