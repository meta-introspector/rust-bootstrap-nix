use anyhow::Result;
use crate::types::{AllDeclarationsExtractionResult};
use crate::trait_generator::{GeneratedTrait, GeneratedTraitMethod};
use quote::quote;
use std::collections::HashSet;
use syn::Ident; // Add this import

pub fn generate_traits(
    extraction_result: &AllDeclarationsExtractionResult,
) -> Result<Vec<GeneratedTrait>> {
    let mut generated_traits = Vec::new();

    for declaration in &extraction_result.declarations {
        let declaration_name = declaration.get_identifier();

        // Generate IsX trait
        let is_trait_name = format!("Is{}", declaration_name);
        let get_name_method_name_str = format!("get_{}_name", declaration_name.to_lowercase());
        let get_name_method_name = Ident::new(&get_name_method_name_str, proc_macro2::Span::call_site()); // Create Ident

        let get_name_method_signature = quote! {
            fn #get_name_method_name(&self) -> &'static str; // Use Ident directly
        }.to_string();

        let is_trait_method = GeneratedTraitMethod {
            name: get_name_method_name_str, // Keep the string for the name field
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
            let get_dependency_method_name = Ident::new(&get_dependency_method_name_str, proc_macro2::Span::call_site()); // Create Ident

            let get_dependency_method_signature = quote! {
                fn #get_dependency_method_name(&self); // Use Ident directly
            }.to_string();

            let uses_trait_method = GeneratedTraitMethod {
                name: get_dependency_method_name_str, // Keep the string for the name field
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
    }

    Ok(generated_traits)
}
