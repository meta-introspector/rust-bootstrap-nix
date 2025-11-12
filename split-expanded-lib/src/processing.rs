use anyhow::Context;
use std::path::Path;
use std::fmt::Write as FmtWrite; // For writeln! into String
use tokio;

use quote::{quote, ToTokens}; // For quote! macro

use std::collections::{HashSet, HashMap}; // For HashMap and HashSet
use serde_json; // For serde_json::from_str
use toml; // For toml::to_string_pretty
use syn::{self, visit::Visit};
use regex::Regex;

use crate::types::{ExpandedManifest, ExpandedFileEntry, FileMetadata, Declaration, DeclarationItem, ErrorSample, RustcInfo, PublicSymbol, ExtractionResult};
use crate::visitors::DeclsVisitor; // Assuming visitors are re-exported from lib.rs

pub struct ProcessExpandedManifestInputs<'a> {
    pub expanded_manifest_path: &'a Path,
    pub project_root: &'a Path,
    pub rustc_info: &'a RustcInfo,
    pub verbosity: u8,
    pub layer: Option<u32>,
    pub canonical_output_root: &'a Path,
    pub package_filter: Option<String>,
}

pub async fn process_expanded_manifest(
    inputs: ProcessExpandedManifestInputs<'_>,
) -> anyhow::Result<Vec<String>> {
    let mut warnings: Vec<String> = Vec::new();

    if inputs.verbosity >= 1 {
        warnings.push(format!("split-expanded-lib::process_expanded_manifest started."));
        warnings.push(format!("Verbosity level: {}", inputs.verbosity));
    }

    // Read expanded manifest
    let manifest_content = tokio::fs::read_to_string(inputs.expanded_manifest_path)
        .await
        .context(format!("Failed to read expanded manifest file: {}", inputs.expanded_manifest_path.display()))?;
    let expanded_files: Vec<ExpandedFileEntry> = serde_json::from_str(&manifest_content)
        .context(format!("Failed to parse expanded manifest JSON from: {}", inputs.expanded_manifest_path.display()))?;

    let expanded_manifest = ExpandedManifest {
        rustc_version: inputs.rustc_info.version.clone(),
        rustc_host: inputs.rustc_info.host.clone(),
        project_root: inputs.project_root.to_path_buf(),
        expanded_files,
    };

    // Create RustcInfo from manifest
    let rustc_info = RustcInfo {
        version: expanded_manifest.rustc_version,
        host: expanded_manifest.rustc_host,
    };

    // Create project root and src directory if they don't exist
    let src_dir = inputs.project_root.join("src");
    tokio::fs::create_dir_all(&src_dir)
        .await
        .context(format!("Failed to create project src directory: {}", src_dir.display()))?;
    if inputs.verbosity >= 2 {
        if src_dir.exists() {
            warnings.push(format!("Created project src directory already exists or was created: {}", src_dir.display()));
        } else {
            warnings.push(format!("Failed to create project src directory (but context handled it): {}", src_dir.display()));
        }
    }
    let mut global_declarations: HashMap<String, Declaration> = HashMap::new();
    let all_errors: Vec<ErrorSample> = Vec::new();

    for expanded_file_entry in expanded_manifest.expanded_files {
        if let Some(requested_layer) = inputs.layer {
            if expanded_file_entry.layer != requested_layer {
                if inputs.verbosity >= 1 {
                    warnings.push(format!("Skipping expanded file for package {} (layer {}), not in requested layer {}.",
                        expanded_file_entry.package_name,
                        expanded_file_entry.layer,
                        requested_layer
                    ));
                }
                continue; // Skip this file if it's not in the requested layer
            }
        }

        // Add package filter here
        if let Some(ref filter_name) = inputs.package_filter {
            if !expanded_file_entry.package_name.contains(filter_name) {
                if inputs.verbosity >= 1 {
                    warnings.push(format!("Skipping expanded file for package {} (does not match filter '{}').",
                        expanded_file_entry.package_name,
                        filter_name
                    ));
                }
                continue; // Skip this file if it doesn't match the filter
            }
        }

        if inputs.verbosity >= 1 {
            warnings.push(format!("Processing expanded RS file: {}", expanded_file_entry.expanded_rs_path.display()));
        }

        let crate_name = expanded_file_entry.package_name.as_str();
        let expanded_rs_file_path = expanded_file_entry.expanded_rs_path;

        if inputs.verbosity >= 2 {
            warnings.push(format!("  Derived crate_name: {}", crate_name));
            warnings.push(format!("  Associated expanded RS file: {}", expanded_rs_file_path.display()));
        }

        let extraction_result = extract_declarations_from_single_file(
            &expanded_rs_file_path,
            &rustc_info,
            crate_name,
            inputs.verbosity,
            &mut warnings,
            inputs.canonical_output_root,
        ).await?;

        let declarations = extraction_result.declarations;
        let errors = extraction_result.errors;

        if inputs.verbosity >= 2 {
            warnings.push(format!("  Extracted {} declarations and {} errors from {}", declarations.len(), errors.len(), expanded_rs_file_path.display()));
        }

        for (identifier, decl) in declarations {
            if inputs.verbosity >= 3 {
                warnings.push(format!("    Inserting declaration: {}", identifier));
            }
            global_declarations.insert(identifier, decl);
        }
        if !errors.is_empty() {
            if inputs.verbosity >= 2 {
                warnings.push(format!("    Extending all_errors with {} new errors.", errors.len()));
            }
            for error in errors {
                warnings.push(format!("  Error in {}: {} - {}", error.file_path.display(), error.error_type, error.error_message));
                if let Some(snippet) = error.code_snippet {
                    warnings.push(format!("    Code Snippet:\n{}", snippet));
                }
            }
        }
    }

    if !all_errors.is_empty() {
        if inputs.verbosity >= 2 {
            warnings.push(format!("Entering error reporting block."));
        }
        warnings.push(format!("Errors encountered during parsing:"));
        for error in all_errors {
            warnings.push(format!("  File: {}", error.file_path.display()));
            warnings.push(format!("  Error Type: {}", error.error_type));
            warnings.push(format!("  Message: {}", error.error_message));
            if let Some(snippet) = error.code_snippet {
                warnings.push(format!("  Code Snippet:\n{}", snippet));
            }
        }
    }

    // TODO: Dependency resolution phase here
    let mut dependencies_to_resolve: HashMap<String, HashSet<String>> = HashMap::new();

    // First pass: Collect all dependencies to resolve without mutating global_declarations
    if inputs.verbosity >= 2 {
        warnings.push(format!("Starting first pass of dependency resolution."));
    }
    for (decl_id, decl) in &global_declarations {
        let mut current_resolved_dependencies = HashSet::new();
        if inputs.verbosity >= 3 {
            warnings.push(format!("  Processing dependencies for declaration: {}", decl_id));
        }

        // Resolve referenced types (internal and external)
        for referenced_type in &decl.referenced_types {
            if let Some(resolved_decl) = global_declarations.get(referenced_type) {
                if inputs.verbosity >= 3 {
                    warnings.push(format!("    Resolved internal type dependency: {} -> {}", referenced_type, resolved_decl.get_identifier()));
                }
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                if inputs.verbosity >= 3 {
                    warnings.push(format!("    Identified external type dependency: {}", referenced_type));
                }
                // This is an external type dependency
                current_resolved_dependencies.insert(referenced_type.clone());
            }
        }

        // Resolve referenced functions (internal and external)
        for referenced_fn in &decl.referenced_functions {
            if let Some(resolved_decl) = global_declarations.get(referenced_fn) {
                if inputs.verbosity >= 3 {
                    warnings.push(format!("    Resolved internal function dependency: {} -> {}", referenced_fn, resolved_decl.get_identifier()));
                }
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                if inputs.verbosity >= 3 {
                    warnings.push(format!("    Identified external function dependency: {}", referenced_fn));
                }
                // This is an external function dependency
                current_resolved_dependencies.insert(referenced_fn.clone());
            }
        }

        // Also consider required_imports as dependencies if they are not internal declarations
        for import in &decl.required_imports {
            if !global_declarations.contains_key(import) {
                if inputs.verbosity >= 3 {
                    warnings.push(format!("    Identified external import dependency: {}", import));
                }
                current_resolved_dependencies.insert(import.clone());
            }
        }

        dependencies_to_resolve.insert(decl_id.clone(), current_resolved_dependencies);
    }

    // Second pass: Populate direct_dependencies and resolved_dependencies
    if inputs.verbosity >= 2 {
        warnings.push(format!("Starting second pass of dependency resolution."));
    }
    for (decl_id, decl) in global_declarations.iter_mut() {
        if let Some(resolved_deps) = dependencies_to_resolve.remove(decl_id) {
            if inputs.verbosity >= 3 {
                warnings.push(format!("  Populating dependencies for declaration: {}", decl_id));
            }
            decl.direct_dependencies = resolved_deps.iter().map(|s| s.split("::").last().unwrap_or(s).to_string()).collect();
            decl.resolved_dependencies = resolved_deps;
        }
    }

    // Phase 3: Implement Layering Algorithm
    if inputs.verbosity >= 2 {
        warnings.push(format!("Starting layering algorithm."));
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
        if inputs.verbosity >= 2 {
            warnings.push(format!("  Layering algorithm iteration: {}", iteration_count));
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
                if inputs.verbosity >= 3 {
                    warnings.push(format!("    Level changed for {}: {} -> {}", decl_id, current_level, new_level));
                }
                declaration_levels.insert(decl_id.clone(), new_level);
                changed = true;
                max_level = max_level.max(new_level);
            }
        }
    }

    if iteration_count >= max_iterations_limit {
        if inputs.verbosity >= 1 {
            warnings.push(format!("Layering algorithm stopped after {} iterations due to reaching the limit.", max_iterations_limit));
        }
    }

    warnings.push(format!("Max dependency level found: {}", max_level));

    let mut generated_module_names: Vec<String> = Vec::new();
    let mut has_proc_macros = false;

    // Write all declarations to individual files in the src_dir
    if inputs.verbosity >= 2 {
        warnings.push(format!("Starting declaration writing phase."));
    }
    for (identifier, declaration) in global_declarations.into_iter() {
        let level = *declaration_levels.get(&identifier).unwrap_or(&0);
        let declaration_type = match &declaration.item {
            DeclarationItem::Const(_) => "const",
            DeclarationItem::Struct(item_str) => {
                let parsed_item: syn::ItemStruct = syn::parse_str(item_str)
                    .context(format!("Failed to parse struct string: {}", item_str))?;
                if parsed_item.attrs.iter().any(|attr| attr.path().is_ident("proc_macro")) {
                    has_proc_macros = true;
                }
                "struct"
            },
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
        if inputs.verbosity >= 2 {
            warnings.push(format!("  Processing declaration '{}' of type '{}' at level {}", identifier, declaration_type, level));
        }

        let declaration_dir = inputs.canonical_output_root
            .join("rust-bootstrap-core")
            .join("src")
            .join(format!("level_{:02}", level))
            .join("src")
            .join(format!("{}_t", declaration_type))
            .join(&declaration.crate_name);
        tokio::fs::create_dir_all(&declaration_dir)
            .await
            .context(format!("Failed to create declaration directory: {}", declaration_dir.display()))?;
        if inputs.verbosity >= 2 {
            if declaration_dir.exists() {
                warnings.push(format!("  Declaration directory already exists or was created: {}", declaration_dir.display()));
            } else {
                warnings.push(format!("  Failed to create declaration directory (but context handled it): {}", declaration_dir.display()));
            }
        }

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
            if inputs.verbosity >= 3 {
                warnings.push(format!("    Adding use statement: {}", dep));
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
        if inputs.verbosity >= 3 {
            warnings.push(format!("    Converting item to token stream for '{}'.", identifier));
        }
        file_content.push_str(&item_token_stream.to_string());

        tokio::fs::write(&output_file_path, file_content)
            .await
            .context(format!("Failed to write declaration to file: {}", output_file_path.display()))?;
        if inputs.verbosity >= 2 {
            warnings.push(format!("Wrote declaration to file: {}", output_file_path.display()));
        }

        generated_module_names.push(identifier);
        if declaration.is_proc_macro {
            if inputs.verbosity >= 3 {
                warnings.push(format!("  Declaration '{}' is a procedural macro.", declaration.get_identifier()));
            }
            has_proc_macros = true;
        }
    }

    // Print generated module names and proc macro flag for the orchestrating script
    warnings.push(format!("GENERATED_MODULE_NAMES: {}", generated_module_names.join(",")));
    warnings.push(format!("HAS_PROC_MACROS: {}", has_proc_macros));

    Ok(warnings)
}



pub async fn extract_declarations_from_single_file(
    file_path: &Path,
    _rustc_info: &RustcInfo,
    crate_name: &str,
    verbosity: u8,
    warnings: &mut Vec<String>,
    _canonical_output_root: &Path,
) -> anyhow::Result<ExtractionResult> {
    if verbosity >= 2 {
        warnings.push(format!("  [split-expanded-lib] extract_declarations_from_single_file: Processing file: {}", file_path.display()));
    }

    let mut file_content = tokio::fs::read_to_string(file_path)
        .await
        .context(format!("Failed to read file: {}", file_path.display()))?;

    // Strip ANSI escape codes
    let re_ansi = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    file_content = re_ansi.replace_all(&file_content, "").to_string();

    // Strip documentation comments (lines starting with ///)
    let re_docs = Regex::new(r"(?m)^\s*///.*$\n?").unwrap();
    file_content = re_docs.replace_all(&file_content, "").to_string();

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

    let declarations;
    let visitor_declarations_len;
    { // New scope to limit the lifetime of `visitor`
        let mut visitor = DeclsVisitor::new(file_path.to_path_buf(), crate_name.to_string(), verbosity, file_metadata.extern_crates.clone(), warnings);
        visitor.visit_file(&syntax_tree);
        declarations = visitor.declarations;
        visitor_declarations_len = declarations.len();
    } // `visitor` goes out of scope here, releasing the mutable borrow on `warnings`

    if verbosity >= 2 {
        warnings.push(format!("  [split-expanded-lib] extract_declarations_from_single_file: Found {} declarations in {}", visitor_declarations_len, file_path.display()));
    }

    Ok(ExtractionResult {
        declarations,
        errors,
        file_metadata,
        public_symbols,
    })
}
