use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_directory>", args[0]);
        return Ok(());
    }
    let output_dir = PathBuf::from(&args[1]);

    if !output_dir.is_dir() {
        eprintln!("Error: Output directory does not exist or is not a directory.");
        return Ok(());
    }

    let mut root_workspace_members = Vec::new();

    for entry in WalkDir::new(&output_dir).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            if let Ok(level) = path.file_name().unwrap().to_string_lossy().parse::<u32>() {
                root_workspace_members.push(level.to_string());

                let mut level_workspace_members = Vec::new();
                for decl_entry in WalkDir::new(path).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
                    let decl_path = decl_entry.path();
                    if decl_path.is_dir() {
                        let decl_name = decl_path.file_name().unwrap().to_string_lossy().to_string();
                        level_workspace_members.push(decl_name.clone());

                        let cargo_toml_path = decl_path.join("Cargo.toml");
                        let mut cargo_toml_file = fs::File::create(&cargo_toml_path)?;
                        writeln!(
                            cargo_toml_file,
                            "[package]\nname = \"{decl_name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\npath = \"src/lib.rs\"",
                            
                        )?;
                    }
                }

                let level_cargo_toml_path = path.join("Cargo.toml");
                let mut level_cargo_toml_file = fs::File::create(&level_cargo_toml_path)?;
                writeln!(level_cargo_toml_file, "[workspace]")?;
                writeln!(level_cargo_toml_file, "members = [")?;
                for member in level_workspace_members {
                    writeln!(level_cargo_toml_file, "    \"{member}\"", member)?;
                }
                writeln!(level_cargo_toml_file, "]")?;
            }
        }
    }

    let root_cargo_toml_path = output_dir.join("Cargo.toml");
    let mut root_cargo_toml_file = fs::File::create(&root_cargo_toml_path)?;
    writeln!(root_cargo_toml_file, "[workspace]")?;
    writeln!(root_cargo_toml_file, "members = [")?;
    for member in &root_workspace_members {
        if let Ok(level) = member.parse::<u32>() {
            if level >= 2 {
                writeln!(root_cargo_toml_file, "    # \"{member}\"", member)?;
            } else {
                writeln!(root_cargo_toml_file, "    \"{member}\"", member)?;
            }
        } else {
            writeln!(root_cargo_toml_file, "    \"{member}\"", member)?;
        }
    }
    writeln!(root_cargo_toml_file, "]")?;

    println!("Workspace generated successfully.");

    Ok(())
}