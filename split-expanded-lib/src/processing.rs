use anyhow::Context;
use std::path::Path;
use std::io::Write; // For stdout().flush()
use std::fmt::Write as FmtWrite; // For writeln! into String
use tokio;

use quote::{quote, ToTokens}; // For quote! macro

use std::collections::{HashSet, HashMap}; // For HashMap and HashSet
use serde_json; // For serde_json::from_str
use toml; // For toml::to_string_pretty
use syn::{self, visit::Visit};

use crate::types::{ExpandedManifest, FileMetadata, Declaration, DeclarationItem, ErrorSample, RustcInfo, PublicSymbol};
use crate::visitors::DeclsVisitor; // Assuming visitors are re-exported from lib.rs

pub async fn process_expanded_manifest(
    expanded_manifest_path: &Path,
    project_root: &Path,
    _rustc_version: String,
    _rustc_host: String,
    verbosity: u8,
    layer: Option<u32>,
) -> anyhow::Result<()> {
    if verbosity >= 1 {
        println!("split-expanded-lib::process_expanded_manifest started.");
        println!("Verbosity level: {}", verbosity);
        std::io::stdout().flush().unwrap();
    }

    // Read expanded manifest
    let manifest_content = tokio::fs::read_to_string(expanded_manifest_path)
        .await
        .context(format!("Failed to read expanded manifest file: {}", expanded_manifest_path.display()))?;
    let expanded_manifest: ExpandedManifest = serde_json::from_str(&manifest_content)
        .context(format!("Failed to parse expanded manifest JSON from: {}", expanded_manifest_path.display()))?;

    // Create RustcInfo from manifest
    let rustc_info = RustcInfo {
        version: expanded_manifest.rustc_version,
        host: expanded_manifest.rustc_host,
    };

    // Create project root and src directory if they don't exist
    let src_dir = project_root.join("src");
    tokio::fs::create_dir_all(&src_dir)
        .await
        .context(format!("Failed to create project src directory: {}", src_dir.display()))?;
    if verbosity >= 2 {
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
        if let Some(requested_layer) = layer {
            if expanded_file_entry.layer != requested_layer {
                if verbosity >= 1 {
                    println!("Skipping expanded file for package {} (layer {}), not in requested layer {}.",
                        expanded_file_entry.package_name,
                        expanded_file_entry.layer,
                        requested_layer
                    );
                }
                continue; // Skip this file if it's not in the requested layer
            }
        }

        if verbosity >= 1 {
            println!("Processing expanded RS file: {}", expanded_file_entry.expanded_rs_path.display());
            std::io::stdout().flush().unwrap();
        }

        let crate_name = expanded_file_entry.package_name.as_str();
        let expanded_rs_file_path = expanded_file_entry.expanded_rs_path;

        if verbosity >= 2 {
            println!("  Derived crate_name: {}", crate_name);
            println!("  Associated expanded RS file: {}", expanded_rs_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        let (declarations, errors, _file_metadata, _public_symbols) = extract_declarations_from_single_file(
            &expanded_rs_file_path,
            &rustc_info,
            crate_name,
            verbosity,
        ).await?;

        if verbosity >= 2 {
            println!("  Extracted {} declarations and {} errors from {}", declarations.len(), errors.len(), expanded_rs_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        for (identifier, decl) in declarations {
            if verbosity >= 3 {
                println!("    Inserting declaration: {}", decl.get_identifier());
                std::io::stdout().flush().unwrap();
            }
            global_declarations.insert(identifier, decl);
        }
        if !errors.is_empty() {
            if verbosity >= 2 {
                println!("    Extending all_errors with {} new errors.", errors.len());
                std::io::stdout().flush().unwrap();
            }
            all_errors.extend(errors);
        }
    }

    if !all_errors.is_empty() {
        if verbosity >= 2 {
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
    if verbosity >= 2 {
        println!("Starting first pass of dependency resolution.");
        std::io::stdout().flush().unwrap();
    }
    for (decl_id, decl) in &global_declarations {
        let mut current_resolved_dependencies = HashSet::new();
        if verbosity >= 3 {
            println!("  Processing dependencies for declaration: {}", decl_id);
            std::io::stdout().flush().unwrap();
        }

        // Resolve referenced types (internal and external)
        for referenced_type in &decl.referenced_types {
            if let Some(resolved_decl) = global_declarations.get(referenced_type) {
                if verbosity >= 3 {
                    println!("    Resolved internal type dependency: {} -> {}", referenced_type, resolved_decl.get_identifier());
                    std::io::stdout().flush().unwrap();
                }
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                if verbosity >= 3 {
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
                if verbosity >= 3 {
                    println!("    Resolved internal function dependency: {} -> {}", referenced_fn, resolved_decl.get_identifier());
                    std::io::stdout().flush().unwrap();
                }
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                if verbosity >= 3 {
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
                if verbosity >= 3 {
                    println!("    Identified external import dependency: {}", import);
                    std::io::stdout().flush().unwrap();
                }
                current_resolved_dependencies.insert(import.clone());
            }
        }

        dependencies_to_resolve.insert(decl_id.clone(), current_resolved_dependencies);
    }

    // Second pass: Populate direct_dependencies and resolved_dependencies
    if verbosity >= 2 {
        println!("Starting second pass of dependency resolution.");
        std::io::stdout().flush().unwrap();
    }
    for (decl_id, decl) in global_declarations.iter_mut() {
        if let Some(resolved_deps) = dependencies_to_resolve.remove(decl_id) {
            if verbosity >= 3 {
                println!("  Populating dependencies for declaration: {}", decl_id);
                std::io::stdout().flush().unwrap();
            }
            decl.direct_dependencies = resolved_deps.iter().map(|s| s.split("::").last().unwrap_or(s).to_string()).collect();
            decl.resolved_dependencies = resolved_deps;
        }
    }

    // Phase 3: Implement Layering Algorithm
    if verbosity >= 2 {
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
        if verbosity >= 2 {
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

            // A declaration's level is 1 + the maximum level of its direct dependencies
            let new_level = if decl.direct_dependencies.is_empty() {
                0
            } else {
                max_dep_level + 1
            };

            if new_level > current_level {
                if verbosity >= 3 {
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
        if verbosity >= 1 {
            println!("Layering algorithm stopped after {} iterations due to reaching the limit.", max_iterations_limit);
            std::io::stdout().flush().unwrap();
        }
    }

    println!("Max dependency level found: {}", max_level);

    let mut generated_module_names: Vec<String> = Vec::new();
    let mut has_proc_macros = false;

    // Write all declarations to individual files in the src_dir
    if verbosity >= 2 {
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
            DeclarationItem::Other(item_str) => {
                // Attempt to parse the string into a syn::Item to check for proc macros
                if let Ok(parsed_item) = syn::parse_str::<syn::Item>(item_str) {
                    if let syn::Item::Macro(mac) = parsed_item {
                        if mac.mac.path.segments.last().map_or(false, |s| s.ident == "proc_macro") {
                            "proc_macro"
                        } else {
                            "other"
                        }
                    } else {
                        "other"
                    }
                } else {
                    "other" // If parsing fails, treat it as a generic "other"
                }
            },
        };
        if verbosity >= 2 {
            println!("  Processing declaration '{}' of type '{}' at level {}", identifier, declaration_type, level);
            std::io::stdout().flush().unwrap();
        }

        let declaration_dir = project_root.join("rust-bootstrap-core")
            .join("src")
            .join(format!("level_{:02}", level))
            .join("src")
            .join(format!("{}_t", declaration_type))
            .join(&declaration.crate_name);
        tokio::fs::create_dir_all(&declaration_dir)
            .await
            .context(format!("Failed to create declaration directory: {}", declaration_dir.display()))?;
        if verbosity >= 2 {
            if declaration_dir.exists() {
                println!("  Declaration directory already exists or was created: {}", declaration_dir.display());
            }
        } else {
            println!("  Failed to create declaration directory (but context handled it): {}", declaration_dir.display());
        }
        std::io::stdout().flush().unwrap();

        let output_file_path = declaration_dir.join(format!("{}.rs", identifier));

        let mut file_content = String::new();

        // Serialize declaration to TOML and write as header
        let declaration_toml = toml::to_string_pretty(&declaration)
            .unwrap_or_else(|e| format!("Error serializing declaration to TOML: {}", e));

        writeln!(file_content, "// --- DECLARATION_METADATA ---")?;
        for line in declaration_toml.lines() {
            writeln!(file_content, "// {}", line)?;
        }
        writeln!(file_content, "// --- END_DECLARATION_METADATA ---")?;
        writeln!(file_content, "")?; // Add an empty line for separation

        // Add use statements
        for dep in &declaration.resolved_dependencies {
            if verbosity >= 3 {
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
        if verbosity >= 3 {
            println!("    Converting item to token stream for '{}'.", identifier);
            std::io::stdout().flush().unwrap();
        }
        file_content.push_str(&item_token_stream.to_string());

        tokio::fs::write(&output_file_path, file_content)
            .await
            .context(format!("Failed to write declaration to file: {}", output_file_path.display()))?;
        if verbosity >= 2 {
            println!("Wrote declaration to file: {}", output_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        generated_module_names.push(identifier);
        if declaration.is_proc_macro {
            if verbosity >= 3 {
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

async fn extract_declarations_from_single_file(
    file_path: &Path,
    _rustc_info: &RustcInfo,
    crate_name: &str,
    verbosity: u8,
) -> anyhow::Result<(HashMap<String, Declaration>, Vec<ErrorSample>, FileMetadata, Vec<PublicSymbol>)> {
    if verbosity >= 2 {
        println!("  [split-expanded-lib] extract_declarations_from_single_file: Processing file: {}", file_path.display());
        std::io::stdout().flush().unwrap();
    }

    let file_content = tokio::fs::read_to_string(file_path)
        .await
        .context(format!("Failed to read file: {}", file_path.display()))?;

    let mut file_metadata = FileMetadata::default();
    let public_symbols: Vec<PublicSymbol> = Vec::new();
    let errors: Vec<ErrorSample> = Vec::new();

    // Extract global `use` statements and `extern crate` declarations
    let syntax_tree = syn::parse_file(&file_content)
        .context(format!("Failed to parse file as Rust syntax tree: {}", file_path.display()))?;

    for item in &syntax_tree.items {
        match item {
            syn::Item::Use(item_use) => {
                file_metadata.global_uses.insert(item_use.to_token_stream().to_string());
            },
            syn::Item::ExternCrate(item_extern_crate) => {
                file_metadata.extern_crates.insert(item_extern_crate.ident.to_string());
            },
            _ => {}
        }
    }

    let mut visitor = DeclsVisitor::new(file_path.to_path_buf(), crate_name.to_string(), verbosity, file_metadata.extern_crates.clone());
    visitor.visit_file(&syntax_tree);

    if verbosity >= 2 {
        println!("  [split-expanded-lib] extract_declarations_from_single_file: Found {} declarations in {}", visitor.declarations.len(), file_path.display());
        std::io::stdout().flush().unwrap();
    }

    Ok((visitor.declarations, errors, file_metadata, public_symbols))
}
