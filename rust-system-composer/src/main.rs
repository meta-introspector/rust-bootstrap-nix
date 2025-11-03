use clap::Parser;
use anyhow::Context;
use std::path::PathBuf;
use std::fs;
use split_expanded_lib::{extract_declarations_from_single_file, RustcInfo, DeclarationItem};
use quote::quote;
use walkdir::WalkDir;

mod project_generator;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory containing expanded Rust files to process.
    #[clap(short, long, value_parser, required = true)]
    input_dir: PathBuf,

    /// Root directory where the generated projects will be created.
    #[clap(short, long, value_parser, required = true)]
    output_root_dir: PathBuf,

    /// Rustc version to record in error samples.
    #[clap(long, value_parser, required = true)]
    rustc_version: String,

    /// Rustc host triple to record in error samples.
    #[clap(long, value_parser, required = true)]
    rustc_host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Create RustcInfo from command-line arguments
    let rustc_info = RustcInfo {
        version: args.rustc_version,
        host: args.rustc_host,
    };

    // Ensure the output root directory exists
    fs::create_dir_all(&args.output_root_dir)
        .context(format!("Failed to create output root directory: {}", args.output_root_dir.display()))?;

    // Iterate through all .rs files in the input directory
    for entry in WalkDir::new(&args.input_dir) {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "rs") {
            println!("Processing file: {}", file_path.display());

            let file_stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown_file");
            let project_name = format!("{}_project", file_stem.replace('.', "_").replace('-', "_"));
            let project_dir = args.output_root_dir.join(&project_name);
            let project_src_dir = project_dir.join("src");

            // Create project directory structure
            fs::create_dir_all(&project_src_dir)
                .context(format!("Failed to create project src directory for {}: {}", project_name, project_src_dir.display()))?;

            // Generate Cargo.toml
            let cargo_toml_content = project_generator::generate_cargo_toml_content(&project_name);
            fs::write(project_dir.join("Cargo.toml"), cargo_toml_content)
                .context(format!("Failed to write Cargo.toml for project {}", project_name))?;

            // Generate flake.nix
            let nixpkgs_url = "github:NixOS/nixpkgs/nixos-23.11"; // Placeholder
            let system_arch = "x86_64-linux"; // Placeholder
            let use_rustc_wrapper = false; // Placeholder

            let flake_nix_content = flake_template_generator::generate_flake_nix_content(
                nixpkgs_url,
                system_arch,
                use_rustc_wrapper,
            );
            fs::write(project_dir.join("flake.nix"), flake_nix_content)
                .context(format!("Failed to write flake.nix for project {}", project_name))?;

            // Extract declarations using split-expanded-lib
            let (declarations, errors, file_metadata) = extract_declarations_from_single_file(
                file_path,
                &rustc_info,
                &project_name, // Use project_name as crate_name for now
            ).await?;

            if !errors.is_empty() {
                eprintln!("Errors encountered during parsing for {}:", file_path.display());
                for error in errors {
                    eprintln!("  File: {}", error.file_path.display());
                    eprintln!("  Error Type: {}", error.error_type);
                    eprintln!("  Message: {}", error.error_message);
                    if let Some(snippet) = error.code_snippet {
                        eprintln!("  Code Snippet:\n{}", snippet);
                    }
                }
            }

            let mut generated_module_names: Vec<String> = Vec::new();
            let mut has_proc_macros = false;

            // Write all declarations to individual files in the project's src_dir
            for (identifier, declaration) in declarations.into_iter().map(|decl| (decl.get_identifier(), decl)) {
                let output_file_path = project_src_dir.join(format!("{}.rs", identifier));

                let mut file_content = String::new();

                // Add necessary global use statements for the generated file
                file_content.push_str("use std::collections::HashSet;\n");
                file_content.push_str("use split_expanded_lib::{DeclarationItem};\n"); // Assuming DeclarationItem is always needed

                // Add use statements from resolved_dependencies
                for dep in &declaration.resolved_dependencies {
                    file_content.push_str(&format!("use {};\n", dep));
                }
                file_content.push_str("\n");

                // Add the declaration item
                let item_token_stream = match &declaration.item {
                    DeclarationItem::Const(item) => quote! { #item },
                    DeclarationItem::Struct(item) => quote! { #item },
                    DeclarationItem::Enum(item) => quote! { #item },
                    DeclarationItem::Fn(item) => quote! { #item },
                    DeclarationItem::Static(item) => quote! { #item },
                    DeclarationItem::Macro(item) => quote! { #item },
                    DeclarationItem::Mod(item) => quote! { #item },
                    DeclarationItem::Trait(item) => quote! { #item },
                    DeclarationItem::TraitAlias(item) => quote! { #item },
                    DeclarationItem::Type(item) => quote! { #item },
                    DeclarationItem::Union(item) => quote! { #item },
                    DeclarationItem::Other(item) => quote! { #item },
                };
                file_content.push_str(&item_token_stream.to_string());

                fs::write(&output_file_path, file_content)
                    .context(format!("Failed to write declaration to file: {}", output_file_path.display()))?;

                generated_module_names.push(identifier);
                if declaration.is_proc_macro {
                    has_proc_macros = true;
                }
            }

            // Generate src/lib.rs
            let lib_rs_content = project_generator::generate_lib_rs_content(
                &generated_module_names,
                has_proc_macros,
                &file_metadata.feature_attributes,
                &file_metadata.extern_crates,
                file_path.to_str().unwrap_or(""),
            );
            fs::write(project_src_dir.join("lib.rs"), lib_rs_content)
                .context(format!("Failed to write lib.rs for project {}", project_name))?;

            // Generate src/prelude.rs
            let common_imports = project_generator::get_common_imports();
            let prelude_rs_content = project_generator::generate_prelude_rs_content(
                &file_metadata.global_uses,
                &file_metadata.feature_attributes,
                &common_imports,
            );
            fs::write(project_src_dir.join("prelude.rs"), prelude_rs_content)
                .context(format!("Failed to write prelude.rs for project {}", project_name))?;

            // Update Cargo.toml for proc macros if needed
            if has_proc_macros {
                let cargo_toml_path = project_dir.join("Cargo.toml");
                let mut cargo_toml = fs::read_to_string(&cargo_toml_path)
                    .context(format!("Failed to read Cargo.toml for project {}", project_name))?;
                cargo_toml = cargo_toml.replace("[lib]", "[lib]\nproc-macro = true");
                fs::write(&cargo_toml_path, cargo_toml)
                    .context(format!("Failed to update Cargo.toml for project {}", project_name))?;
            }

            println!("Successfully generated project: {}", project_name);
        }
    }

    Ok(())
}