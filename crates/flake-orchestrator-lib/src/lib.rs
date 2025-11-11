use anyhow::{Result, Context};
use std::path::Path; // Keep Path, remove PathBuf
use tokio::fs;

// Re-export necessary types from dependencies
pub use expanded_code_collector::{collect_expanded_code, RustcInfo as ExpandedCodeCollectorRustcInfo};
pub use split_expanded_lib::{process_expanded_manifest, ProcessExpandedManifestInputs, RustcInfo as SplitExpandedLibRustcInfo};
pub use flake_template_generator::flake_generator::generate_flake_nix_content; // Import from flake_generator module

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

    // 3. Prepare the generated flake directory as a Rust crate
    println!("Step 3: Preparing flake output directory as a Rust crate...");
    let flake_src_dir = flake_output_dir.join("src");
    fs::create_dir_all(&flake_src_dir).await
        .context(format!("Failed to create flake src directory: {}", flake_src_dir.display()))?;

    // Create Cargo.toml for the generated crate
    let cargo_toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
"#, flake_component); // Using flake_component as package name
    fs::write(flake_output_dir.join("Cargo.toml"), cargo_toml_content.as_bytes()).await
        .context(format!("Failed to write Cargo.toml to {}", flake_output_dir.display()))?;

    // Copy generated .rs files and create lib.rs with mod statements
    let mut lib_rs_content = String::new();
    let mut read_dir = fs::read_dir(expanded_output_dir).await
        .context(format!("Failed to read expanded output directory: {}", expanded_output_dir.display()))?;

    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let module_name = file_name.trim_end_matches(".rs");
            lib_rs_content.push_str(&format!("pub mod {};\n", module_name));
            fs::copy(&path, flake_src_dir.join(file_name)).await
                .context(format!("Failed to copy {} to {}", path.display(), flake_src_dir.join(file_name).display()))?;
        }
    }
use anyhow::{Result, Context};
use std::path::Path; // Keep Path, remove PathBuf
use tokio::fs;
use tokio::process::Command; // Add this import

// Re-export necessary types from dependencies
pub use expanded_code_collector::{collect_expanded_code, RustcInfo as ExpandedCodeCollectorRustcInfo};
pub use split_expanded_lib::{process_expanded_manifest, ProcessExpandedManifestInputs, RustcInfo as SplitExpandedLibRustcInfo};
pub use flake_template_generator::flake_generator::generate_flake_nix_content; // Import from flake_generator module

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

    // 3. Prepare the generated flake directory as a Rust crate
    println!("Step 3: Preparing flake output directory as a Rust crate...");
    let flake_src_dir = flake_output_dir.join("src");
    fs::create_dir_all(&flake_src_dir).await
        .context(format!("Failed to create flake src directory: {}", flake_src_dir.display()))?;

    // Create Cargo.toml for the generated crate
    let cargo_toml_content = format!(r#"
[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
"#, flake_component); // Using flake_component as package name
    fs::write(flake_output_dir.join("Cargo.toml"), cargo_toml_content.as_bytes()).await
        .context(format!("Failed to write Cargo.toml to {}", flake_output_dir.display()))?;

    // Copy generated .rs files and create lib.rs with mod statements
    let mut lib_rs_content = String::new();
    let mut read_dir = fs::read_dir(expanded_output_dir).await
        .context(format!("Failed to read expanded output directory: {}", expanded_output_dir.display()))?;

    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let module_name = file_name.trim_end_matches(".rs");
            lib_rs_content.push_str(&format!("pub mod {};\n", module_name));
            fs::copy(&path, flake_src_dir.join(file_name)).await
                .context(format!("Failed to copy {} to {}", path.display(), flake_src_dir.join(file_name).display()))?;
        }
    }
    fs::write(flake_src_dir.join("lib.rs"), lib_rs_content.as_bytes()).await
        .context(format!("Failed to write lib.rs to {}", flake_src_dir.join("lib.rs").display()))?;

    // 4. Run cargo2nix to generate Cargo.nix
    println!("Step 4: Running cargo2nix to generate Cargo.nix...");
    let cargo2nix_output = Command::new("cargo2nix")
        .arg("--workspace")
        .arg("--output")
        .arg(flake_output_dir.join("Cargo.nix"))
        .current_dir(flake_output_dir)
        .output()
        .await
        .context("Failed to execute cargo2nix command")?;

    if !cargo2nix_output.status.success() {
        anyhow::bail!(
            "cargo2nix failed with status: {}\nStdout: {}\nStderr: {}",
            cargo2nix_output.status,
            String::from_utf8_lossy(&cargo2nix_output.stdout),
            String::from_utf8_lossy(&cargo2nix_output.stderr)
        );
    }
    println!("Cargo.nix generated successfully by cargo2nix.");

    // 5. Generate Nix flake
    println!("Step 5: Generating Nix flake...");
    let nixpkgs_url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify".to_string(); // Hardcoded for now
    let system_arch = "aarch64-linux"; // Hardcoded for now

    let cargo2nix_url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify".to_string(); // Use the correct URL

    let flake_nix_content = generate_flake_nix_content(
        &nixpkgs_url,
        &cargo2nix_url, // Pass the cargo2nix_url
        system_arch,
        use_rustc_wrapper,
        None, // Pass None for _rustc_wrapper_path
    );

    let output_flake_nix_path = flake_output_dir.join("flake.nix");
    fs::write(&output_flake_nix_path, flake_nix_content.as_bytes()).await
        .context(format!("Failed to write flake.nix to {}", output_flake_nix_path.display()))?;

    println!("Flake generated successfully at {}", flake_output_dir.display());

    // 6. Initialize Git repository and commit generated files
    println!("Step 6: Initializing Git repository and committing generated files...");
    git_utils::init_repo(flake_output_dir)
        .context(format!("Failed to initialize Git repository in {}", flake_output_dir.display()))?;
    git_utils::add_all(flake_output_dir)
        .context(format!("Failed to add all files to Git repository in {}", flake_output_dir.display()))?;
    git_utils::commit_files(
        flake_output_dir,
        &format!("feat: Initial commit for generated flake for {}", flake_component),
        "Gemini", // Author name
        "gemini@google.com", // Author email
    ).context(format!("Failed to commit files in {}", flake_output_dir.display()))?;
    println!("Generated flake committed to its own Git repository.");

    Ok(())
}
