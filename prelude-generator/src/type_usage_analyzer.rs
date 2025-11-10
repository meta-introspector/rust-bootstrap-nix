use anyhow::Context;
use std::fs;
use crate::Args;
use walkdir::WalkDir;
use syn::{self, visit::Visit};
use std::collections::HashMap;
use crate::expression_info::ExpressionInfo;
use crate::type_usage_visitor::TypeUsageVisitor;
use crate::report_generator::generate_report;
use crate::{struct_lattice_info::StructLatticeInfo, enum_lattice_info::EnumLatticeInfo, impl_lattice_info::ImplLatticeInfo};
use crate::types::CollectedAnalysisData;
use toml;
use std::path::{Path, PathBuf}; // Added for path canonicalization
use std::collections::HashSet; // Added for canonicalized exclude paths
use sha2::{Sha256, Digest}; // Added for file hashing
use serde::{Serialize, Deserialize}; // Added for caching CollectedAnalysisData

// Helper function to canonicalize a single PathBuf
fn canonicalize_path_buf(path: &Path) -> anyhow::Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path.display()))
}

// Helper function to canonicalize a vector of exclude paths
fn canonicalize_exclude_paths(
    exclude_paths: &[PathBuf],
    project_root: &Path,
) -> anyhow::Result<HashSet<PathBuf>> {
    let mut canonicalized_set = HashSet::new();
    for path in exclude_paths {
        let absolute_path = if path.is_absolute() {
            path.clone()
        } else {
            project_root.join(path)
        };
        canonicalized_set.insert(canonicalize_path_buf(&absolute_path)?);
    }
    Ok(canonicalized_set)
}

// Function to calculate SHA256 hash of a file's content
fn calculate_file_hash(file_path: &Path) -> anyhow::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

// Define a struct to hold cached data for a single file
#[derive(Debug, Serialize, Deserialize)]
struct FileAnalysisCache {
    expressions: HashMap<String, ExpressionInfo>,
    struct_lattices: HashMap<String, StructLatticeInfo>,
    enum_lattices: HashMap<String, EnumLatticeInfo>,
    impl_lattices: HashMap<String, ImplLatticeInfo>,
}

// Function to load analysis data from cache
async fn load_from_cache(cache_file_path: &Path) -> anyhow::Result<Option<FileAnalysisCache>> {
    if cache_file_path.exists() {
        let cached_content = tokio::fs::read_to_string(cache_file_path).await?;
        let cached_data: FileAnalysisCache = serde_json::from_str(&cached_content)?;
        Ok(Some(cached_data))
    } else {
        Ok(None)
    }
}

// Function to save analysis data to cache
async fn save_to_cache(cache_file_path: &Path, data: &FileAnalysisCache) -> anyhow::Result<()> {
    tokio::fs::create_dir_all(cache_file_path.parent().unwrap()).await?;
    let serialized_data = serde_json::to_string_pretty(data)?;
    tokio::fs::write(cache_file_path, serialized_data).await?;
    Ok(())
}

pub async fn analyze_type_usage(args: &Args) -> anyhow::Result<CollectedAnalysisData> { // Modified return type
    println!("Debug: analyze_type_usage received dry_run = {}", args.dry_run);
    println!("Running type usage analysis...");

    let max_expression_depth = args.max_expression_depth.context("Max expression depth must be specified for type usage analysis")?;
    let output_path = args.output_type_usage_report.as_ref().context("Output path for type usage report must be specified")?;

    println!("Max Expression Depth: {}", max_expression_depth);
    println!("Output Report Path: {:?}", output_path);

    // Canonicalize args.path
    let canonical_args_path = canonicalize_path_buf(&args.path)?;

    // Canonicalize exclude paths once
    let canonical_exclude_paths = canonicalize_exclude_paths(&args.exclude_paths, &canonical_args_path)?;

    // Define cache directory
    let cache_dir = canonical_args_path.join(".prelude_cache").join("type_analysis");
    tokio::fs::create_dir_all(&cache_dir).await.context("Failed to create type analysis cache directory")?;

    let mut all_expression_info: HashMap<String, ExpressionInfo> = HashMap::new();
    let mut all_struct_lattices: HashMap<String, StructLatticeInfo> = HashMap::new();
    let mut all_enum_lattices: HashMap<String, EnumLatticeInfo> = HashMap::new();
    let mut all_impl_lattices: HashMap<String, ImplLatticeInfo> = HashMap::new();

    for entry in WalkDir::new(&canonical_args_path)
        .into_iter()
        .filter_entry(|e| {
            // Canonicalize entry path for comparison
            if let Ok(canonical_entry_path) = canonicalize_path_buf(e.path()) {
                !canonical_exclude_paths.iter().any(|exclude_path| {
                            canonical_entry_path.starts_with(exclude_path)
                        })
            } else {
                // If canonicalization fails, treat as not excluded (or handle error)
                true
            }
        })
    {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().map_or(false, |ext| ext == "rs") {
            let file_hash = calculate_file_hash(file_path)?;
            let cache_file_name = format!("{}.json", file_hash);
            let cache_file_path = cache_dir.join(cache_file_name);

            if let Some(cached_data) = load_from_cache(&cache_file_path).await? {
                println!("Loaded from cache: {}", file_path.display());
                all_expression_info.extend(cached_data.expressions);
                all_struct_lattices.extend(cached_data.struct_lattices);
                all_enum_lattices.extend(cached_data.enum_lattices);
                all_impl_lattices.extend(cached_data.impl_lattices);
                continue; // Skip processing this file
            }

            println!("Processing file for type usage analysis: {}", file_path.display());

            let file_content = fs::read_to_string(&file_path)
                .context(format!("Failed to read file: {:?}", file_path))?;

            let file = match syn::parse_file(&file_content) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Warning: Could not parse file {}: {}", file_path.display(), e);
                    continue;
                }
            };

            let mut visitor = TypeUsageVisitor::new(max_expression_depth);
            visitor.visit_file(&file);

            let file_cache_data = FileAnalysisCache {
                expressions: visitor.expressions,
                struct_lattices: visitor.struct_lattices,
                enum_lattices: visitor.enum_lattices,
                impl_lattices: visitor.impl_lattices,
            };

            // Save to cache
            save_to_cache(&cache_file_path, &file_cache_data).await?;

            all_expression_info.extend(file_cache_data.expressions);
            all_struct_lattices.extend(file_cache_data.struct_lattices);
            all_enum_lattices.extend(file_cache_data.enum_lattices);
            all_impl_lattices.extend(file_cache_data.impl_lattices);
        }
    }

    generate_report(&all_expression_info, max_expression_depth, output_path, &all_struct_lattices, &all_enum_lattices, &all_impl_lattices)?;

    let collected_data = CollectedAnalysisData { // Construct CollectedAnalysisData
        expressions: all_expression_info,
        struct_lattices: all_struct_lattices,
        enum_lattices: all_enum_lattices,
        impl_lattices: all_impl_lattices,
    };

    if let Some(toml_output_path) = &args.output_toml_report {
        let toml_content = toml::to_string_pretty(&collected_data)
            .context("Failed to serialize collected analysis data to TOML")?;
        fs::write(toml_output_path, toml_content)
            .context(format!("Failed to write TOML report to {:?}", toml_output_path))?;
        println!("TOML report saved to {:?}", toml_output_path);
    }

    if let Some(json_output_path) = &args.output_analysis_data_json {
        let json_content = serde_json::to_string_pretty(&collected_data)
            .context("Failed to serialize collected analysis data to JSON")?;
        fs::write(json_output_path, json_content)
            .context(format!("Failed to write JSON report to {:?}", json_output_path))?;
        println!("JSON report saved to {:?}", json_output_path);
    }

    println!("Type usage analysis completed. Report saved to {:?}", output_path);
    Ok(collected_data) // Return CollectedAnalysisData
}
