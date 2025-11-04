use anyhow::Context;
use std::path::{PathBuf, Path};
use std::fs;
use walkdir::WalkDir;
use crate::Args;
use crate::error_collector::ErrorCollection;
use crate::decls_visitor::DeclsVisitor; // Import DeclsVisitor
use crate::gem_parser::GemConfig; // Assuming GemConfig is needed for DeclsVisitor
use syn::parse_file; // Import parse_file
use quote::quote; // Import quote for converting syn items to string
use syn::visit; // Import the visit module

pub async fn handle_split_expanded_bin(args: &Args) -> anyhow::Result<()> {
    println!("Running split-expanded-bin functionality...");

    let files_to_process = args.split_expanded_files.clone(); // This is already a Vec<PathBuf>
    let project_root = args.split_expanded_project_root.clone().unwrap_or_else(|| PathBuf::from("generated_workspace"));
    let rustc_version = args.split_expanded_rustc_version.clone().unwrap_or_else(|| "1.89.0".to_string());
    let rustc_host = args.split_expanded_rustc_host.clone().unwrap_or_else(|| "aarch64-unknown-linux-gnu".to_string());
    let verbose = args.verbose;
    let output_global_toml = args.split_expanded_output_global_toml.clone();

    if project_root.exists() {
        fs::remove_dir_all(&project_root).context(format!("Failed to remove existing project root: {:?}", project_root))?;
    }
    fs::create_dir_all(&project_root).context(format!("Failed to create project root: {:?}", project_root))?;

    let mut error_collection = ErrorCollection::default();
    let gem_config = GemConfig::default(); // Assuming a default GemConfig for now

    if files_to_process.is_empty() {
        println!("No expanded files provided for processing.");
        return Ok(());
    }

    for file_path in files_to_process {
        println!("Processing file: {:?}", file_path);
        let file_content = fs::read_to_string(&file_path)
            .context(format!("Failed to read file: {:?}", file_path))?;

        let file = match parse_file(&file_content) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Warning: Could not parse file {}: {}", file_path.display(), e);
                // Collect error if needed
                continue;
            }
        };

        let mut visitor = DeclsVisitor::new(
            &gem_config,
            Some(file_path.clone()),
            None, // Crate name is not directly available here, might need to be passed as an arg
        );
        syn::visit::Visit::visit_file(&mut visitor, &file); // Corrected call

        // Process collected declarations (this part needs to be adapted from split-expanded-lib's logic)
        // For now, let's just print them
        for decl in visitor.declarations {
            println!("  Declaration: {:?}", decl.item.get_name());
            // Here you would typically write these declarations to individual files
            // or process them further to generate the workspace structure.
            // This is where the logic from split-expanded-lib's main would go.
        }
    }

    if !error_collection.errors.is_empty() {
        let errors_json_path = project_root.join("collected_errors.json");
        error_collection.write_to_file(&errors_json_path).await?;
        eprintln!("Errors collected during processing. See {:?} for details.", errors_json_path);
    }

    println!("Split expanded bin functionality completed.");
    Ok(())
}
