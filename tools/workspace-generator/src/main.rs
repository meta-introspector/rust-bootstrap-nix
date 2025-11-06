use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use itertools::Itertools;
use serde_json;
use std::collections::{HashMap, HashSet};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_directory> [verbosity_level]", args[0]);
        return Ok(())
    }
    let output_dir = PathBuf::from(&args[1]);
    let verbosity: u8 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

    if verbosity >= 1 {
        println!("Workspace generator started with verbosity level: {}", verbosity);
    }

    if !output_dir.is_dir() {
        eprintln!("Error: Output directory does not exist or is not a directory.");
        return Ok(())
    }

    // Read dependencies.json
    let dependencies_json_path = output_dir.join("dependencies.json");
    let external_dependencies_by_level: HashMap<u32, HashSet<String>> = if dependencies_json_path.exists() {
        let json_content = fs::read_to_string(&dependencies_json_path)?;
        serde_json::from_str(&json_content).context("Failed to deserialize dependencies.json")?
    } else {
        if verbosity >= 1 {
            println!("Warning: dependencies.json not found at {}. Generating Cargo.toml files without external dependencies.", dependencies_json_path.display());
        }
        HashMap::new()
    };

    let mut root_workspace_members = Vec::new();

    let src_dir = output_dir.join("src");
    println!("Searching for dependency level directories in: {:?}/src", output_dir);
    for entry in WalkDir::new(&src_dir).min_depth(1).max_depth(1) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error walking directory: {}", e);
                continue;
            }
        };
        let path = entry.path();
        println!("Found entry: {:?}", path);

        if path.is_dir() {
            println!("  Is a directory.");
            if let Some(dir_name) = path.file_name().and_then(|s| s.to_str()) {
                println!("    Dir name: {}", dir_name);
                if dir_name.starts_with("level_") {
                    if let Ok(level) = dir_name["level_".len()..].parse::<u32>() {
                        println!("      Is a dependency level directory: {}", level);
                        root_workspace_members.push(format!("src/{}", dir_name));
                        
                        // Create Cargo.toml for the dependency level directory (e.g., 3/Cargo.toml) as a package
                        let package_cargo_toml_path = path.join("Cargo.toml");
                        let mut package_cargo_toml_content = String::new();
                        package_cargo_toml_content.push_str(&format!(
                            "[package]\nname = \"pkg-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
                            dir_name
                        ));

                        if let Some(deps) = external_dependencies_by_level.get(&level) {
                            if !deps.is_empty() {
                                package_cargo_toml_content.push_str("\n[dependencies]\n");
                                for dep in deps.iter().sorted() {
                                    // Hardcode versions for now, or get from a config
                                    let version = match dep.as_str() {
                                        "serde" => "1.0",
                                        _ => "0.1", // Default version for other dependencies
                                    };
                                    package_cargo_toml_content.push_str(&format!("{0} = \"{1}\"\n", dep, version));
                                }
                            }
                        }
                        
                        fs::write(&package_cargo_toml_path, package_cargo_toml_content)?;
                        // Create src directory and lib.rs for the package
                        let src_dir_for_package = path.join("src"); // This is level_XX/src
                        fs::create_dir_all(&src_dir_for_package)?;
                        let lib_rs_path = src_dir_for_package.join("lib.rs"); // This is level_XX/src/lib.rs
                        let mut lib_rs_content = String::new();

                        // Iterate through type directories (e.g., const, struct, fn) within the level_XX/src directory
                        for type_entry in WalkDir::new(&src_dir_for_package).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                            let type_path = type_entry.path(); // This is level_XX/src/const_t, level_XX/src/struct_t, etc.
                            if type_path.is_dir() {
                                if let Some(type_name_original) = type_path.file_name().and_then(|s| s.to_str()) {
                                    if type_name_original == "src" {
                                        if verbosity >= 2 {
                                            println!("  Skipping 'src' directory as a module.");
                                        }
                                        continue;
                                    }
                                    if verbosity >= 3 {
                                        println!("    Processing type directory: {:?}, original name: {}", type_path, type_name_original);
                                    }
                                    let type_name_base = type_name_original.trim_end_matches("_t");
                                    let type_name_for_mod = if type_name_base == "_" {
                                        "UNDERSCORE"
                                    } else {
                                        type_name_base
                                    };
                                    let type_name_with_suffix = format!("{}_t", type_name_for_mod);

                                    // Create mod.rs for the type directory
                                    let mod_rs_path = type_path.join("mod.rs");
                                    let mut mod_rs_content = String::new();

                                    // Iterate through individual declaration files within the type directory
                                    for decl_entry in WalkDir::new(&type_path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                                        let decl_path = decl_entry.path();
                                        if decl_path.is_file() && decl_path.extension().map_or(false, |ext| ext == "rs") {
                                            if let Some(decl_name) = decl_path.file_stem().and_then(|s| s.to_str()) {
                                                let final_decl_name = if decl_name == "mod" {
                                                    "r#mod"
                                                } else {
                                                    decl_name
                                                };
                                                mod_rs_content.push_str(&format!("pub mod {};\n", final_decl_name));
                                            }
                                        }
                                    }

                                    if !mod_rs_content.is_empty() {
                                        fs::write(&mod_rs_path, mod_rs_content)?;
                                        lib_rs_content.push_str(&format!("pub mod {};\n", type_name_with_suffix));
                                    } else if verbosity >= 2 {
                                        println!("  Skipping empty module for type: {}", type_name_original);
                                    }
                                }
                            }
                        }
                        fs::write(&lib_rs_path, lib_rs_content)?;                    }
                }
            }
        }
    }
    let root_cargo_toml_path = output_dir.join("Cargo.toml");
    let mut root_cargo_toml_content = String::new();
    root_cargo_toml_content.push_str("[workspace]\n");
    root_cargo_toml_content.push_str("members = [\n");
    root_cargo_toml_content.push_str(&root_workspace_members.iter().map(|s| format!("    \"{}\"", s)).join(",\n"));
    root_cargo_toml_content.push_str("\n]\n");
    fs::write(&root_cargo_toml_path, root_cargo_toml_content)?;
    
    println!("Workspace generated successfully.");
    
    Ok(())
}
