use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Item, ItemUse};
use walkdir::WalkDir;
use std::collections::HashSet;
use prettyplease;
use serde::{Deserialize, Serialize};
use std::process::Command;
use syn::UseTree; // Add UseTree
use tempfile;
use sha2::{Sha256, Digest};
use indoc::indoc;
use toml;

// Struct to hold rustc version and host triple
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RustcInfo {
    version: String,
    host: String,
}

// Helper to get rustc version and host triple
fn get_rustc_info() -> Result<RustcInfo> {
    let output = Command::new("rustc")
        .arg("--version")
        .arg("--verbose")
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "rustc --version --verbose failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_line = stdout.lines().find(|line| line.starts_with("rustc "));
    let host_line = stdout.lines().find(|line| line.starts_with("host: "));

    let version = version_line
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("unknown")
        .to_string();
    let host = host_line
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("unknown")
        .to_string();

    Ok(RustcInfo { version, host })
}

fn expand_macros_and_parse(file_path: &Path, content: &str, rustc_info: &RustcInfo, cache_dir: &Path) -> Result<syn::File> {
    // Calculate content hash
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let content_hash = format!("{:x}", hasher.finalize());

    // Create a unique cache key based on file hash, rustc info, and rustc flags
    let cache_key = format!(
        "expanded_{}_{}_{}_{}_{}",
        content_hash,
        rustc_info.version,
        rustc_info.host,
        "lib", // --crate-type
        "2021" // --edition
    );
    let cached_file_path = cache_dir.join(cache_key);

    // Check if expanded code is already cached
    if cached_file_path.exists() {
        println!("      -> Using cached expanded code for: {}", file_path.display());
        let expanded_code = fs::read_to_string(&cached_file_path)
            .with_context(|| format!("Failed to read cached expanded code for {}", file_path.display()))?;
        return syn::parse_file(&expanded_code).with_context(|| format!("Failed to parse cached expanded code for {}", file_path.display()));
    }

    // If not cached, perform expansion by creating a temporary crate
    let temp_crate_dir = tempfile::tempdir()?;
    let temp_crate_path = temp_crate_dir.path();

    // Create Cargo.toml for the temporary crate
    let cargo_toml_content = indoc! {
        r#"#,
        "[package]
        name = \"temp_crate\"
        version = \"0.1.0\"
        edition = \"2021\"

        [dependencies]
        serde = {{ version = \"1.0\", features = [\"derive\"] }}
        serde_json = \"1.0\"
        anyhow = \"1.0\"
        # Add other common dependencies that might contain macros
        "#
    };
    fs::write(temp_crate_path.join("Cargo.toml"), cargo_toml_content)?;

    // Create src directory
    let temp_src_dir = temp_crate_path.join("src");
    fs::create_dir(&temp_src_dir)?;

    // Write the original content to a file within the temporary crate
    let temp_rs_file_name = file_path.file_name().unwrap_or_else(|| "temp_file.rs".as_ref());
    let temp_rs_file_path = temp_src_dir.join(temp_rs_file_name);
    fs::write(&temp_rs_file_path, content)?;

    // Create lib.rs that includes the target file
    let lib_rs_content = format!(
        "#![allow(unused_imports)]\n#![allow(dead_code)]\ninclude!(\"{}\");\n",
        temp_rs_file_name.to_string_lossy() // Pass the full file name
    );
    fs::write(temp_src_dir.join("lib.rs"), lib_rs_content)?;

    println!("        -> PATH environment variable: {:?}", std::env::var("PATH"));
    println!("        -> Running cargo rustc -Zunpretty=expanded for: {}", file_path.display());
    let output = Command::new("cargo")
        .arg("rustc")
        .arg("--")
        .arg("-Zunpretty=expanded")
        .arg("--crate-type")
        .arg("lib")
        .current_dir(temp_crate_path)
        .output()?;

    println!("        -> cargo rustc status for {}: {}", file_path.display(), output.status);
    println!("        -> cargo rustc stdout for {}: {}", file_path.display(), String::from_utf8_lossy(&output.stdout));
    if !output.status.success() {
        println!("        -> cargo rustc stderr for {}: {}", file_path.display(), String::from_utf8_lossy(&output.stderr));
        anyhow::bail!(
            "cargo rustc -Zunpretty=expanded failed for {}: {}\nStderr: {}",
            file_path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let expanded_code = String::from_utf8_lossy(&output.stdout).to_string();

    // Extract the relevant expanded code for the specific file
    // This is a heuristic and might need refinement.
    let search_string = format!("// {}\n", temp_rs_file_name.to_string_lossy());
    let start_index = expanded_code.find(&search_string).unwrap_or(0);
    let end_index = expanded_code[start_index..].find("// ").map_or(expanded_code.len(), |i| start_index + i);
    let relevant_expanded_code = expanded_code[start_index..end_index].to_string();

    println!("        -> Writing expanded code to cache for: {}", file_path.display());
    // Cache the expanded code
    fs::write(&cached_file_path, &relevant_expanded_code)
        .with_context(|| format!("Failed to write expanded code to cache for {}", file_path.display()))?;
    println!("      -> Wrote expanded code to cache: {}", cached_file_path.display());

    println!("        -> Parsing expanded code for: {}", file_path.display());
    syn::parse_file(&relevant_expanded_code).with_context(|| format!("Failed to parse expanded code for {}", file_path.display()))
}

#[derive(Deserialize, Debug)]
struct Metadata {
    packages: Vec<Package>,
    #[allow(dead_code)]
    workspace_root: PathBuf,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    manifest_path: PathBuf,
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    #[serde(default)]
    lib: Option<LibSection>,
}

#[derive(Deserialize, Debug)]
struct LibSection {
    #[serde(rename = "proc-macro", default)]
    proc_macro: bool,
}

fn is_proc_macro_crate(crate_root: &Path) -> Result<bool> {
    let cargo_toml_path = crate_root.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Ok(false);
    }
    let content = fs::read_to_string(&cargo_toml_path)
        .context(format!("Failed to read Cargo.toml for {}", crate_root.display()))?;
    let cargo_toml: CargoToml = toml::from_str(&content)
        .context(format!("Failed to parse Cargo.toml for {}", crate_root.display()))?;

    Ok(cargo_toml.lib.map_or(false, |lib| lib.proc_macro))
}

// Helper function to flatten UseTree into individual use paths
fn flatten_use_tree(
    base_path: &mut Vec<String>,
    use_tree: &UseTree,
    flat_uses: &mut HashSet<String>,
) {
    match use_tree {
        UseTree::Path(path) => {
            base_path.push(path.ident.to_string());
            flatten_use_tree(base_path, &path.tree, flat_uses);
            base_path.pop();
        }
        UseTree::Name(name) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&name.ident.to_string());
            flat_uses.insert(format!("use {};", full_path));
        }
        UseTree::Rename(rename) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str(&rename.ident.to_string());
            flat_uses.insert(format!("use {} as {};", full_path, rename.rename.to_string()));
        }
        UseTree::Glob(_glob) => {
            let mut full_path = base_path.join("::");
            if !full_path.is_empty() {
                full_path.push_str("::");
            }
            full_path.push_str("* ");
            flat_uses.insert(format!("use {};", full_path));
        }
        UseTree::Group(group) => {
            for tree in group.items.iter() {
                flatten_use_tree(base_path, tree, flat_uses);
            }
        }
    }
}

/// Extracts unique 'use' statements from Rust files within a given repository root.
pub fn extract_unique_use_statements(repo_root: &Path) -> Result<HashSet<String>> {
    let mut unique_use_statements = HashSet::new();

    let rustc_info = get_rustc_info()?;
    let cache_dir = repo_root.join(".prelude_cache"); // Use repo_root for cache dir
    fs::create_dir_all(&cache_dir).context("Failed to create prelude cache directory")?;

    for entry in WalkDir::new(repo_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let file_path = entry.path();
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Warning: Failed to read file {}: {}", file_path.display(), e);
                continue;
            }
        };

        let ast = match expand_macros_and_parse(file_path, &content, &rustc_info, &cache_dir) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Warning: Failed to parse Rust file (after macro expansion) {}: {}", file_path.display(), e);
                continue;
            }
        };

        for item in ast.items {
            if let Item::Use(use_item) = item {
                let mut base_path = Vec::new();
                flatten_use_tree(&mut base_path, &use_item.tree, &mut unique_use_statements);
            }
        }
    }
    Ok(unique_use_statements)
}

/// Generates test files for each unique 'use' statement.
pub fn generate_use_statement_test_files(
    output_dir: &Path,
    use_statements: HashSet<String>,
) -> Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory {}", output_dir.display()))?;

    for (i, use_statement) in use_statements.iter().enumerate() {
        let file_name = format!("use_test_{}.rs", i);
        let file_path = output_dir.join(file_name);
        fs::write(&file_path, use_statement.as_bytes())
            .with_context(|| format!("Failed to write use statement test file to {}", file_path.display()))?;
    }

    println!("Generated {} use statement test files in {}", use_statements.len(), output_dir.display());
    Ok(())
}
