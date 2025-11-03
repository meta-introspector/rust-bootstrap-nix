use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use syn::{self, Item};
use quote::quote;

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

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }
    fs::create_dir_all(&output_dir)?;

    let mut declaration_count = 0;

    for entry in WalkDir::new(&input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("Processing file: {:?}", path);

            let file_content = fs::read_to_string(path)?;
            let syntax_tree = syn::parse_file(&file_content).expect("Failed to parse Rust file");

            for item in syntax_tree.items {
                let item_str = quote! { #item }.to_string();
                let item_name = get_item_name(&item).unwrap_or_else(|| format!("declaration_{}", declaration_count));

                let relative_path = path.strip_prefix(&input_dir).unwrap();
                let level_dir = output_dir.join(relative_path.parent().unwrap());
                let decl_dir = level_dir.join(&item_name);

                fs::create_dir_all(&decl_dir)?;
                fs::create_dir_all(decl_dir.join("src"))?;

                let cargo_toml_content = format!(
                    "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
                    item_name
                );
                fs::write(decl_dir.join("Cargo.toml"), cargo_toml_content)?;

                let lib_rs_content = format!(
                    "#![feature(panic_internals)]\n#![feature(print_internals)]\n
use prelude::*;

{}",
                    item_str
                );
                fs::write(decl_dir.join("src/lib.rs"), lib_rs_content)?;

                declaration_count += 1;
            }
        }
    }

    println!("Split {} declarations into separate crates.", declaration_count);

    Ok(())
}

fn get_item_name(item: &Item) -> Option<String> {
    match item {
        Item::Fn(item_fn) => Some(item_fn.sig.ident.to_string()),
        Item::Struct(item_struct) => Some(item_struct.ident.to_string()),
        Item::Enum(item_enum) => Some(item_enum.ident.to_string()),
        Item::Mod(item_mod) => Some(item_mod.ident.to_string()),
        _ => None,
    }
}