use std::env;
use std::fs;
use std::io::{self};
use std::path::PathBuf;
use walkdir::WalkDir;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_directory>", args[0]);
        return Ok(())
    }
    let output_dir = PathBuf::from(&args[1]);

    if !output_dir.is_dir() {
        eprintln!("Error: Output directory does not exist or is not a directory.");
        return Ok(())
    }

    let mut root_workspace_members = Vec::new();
    root_workspace_members.push("prelude".to_string());

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
                if dir_name.starts_with("types_") {
                    if let Ok(level) = dir_name["types_".len()..].parse::<u32>() {
                        println!("      Is a dependency level directory: {}", level);
                        root_workspace_members.push(format!("src/{}", dir_name));
                        
                        // Create Cargo.toml for the dependency level directory (e.g., 3/Cargo.toml) as a package
                        let package_cargo_toml_path = path.join("Cargo.toml");
                        let package_cargo_toml_content = format!(
                            "[package]\nname = \"pkg-{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../../prelude\" }}\nserde = {{ version = \"1.0\", features = [\"derive\"] }}\n",
                            dir_name
                        );
                        
                        fs::write(&package_cargo_toml_path, package_cargo_toml_content)?;
                        // Create src directory and lib.rs for the package
                        let src_dir = path.join("src");
                        fs::create_dir_all(&src_dir)?;
                        let lib_rs_path = src_dir.join("lib.rs");
                        let mut lib_rs_content = String::new();
                        
                                                            for decl_entry in WalkDir::new(&path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                                                                let decl_path = decl_entry.path();
                                                                if decl_path.is_file() && decl_path.extension().map_or(false, |ext| ext == "rs") {
                                                                    if let Some(decl_name) = decl_path.file_stem().and_then(|s| s.to_str()) {
                                                                        lib_rs_content.push_str(&format!("pub mod {};\n", decl_name));
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
