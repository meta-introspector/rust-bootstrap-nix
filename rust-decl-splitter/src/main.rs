pub mod prelude;
mod measurement;
use crate::prelude::*;
use measurement::{record_function_entry, record_function_exit};
use syn::parse_quote;
use serde_json;
fn to_snake_case(ident: &Ident) -> String {
    let mut s = String::new();
    for (i, char) in ident.to_string().chars().enumerate() {
        if char.is_uppercase() && i != 0 {
            s.push('_');
        }
        s.push(char.to_ascii_lowercase());
    }
    s
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_directory> <output_directory>", args[0]);
        return Ok(());
    }
    let input_dir = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);
    if !input_dir.is_dir() {
        eprintln!("Error: Input directory does not exist or is not a directory.");
        return Ok(());
    }
    fs::create_dir_all(&output_dir)?;
    for entry in WalkDir::new(&input_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("Processing file: {}", path.display());
            let content = fs::read_to_string(path)?;
            match syn::parse_file(&content) {
                Ok(file) => {
                    println!("Successfully parsed: {}", path.display());
                    let relative_path = path.strip_prefix(&input_dir).unwrap();
                    let output_file_dir = output_dir
                        .join(relative_path.parent().unwrap_or(Path::new("")));
                    fs::create_dir_all(&output_file_dir)?;
                    let mut original_file_content = String::new();
                    for item in file.items {
                        let (ident, item_code) = match item {
                            Item::Fn(mut item_fn) => {
                                let original_block = item_fn.block;
                                let fn_ident = item_fn.sig.ident.clone();
                                item_fn.block = parse_quote! {
                                    {
                                        measurement::record_function_entry(stringify!(#fn_ident));
                                        let __result = #original_block;
                                        measurement::record_function_exit(stringify!(#fn_ident));
                                        __result
                                    }
                                };
                                (
                                    fn_ident,
                                    quote! {
                                        #item_fn
                                    },
                                )
                            }
                            Item::Struct(item_struct) => {
                                (
                                    item_struct.ident.clone(),
                                    quote! {
                                        # item_struct
                                    },
                                )
                            }
                            Item::Enum(item_enum) => {
                                (
                                    item_enum.ident.clone(),
                                    quote! {
                                        # item_enum
                                    },
                                )
                            }
                            Item::Trait(item_trait) => {
                                (
                                    item_trait.ident.clone(),
                                    quote! {
                                        # item_trait
                                    },
                                )
                            }
                            Item::Mod(item_mod) => {
                                let original_item_mod = item_mod.clone();
                                if original_item_mod.content.is_some() {
                                    let mod_ident = original_item_mod.ident.clone();
                                    let mod_name_snake = to_snake_case(&mod_ident);
                                    let new_mod_file_path = output_file_dir
                                        .join(format!("{}.rs", mod_name_snake));
                                    let new_mod_content = quote! {
                                        # original_item_mod
                                    };
                                    fs::write(&new_mod_file_path, new_mod_content.to_string())?;
                                    original_file_content
                                        .push_str(&format!("pub mod {};\n", mod_name_snake));
                                    continue;
                                } else {
                                    (
                                        original_item_mod.ident.clone(),
                                        quote! {
                                            # original_item_mod
                                        },
                                    )
                                }
                            }
                            Item::Impl(item_impl) => {
                                let target_ident = match &*item_impl.self_ty {
                                    syn::Type::Path(type_path) => {
                                        type_path
                                            .path
                                            .segments
                                            .last()
                                            .map(|segment| segment.ident.clone())
                                            .unwrap_or_else(|| Ident::new("unknown", item_impl.span()))
                                    }
                                    _ => Ident::new("unknown", item_impl.span()),
                                };
                                let snake_case_name = to_snake_case(&target_ident);
                                let target_file_name = format!("{}.rs", snake_case_name);
                                let target_file_path = output_file_dir
                                    .join(&target_file_name);
                                println!(
                                    "  Moving impl block for {} to {}", target_ident,
                                    target_file_path.display()
                                );
                                let impl_code = quote! {
                                    # item_impl
                                };
                                let mut existing_content = fs::read_to_string(
                                        &target_file_path,
                                    )
                                    .unwrap_or_default();
                                existing_content
                                    .push_str(&format!("\n{}\n", impl_code.to_string()));
                                fs::write(&target_file_path, existing_content)?;
                                continue;
                            }
                            Item::Use(_item_use) => {
                                continue;
                            }
                            _ => {
                                original_file_content
                                    .push_str(&format!("{}\n", quote! { # item }));
                                continue;
                            }
                        };
                        let snake_case_name = to_snake_case(&ident);
                        let new_file_name = format!("{}.rs", snake_case_name);
                        let new_file_path = output_file_dir.join(&new_file_name);
                        println!(
                            "  Splitting {} into {}", ident, new_file_path.display()
                        );
                        let new_file_name = format!("{}.rs", snake_case_name);
                        let new_file_path = output_file_dir.join(&new_file_name);

                        let function_output_dir = output_file_dir.join(&snake_case_name); // Directory for this function
                        fs::create_dir_all(&function_output_dir)?;

                        let rollup_data_dir = function_output_dir.join("rollup_data"); // rollup_data inside function dir
                        fs::create_dir_all(&rollup_data_dir)?;

                        let wrapped_code_path = rollup_data_dir.join("wrapped_code.rs");

                        let mut new_file_content = String::new();
                        new_file_content.push_str("use prelude::*;\n");
                        new_file_content.push_str(&item_code.to_string());
                        fs::write(&wrapped_code_path, new_file_content)?; // Save as wrapped_code.rs

                        // The original file now just re-exports from the function's directory
                        fs::write(&new_file_path, format!("pub use {}::{};\n", snake_case_name, ident))?;
                        original_file_content
                            .push_str(
                                &format!("pub use {}::{};\n", snake_case_name, ident),
                            );
                    }
                    let original_output_path = output_dir.join(relative_path);
                    fs::write(&original_output_path, original_file_content)?;
                }
                Err(e) => {
                    eprintln!("Error parsing {}: {}", path.display(), e);
                }
            }
        }
    }
    Ok(())
}
