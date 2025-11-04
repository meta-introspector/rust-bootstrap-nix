use clap::Parser;
use anyhow::Context;
use std::path::PathBuf;
use split_expanded_lib::{extract_declarations_from_single_file, RustcInfo, DeclarationItem, Declaration, ErrorSample};
use std::collections::{HashMap, HashSet};
use std::fs; // Added
use std::io::Write;

use quote::quote;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable verbose output.
    #[arg(short, long)]
    pub verbose: bool,

    /// Paths to the input Rust files (e.g., expanded .rs files).
    #[arg(long)]
    pub files: Vec<PathBuf>,

    /// Directory to output the generated declaration files.
    #[clap(short, long, value_parser, required = true)]
    project_root: PathBuf,

    /// Rustc version (e.g., "1.89.0").
    #[arg(long)]
    pub rustc_version: String,

    /// Rustc host triple (e.g., "aarch64-unknown-linux-gnu").
    #[arg(long)]
    pub rustc_host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Create RustcInfo from command-line arguments
    let rustc_info = RustcInfo {
        version: args.rustc_version,
        host: args.rustc_host,
    };

    // Create project root and src directory if they don't exist
    let src_dir = args.project_root.join("src");
    fs::create_dir_all(&src_dir)
        .context(format!("Failed to create project src directory: {}", src_dir.display()))?;
    if args.verbose {
        println!("Created project src directory: {}", src_dir.display());
        std::io::stdout().flush().unwrap();
    }
    let mut global_declarations: HashMap<String, Declaration> = HashMap::new();
    let mut all_errors: Vec<ErrorSample> = Vec::new();

    for file_path in &args.files {
        if args.verbose {
            println!("Processing file: {}", file_path.display());
            std::io::stdout().flush().unwrap();
        }

        let file_stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown_crate");
        let crate_name = file_stem.trim_start_matches(".expand_output_");

        let (declarations, errors, _file_metadata) = extract_declarations_from_single_file(
            file_path,
            &rustc_info,
            crate_name,
        ).await?;

        for decl in declarations {
            global_declarations.insert(decl.get_identifier(), decl);
        }
        all_errors.extend(errors);
    }

    if !all_errors.is_empty() {
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
    for (decl_id, decl) in &global_declarations {
        let mut current_resolved_dependencies = HashSet::new();

        // Resolve referenced types (internal and external)
        for referenced_type in &decl.referenced_types {
            if let Some(resolved_decl) = global_declarations.get(referenced_type) {
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                // This is an external type dependency
                current_resolved_dependencies.insert(referenced_type.clone());
            }
        }

        // Resolve referenced functions (internal and external)
        for referenced_fn in &decl.referenced_functions {
            if let Some(resolved_decl) = global_declarations.get(referenced_fn) {
                current_resolved_dependencies.insert(format!("{}::{}", resolved_decl.crate_name, resolved_decl.get_identifier()));
            } else {
                // This is an external function dependency
                current_resolved_dependencies.insert(referenced_fn.clone());
            }
        }

        // Also consider required_imports as dependencies if they are not internal declarations
        for import in &decl.required_imports {
            if !global_declarations.contains_key(import) {
                current_resolved_dependencies.insert(import.clone());
            }
        }

        dependencies_to_resolve.insert(decl_id.clone(), current_resolved_dependencies);
    }

    // Second pass: Populate direct_dependencies and resolved_dependencies
    for (decl_id, decl) in global_declarations.iter_mut() {
        if let Some(resolved_deps) = dependencies_to_resolve.remove(decl_id) {
            decl.direct_dependencies = resolved_deps.iter().map(|s| s.split("::").last().unwrap_or(s).to_string()).collect();
            decl.resolved_dependencies = resolved_deps;
        }
    }

    // Phase 3: Implement Layering Algorithm
    let mut declaration_levels: HashMap<String, usize> = HashMap::new();
    let mut changed = true;
    let mut max_level = 0;

    // Initialize all declarations to level 0
    for (decl_id, _) in &global_declarations {
        declaration_levels.insert(decl_id.clone(), 0);
    }

    while changed {
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
                declaration_levels.insert(decl_id.clone(), new_level);
                changed = true;
                max_level = max_level.max(new_level);
            }
        }
    }

    println!("Max dependency level found: {}", max_level);

    let mut generated_module_names: Vec<String> = Vec::new();
    let mut has_proc_macros = false;

    // Write all declarations to individual files in the src_dir
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

        let declaration_dir = src_dir.join(format!("level_{}/{}", level, declaration_type));
        fs::create_dir_all(&declaration_dir)
            .context(format!("Failed to create declaration directory: {}", declaration_dir.display()))?;
        if args.verbose {
            println!("Created declaration directory: {}", declaration_dir.display());
            std::io::stdout().flush().unwrap();
        }

        let output_file_path = declaration_dir.join(format!("{}.rs", identifier));

        let mut file_content = String::new();

        // Add use statements
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
        if args.verbose {
            println!("Wrote declaration to file: {}", output_file_path.display());
            std::io::stdout().flush().unwrap();
        }

        generated_module_names.push(identifier);
        if declaration.is_proc_macro {
            has_proc_macros = true;
        }
    }

    // Print generated module names and proc macro flag for the orchestrating script
    println!("GENERATED_MODULE_NAMES: {}", generated_module_names.join(","));
    println!("HAS_PROC_MACROS: {}", has_proc_macros);

    Ok(())
}
