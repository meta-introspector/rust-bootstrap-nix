use clap::Parser;
use anyhow::Context;
use std::path::{PathBuf};
use split_expanded_lib::{extract_declarations_from_single_file, RustcInfo, DeclarationItem, Declaration, ErrorSample};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json;

use quote::quote;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedManifest {
    pub rustc_version: String,
    pub rustc_host: String,
    pub project_root: PathBuf,
    pub expanded_files: Vec<ExpandedFileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpandedFileEntry {
    pub package_name: String,
    pub target_type: String,
    pub target_name: String,
    pub expanded_rs_path: PathBuf,
    pub cargo_expand_command: String,
    pub timestamp: u64,
    pub flake_lock_details: serde_json::Value,
    pub layer: u32,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set verbosity level (0 = silent, 1 = normal, 2 = detailed, 3 = debug).
    #[arg(short, long, default_value_t = 1)]
    pub verbosity: u8,

    /// Path to the expanded_manifest.json file.
    #[arg(long)]
    pub expanded_manifest: PathBuf,

    /// Directory to output the generated declaration files.
    #[clap(short, long, value_parser, required = true)]
    project_root: PathBuf,

    /// Rustc version (e.g., "1.89.0").
    #[arg(long)]
    pub rustc_version: String,

    /// Rustc host triple (e.g., "aarch64-unknown-linux-gnu").
    #[arg(long)]
    pub rustc_host: String,

    /// Optional: Process only crates at a specific dependency layer.
    #[arg(long)]
    layer: Option<u32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.verbosity >= 1 {
        println!("split-expanded-bin started.");
        println!("Verbosity level: {}", args.verbosity);
        std::io::stdout().flush().unwrap();
    }

    // Read expanded manifest
    let manifest_content = fs::read_to_string(&args.expanded_manifest)
        .context(format!("Failed to read expanded manifest file: {}", args.expanded_manifest.display()))?;
    let expanded_manifest: ExpandedManifest = serde_json::from_str(&manifest_content)
        .context(format!("Failed to parse expanded manifest JSON from: {}", args.expanded_manifest.display()))?;

    // Create RustcInfo from manifest
    let rustc_info = RustcInfo {
        version: expanded_manifest.rustc_version,
        host: expanded_manifest.rustc_host,
    };

    // Create project root and src directory if they don't exist
    let src_dir = expanded_manifest.project_root.join("src");
    fs::create_dir_all(&src_dir)
        .context(format!("Failed to create project src directory: {}", src_dir.display()))?;
    if args.verbosity >= 2 {
        if src_dir.exists() {
            println!("Created project src directory already exists or was created: {}", src_dir.display());
        } else {
            println!("Failed to create project src directory (but context handled it): {}", src_dir.display());
        }
        std::io::stdout().flush().unwrap();
    }
    let mut global_declarations: HashMap<String, Declaration> = HashMap::new();
    let mut all_errors: Vec<ErrorSample> = Vec::new();

    for expanded_file_entry in expanded_manifest.expanded_files {
        if let Some(requested_layer) = args.layer {
            if expanded_file_entry.layer != requested_layer {
                if args.verbosity >= 1 {
                    println!("Skipping expanded file for package {} (layer {}), not in requested layer {}.",
                        expanded_file_entry.package_name,
                        expanded_file_entry.layer,
                        requested_layer
                    );
                }
                continue; // Skip this file if it's not in the requested layer
            }
        }

        if args.verbosity >= 1 {
            println!("Processing expanded RS file: {}", expanded_file_entry.expanded_rs_path.display());
            std::io::stdout().flush().unwrap();
        }

        let crate_name = expanded_file_entry.package_name.as_str();
        let expanded_rs_file_path = expanded_file_entry.expanded_rs_path;

        if args.verbosity >= 2 {
            println!("  Derived crate_name: {}", crate_name);
            println!("  Associated expanded RS file: {}", expanded_rs_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        let (declarations, errors, _file_metadata, _public_symbols) = extract_declarations_from_single_file(
            &expanded_rs_file_path,
            &rustc_info,
            crate_name,
            args.verbosity,
        ).await?;

        if args.verbosity >= 2 {
            println!("  Extracted {} declarations and {} errors from {}", declarations.len(), errors.len(), expanded_rs_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        for decl in declarations {
            if args.verbosity >= 3 {
                println!("    Inserting declaration: {}", decl.get_identifier());
                std::io::stdout().flush().unwrap();
            }
            global_declarations.insert(decl.get_identifier(), decl);
        }
        if !errors.is_empty() {
            if args.verbosity >= 2 {
                println!("    Extending all_errors with {} new errors.", errors.len());
                std::io::stdout().flush().unwrap();
            }
            all_errors.extend(errors);
        }
    }

    if !all_errors.is_empty() {
        if args.verbosity >= 2 {
            println!("Entering error reporting block.");
            std::io::stdout().flush().unwrap();
        }
        eprintln!("Errors encountered during parsing:");
        for error in all_errors {
            eprintln!("  File: {}", error.file_path.display());
            eprintln!("  Error Type: {}", error.error_type);
            eprintln!("  Message: {}", error.error_message);
            if let Some(snippet) = error.code_snippet {
                eprintln!("  Code Snippet:\n{}", snippet);
            }
        }
    }

    // TODO: Dependency resolution phase here
    let mut dependencies_to_resolve: HashMap<String, HashSet<String>> = HashMap::new();

    // First pass: Collect all dependencies to resolve without mutating global_declarations
    if args.verbosity >= 2 {
        println!("Starting first pass of dependency resolution.");
        std::io::stdout().flush().unwrap();
    }
    for (decl_id, decl) in &global_declarations {
        let mut current_resolved_dependencies = HashSet::new();
        if args.verbosity >= 3 {
            println!("  Processing dependencies for declaration: {}", decl_id);
            std::io::stdout().flush().unwrap();
        }

        // Resolve referenced types (internal and external)
        for referenced_type in &decl.referenced_types {
            if let Some(resolved_decl) = global_declarations.get(referenced_type) {
                if args.verbosity >= 3 {
                    println!("    Resolved internal type dependency: {} -> {}", referenced_type, resolved_decl.get_identifier());
                    std::io::stdout().flush().unwrap();
                }
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                if args.verbosity >= 3 {
                    println!("    Identified external type dependency: {}", referenced_type);
                    std::io::stdout().flush().unwrap();
                }
                // This is an external type dependency
                current_resolved_dependencies.insert(referenced_type.clone());
            }
        }

        // Resolve referenced functions (internal and external)
        for referenced_fn in &decl.referenced_functions {
            if let Some(resolved_decl) = global_declarations.get(referenced_fn) {
                if args.verbosity >= 3 {
                    println!("    Resolved internal function dependency: {} -> {}", referenced_fn, resolved_decl.get_identifier());
                    std::io::stdout().flush().unwrap();
                }
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                if args.verbosity >= 3 {
                    println!("    Identified external function dependency: {}", referenced_fn);
                    std::io::stdout().flush().unwrap();
                }
                // This is an external function dependency
                current_resolved_dependencies.insert(referenced_fn.clone());
            }
        }

        // Also consider required_imports as dependencies if they are not internal declarations
        for import in &decl.required_imports {
            if !global_declarations.contains_key(import) {
                if args.verbosity >= 3 {
                    println!("    Identified external import dependency: {}", import);
                    std::io::stdout().flush().unwrap();
                }
                current_resolved_dependencies.insert(import.clone());
            }
        }

        dependencies_to_resolve.insert(decl_id.clone(), current_resolved_dependencies);
    }

    // Second pass: Populate direct_dependencies and resolved_dependencies
    if args.verbosity >= 2 {
        println!("Starting second pass of dependency resolution.");
        std::io::stdout().flush().unwrap();
    }
    for (decl_id, decl) in global_declarations.iter_mut() {
        if let Some(resolved_deps) = dependencies_to_resolve.remove(decl_id) {
            if args.verbosity >= 3 {
                println!("  Populating dependencies for declaration: {}", decl_id);
                std::io::stdout().flush().unwrap();
            }
            decl.direct_dependencies = resolved_deps.iter().map(|s| s.split("::").last().unwrap_or(s).to_string()).collect();
            decl.resolved_dependencies = resolved_deps;
        }
    }

    // Phase 3: Implement Layering Algorithm
    if args.verbosity >= 2 {
        println!("Starting layering algorithm.");
        std::io::stdout().flush().unwrap();
    }
    let mut declaration_levels: HashMap<String, usize> = HashMap::new();
    let mut changed = true;
    let mut max_level = 0;

    // Initialize all declarations to level 0
    for (decl_id, _) in &global_declarations {
        declaration_levels.insert(decl_id.clone(), 0);
    }

    let mut iteration_count = 0;
    let max_iterations_limit = 8; // User-defined limit for layering algorithm iterations
    while changed && iteration_count < max_iterations_limit {
        iteration_count += 1;
        if args.verbosity >= 2 {
            println!("  Layering algorithm iteration: {}", iteration_count);
            std::io::stdout().flush().unwrap();
        }
        changed = false;
        for (decl_id, decl) in &global_declarations {
            let current_level = *declaration_levels.get(decl_id).unwrap_or(&0);
            let mut max_dep_level = 0;

            for dep_id in &decl.direct_dependencies {
                if let Some(dep_level) = declaration_levels.get(dep_id) {
                    max_dep_level = max_dep_level.max(*dep_level);
                }
            }

            // A declaration\'s level is 1 + the maximum level of its direct dependencies
            let new_level = if decl.direct_dependencies.is_empty() {
                0
            } else {
                max_dep_level + 1
            };

            if new_level > current_level {
                if args.verbosity >= 3 {
                    println!("    Level changed for {}: {} -> {}", decl_id, current_level, new_level);
                    std::io::stdout().flush().unwrap();
                }
                declaration_levels.insert(decl_id.clone(), new_level);
                changed = true;
                max_level = max_level.max(new_level);
            }
        }
    }

    if iteration_count >= max_iterations_limit {
        if args.verbosity >= 1 {
            println!("Layering algorithm stopped after {} iterations due to reaching the limit.", max_iterations_limit);
            std::io::stdout().flush().unwrap();
        }
    }

    println!("Max dependency level found: {}", max_level);

    let mut generated_module_names: Vec<String> = Vec::new();
    let mut has_proc_macros = false;

    // Write all declarations to individual files in the src_dir
    if args.verbosity >= 2 {
        println!("Starting declaration writing phase.");
        std::io::stdout().flush().unwrap();
    }
    for (identifier, declaration) in global_declarations.into_iter() {
        let level = *declaration_levels.get(&identifier).unwrap_or(&0);
        let declaration_type = match &declaration.item {
            DeclarationItem::Const(_) => "const",
            DeclarationItem::Struct(_) => "struct",
            DeclarationItem::Enum(_) => "enum",
            DeclarationItem::Fn(_) => "fn",
            DeclarationItem::Static(_) => "static",
            DeclarationItem::Macro(_) => "macro",
            DeclarationItem::Mod(_) => "mod",
            DeclarationItem::Trait(_) => "trait",
            DeclarationItem::TraitAlias(_) => "trait_alias",
            DeclarationItem::Type(_) => "type",
            DeclarationItem::Union(_) => "union",
            DeclarationItem::Other(_) => "other",
        };
        if args.verbosity >= 2 {
            println!("  Processing declaration '{}' of type '{}' at level {}", identifier, declaration_type, level);
            std::io::stdout().flush().unwrap();
        }

        let declaration_dir = src_dir.join(format!("level_{:02}/src/{}_t/{}", level, declaration_type, declaration.crate_name));
        fs::create_dir_all(&declaration_dir)
            .context(format!("Failed to create declaration directory: {}", declaration_dir.display()))?;
        if args.verbosity >= 2 {
            if declaration_dir.exists() {
                println!("  Declaration directory already exists or was created: {}", declaration_dir.display());
            } else {
                println!("  Failed to create declaration directory (but context handled it): {}", declaration_dir.display());
            }
            std::io::stdout().flush().unwrap();
        }

        let output_file_path = declaration_dir.join(format!("{}.rs", identifier));

        let mut file_content = String::new();

        // Add use statements
        for dep in &declaration.resolved_dependencies {
            if args.verbosity >= 3 {
                println!("    Adding use statement: {}", dep);
                std::io::stdout().flush().unwrap();
            }
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
        if args.verbosity >= 3 {
            println!("    Converting item to token stream for '{}'.", identifier);
            std::io::stdout().flush().unwrap();
        }
        file_content.push_str(&item_token_stream.to_string());

        fs::write(&output_file_path, file_content)
            .context(format!("Failed to write declaration to file: {}", output_file_path.display()))?;
        if args.verbosity >= 2 {
            println!("Wrote declaration to file: {}", output_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        generated_module_names.push(identifier);
        if declaration.is_proc_macro {
            if args.verbosity >= 3 {
                println!("  Declaration '{}' is a procedural macro.", declaration.get_identifier());
                std::io::stdout().flush().unwrap();
            }
            has_proc_macros = true;
        }
    }

    // Print generated module names and proc macro flag for the orchestrating script
    println!("GENERATED_MODULE_NAMES: {}", generated_module_names.join(","));
    println!("HAS_PROC_MACROS: {}", has_proc_macros);

    Ok(())
}
