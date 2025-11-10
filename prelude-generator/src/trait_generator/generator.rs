use anyhow::{Context, Result};
use crate::types::{AllDeclarationsExtractionResult};
use crate::trait_generator::{GeneratedTrait, GeneratedTraitMethod};
use quote::quote;
use std::collections::HashSet;
use syn::{Ident, ItemStruct, Fields};

pub fn generate_traits(
    extraction_result: &AllDeclarationsExtractionResult,
) -> Result<Vec<GeneratedTrait>> {
    let mut generated_traits = Vec::new();

    for declaration in &extraction_result.declarations {
        let declaration_name = declaration.get_identifier();

        // Generate IsX trait
        let is_trait_name = format!("Is{}", declaration_name);
        let get_name_method_name_str = format!("get_{}_name", declaration_name.to_lowercase());
        let get_name_method_name = Ident::new(&get_name_method_name_str, proc_macro2::Span::call_site());

        let get_name_method_signature = quote! {
            fn #get_name_method_name(&self) -> &'static str;
        }.to_string();

        let is_trait_method = GeneratedTraitMethod {
            name: get_name_method_name_str,
            signature: get_name_method_signature,
            generics: None,
            where_clause: None,
            visibility: None,
        };

        let is_trait = GeneratedTrait {
            name: is_trait_name.clone(),
            generics: None,
            where_clause: None,
            visibility: Some(quote! { pub }.into()),
            methods: vec![is_trait_method],
            associated_types: Vec::new(),
            supertraits: Vec::new(),
        };
        generated_traits.push(is_trait);

        // Generate UsesD traits
        let mut all_dependencies = HashSet::new();
        all_dependencies.extend(declaration.referenced_types.iter().cloned());
        all_dependencies.extend(declaration.referenced_functions.iter().cloned());
        all_dependencies.extend(declaration.external_identifiers.iter().cloned());

        for dependency_name in all_dependencies {
            let uses_trait_name = format!("Uses{}", dependency_name);
            let get_dependency_method_name_str = format!("uses_{}", dependency_name.to_lowercase());
            let get_dependency_method_name = Ident::new(&get_dependency_method_name_str, proc_macro2::Span::call_site());

            let get_dependency_method_signature = quote! {
                fn #get_dependency_method_name(&self);
            }.to_string();

            let uses_trait_method = GeneratedTraitMethod {
                name: get_dependency_method_name_str,
                signature: get_dependency_method_signature,
                generics: None,
                where_clause: None,
                visibility: None,
            };

            let uses_trait = GeneratedTrait {
                name: uses_trait_name,
                generics: None,
                where_clause: None,
                visibility: Some(quote! { pub }.into()),
                methods: vec![uses_trait_method],
                associated_types: Vec::new(),
                supertraits: Vec::new(),
            };
            generated_traits.push(uses_trait);
        }

        // Check for clap-derived structs and generate traits
        if let split_expanded_lib::types::DeclarationItem::Struct(item_struct_str) = &declaration.item {
            let parsed_item_struct: syn::ItemStruct = syn::parse_str(item_struct_str)
                .context(format!("Failed to parse struct string: {}", item_struct_str))?;

            if is_clap_struct(&parsed_item_struct) {
                let clap_trait_name = format!("IsClapArgs{}", declaration_name);
                let mut clap_trait_methods = Vec::new();

                if let Fields::Named(fields) = &parsed_item_struct.fields {
                    for field in &fields.named {
                        if let Some(field_ident) = &field.ident {
                            let field_name_str = field_ident.to_string();
                            let method_name_str = format!("get_{}", field_name_str);
                            let method_name = Ident::new(&method_name_str, proc_macro2::Span::call_site());
                            let field_type = &field.ty;

                            let method_signature = quote! {
                                fn #method_name(&self) -> &#field_type;
                            }.to_string();

                            clap_trait_methods.push(GeneratedTraitMethod {
                                name: method_name_str,
                                signature: method_signature,
                                generics: None,
                                where_clause: None,
                                visibility: None,
                            });
                        }
                    }
                }

                let clap_trait = GeneratedTrait {
                    name: clap_trait_name,
                    generics: None,
                    where_clause: None,
                    visibility: Some(quote! { pub }.into()),
                    methods: clap_trait_methods,
                    associated_types: Vec::new(),
                    supertraits: Vec::new(),
                };
                generated_traits.push(clap_trait);
            }
        }
    }

    Ok(generated_traits)
}

fn is_clap_struct(item_struct: &ItemStruct) -> bool {
    for attr in &item_struct.attrs {
        if attr.path().is_ident("derive") {
            if let Ok(paths) = attr.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated) {
                for path in paths {
                    if path.is_ident("Parser") || path.is_ident("Args") {
                        return true;
                    }
                }
            }
        }
    }
    false
}
