use anyhow::{Result, Context};
use std::path::Path; // Keep Path, remove PathBuf
use tokio::fs;

// Re-export necessary types from dependencies
pub use expanded_code_collector::{collect_expanded_code, RustcInfo as ExpandedCodeCollectorRustcInfo};
pub use split_expanded_lib::{process_expanded_manifest, ProcessExpandedManifestInputs, RustcInfo as SplitExpandedLibRustcInfo};
pub use flake_template_generator::generate_flake_nix_content; // Removed Args as it's not directly accessible

pub async fn orchestrate_flake_generation(
    metadata_path: &Path,
    expanded_output_dir: &Path,
    flake_lock_path: &Path,
    layer: Option<u32>,
    package_filter: Option<String>,
    dry_run: bool,
    force: bool,
    rustc_version: String,
    rustc_host: String,
    project_root: &Path,
    json_summary_path: &Path,
    log_output_dir: &Path,
    flake_output_dir: &Path,
    flake_component: String,
    flake_arch: String,
    flake_phase: String,
    flake_step: String,
    use_rustc_wrapper: bool,
) -> Result<()> {
    println!("Orchestrating flake generation...");

    // 1. Collect expanded code
    println!("Step 1: Collecting expanded code...");
    let flake_lock_content = fs::read_to_string(flake_lock_path)
        .await
        .context(format!("Failed to read flake.lock file: {}", flake_lock_path.display()))?;
    let flake_lock_json: serde_json::Value = serde_json::from_str(&flake_lock_content)
        .context(format!("Failed to parse flake.lock JSON from: {}", flake_lock_path.display()))?;

    expanded_code_collector::collect_expanded_code(
        metadata_path,
        expanded_output_dir,
        &flake_lock_json,
        layer,
        package_filter.clone(), // Clone for expanded_code_collector
        dry_run,
        force,
        rustc_version.clone(),
        rustc_host.clone(),
    ).await?;

    // 2. Process expanded manifest (split declarations)
    println!("Step 2: Processing expanded manifest...");
    let split_expanded_lib_rustc_info = SplitExpandedLibRustcInfo {
        version: rustc_version.clone(),
        host: rustc_host.clone(),
    };

    let process_inputs = ProcessExpandedManifestInputs {
        expanded_manifest_path: &expanded_output_dir.join("expanded_manifest.json"),
        project_root: project_root,
        rustc_info: &split_expanded_lib_rustc_info,
        verbosity: 3, // Assuming a default verbosity for now
        layer: layer,
        canonical_output_root: project_root,
        package_filter: package_filter,
        json_summary_path: Some(json_summary_path),
        log_output_dir: Some(log_output_dir),
    };

    split_expanded_lib::process_expanded_manifest(process_inputs).await?;

    // 3. Generate Nix flake
    println!("Step 3: Generating Nix flake...");
    fs::create_dir_all(flake_output_dir).await
        .context(format!("Failed to create flake output directory: {}", flake_output_dir.display()))?;

    let nixpkgs_url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify".to_string(); // Hardcoded for now
    let system_arch = "aarch64-linux"; // Hardcoded for now

    let flake_nix_content = flake_template_generator::generate_flake_nix_content(
        &nixpkgs_url,
        system_arch,
        use_rustc_wrapper,
        // Removed `None` for `rustc_path_override` as it's not expected by the function
    );

    let output_flake_nix_path = flake_output_dir.join("flake.nix");
    fs::write(&output_flake_nix_path, flake_nix_content.as_bytes()).await
        .context(format!("Failed to write flake.nix to {}", output_flake_nix_path.display()))?;

    println!("Flake generated successfully at {}", flake_output_dir.display());

    Ok(())
}
