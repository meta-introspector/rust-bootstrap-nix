use anyhow::{Result, Context};
use serde::Deserialize;
use std::path::PathBuf;
use quote::{quote, format_ident};
use proc_macro2::{TokenStream, Ident};
use syn::parse_str;

#[derive(Debug, Deserialize)]
struct Method {
    name: String,
    args: Vec<String>,
    return_type: String,
    generics: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Interface {
    name: String,
    methods: Vec<Method>,
}

#[derive(Debug, Deserialize)]
struct InterfacesConfig {
    interface: Vec<Interface>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let project_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let interfaces_toml_path = project_root.join("src/external_interfaces/interfaces.toml");
    let external_interfaces_dir = project_root.join("src/external_interfaces");

    let config_content = tokio::fs::read_to_string(&interfaces_toml_path).await
        .with_context(|| format!("Failed to read interfaces.toml: {}", interfaces_toml_path.display()))?;

    let interfaces_config: InterfacesConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse interfaces.toml: {}", interfaces_toml_path.display()))?;

    let interfaces = interfaces_config.interface;

    // Generate mod.rs content
    let mut mod_decls = Vec::new();
    let mut trait_defs = Vec::new();
    let mut gateway_fields = Vec::new();
    let mut gateway_defaults = Vec::new();

    for interface in &interfaces {
        let interface_name = format_ident!("{}", interface.name);
        let impl_module_name = format_ident!("{}_impl", interface.name.to_lowercase().replace("interface", ""));
        let impl_struct_name = format_ident!("{}", format!("{}Impl", interface.name));

        mod_decls.push(quote! { pub mod #impl_module_name; });

        let mut method_defs = Vec::new();
        let mut impl_method_defs = Vec::new();

        for method in &interface.methods {
            let method_name = format_ident!("{}", method.name);
            let args: Vec<TokenStream> = method.args.iter().map(|arg| parse_str(arg).unwrap()).collect();
            let return_type: TokenStream = parse_str(&method.return_type).unwrap();
            let generics: Vec<TokenStream> = method.generics.as_ref().unwrap_or(&vec![]).iter().map(|g| parse_str(g).unwrap()).collect();

            if method.return_type.starts_with("impl std::future::Future") {
                method_defs.push(quote! {
                    fn #method_name<#(#generics),*>(&self, #(#args),*) -> #return_type;
                });
                impl_method_defs.push(quote! {
                    async fn #method_name<#(#generics),*>(&self, #(#args),*) -> #return_type {
                        todo!("Implement #method_name for #interface_name");
                    }
                });
            } else {
                method_defs.push(quote! {
                    fn #method_name<#(#generics),*>(&self, #(#args),*) -> #return_type;
                });
                impl_method_defs.push(quote! {
                    fn #method_name<#(#generics),*>(&self, #(#args),*) -> #return_type {
                        todo!("Implement #method_name for #interface_name");
                    }
                });
            }
        }

        trait_defs.push(quote! {
            pub trait #interface_name {
                #(#method_defs)*
            }
        });

        let impl_rs_content = quote! {
            use super::#interface_name;
            use anyhow::Result;
            use std::path::PathBuf;
            use syn::File;
            use cargo_metadata::Metadata;

            pub struct #impl_struct_name;

            impl #interface_name for #impl_struct_name {
                #(#impl_method_defs)*
            }
        };

        let impl_rs_path = external_interfaces_dir.join(format!("{}.rs", impl_module_name));
        tokio::fs::write(&impl_rs_path, impl_rs_content.to_string().as_bytes()).await
            .with_context(|| format!("Failed to write generated {}.rs: {}", impl_module_name, impl_rs_path.display()))?;

        println!("Generated {}.rs at: {}", impl_module_name, impl_rs_path.display());

        let field_name = format_ident!("{}", interface.name.to_lowercase());
        gateway_fields.push(quote! { pub #field_name: Box<dyn #interface_name + Send + Sync>, });
        gateway_defaults.push(quote! { #field_name: Box::new(#impl_module_name::#impl_struct_name), });
    }

    let mod_rs_content = quote! {
        use anyhow::Result;
        use std::path::PathBuf;

        #(#mod_decls)*

        #(#trait_defs)*

        pub struct ExternalInterfaceGateway {
            #(#gateway_fields)*
        }

        impl Default for ExternalInterfaceGateway {
            fn default() -> Self {
                Self {
                    #(#gateway_defaults)*
                }
            }
        }
    };

    let mod_rs_path = external_interfaces_dir.join("mod.rs");
    tokio::fs::write(&mod_rs_path, mod_rs_content.to_string().as_bytes()).await
        .with_context(|| format!("Failed to write generated mod.rs: {}", mod_rs_path.display()))?;

    println!("Generated mod.rs at: {}", mod_rs_path.display());

    Ok(())
}