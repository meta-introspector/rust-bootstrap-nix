use std::env;
use std::fs;
use std::io::{self, Write};
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

    println!("Searching for dependency level directories in: {:?}", output_dir);
    for entry in WalkDir::new(&output_dir).min_depth(1).max_depth(1) {
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
                if let Ok(level) = dir_name.parse::<u32>() {
                    println!("      Is a dependency level directory: {}", level);
                    root_workspace_members.push(format!("{}", level));

                    for decl_entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                        let decl_path = decl_entry.path();
                        if decl_path.is_dir() {
                            let decl_name = decl_path.file_name().unwrap().to_string_lossy().to_string();

                            let cargo_toml_path = decl_path.join("Cargo.toml");
                            let cargo_toml_content = format!(
                                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nprelude = {{ path = \"../prelude\" }}\n",
                                decl_name
                            );
                            fs::write(&cargo_toml_path, cargo_toml_content)?;
                        }
                    }
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