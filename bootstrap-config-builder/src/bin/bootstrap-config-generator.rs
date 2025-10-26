use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use log::{info, debug}; // Import log macros
use toml;
use bootstrap_config_builder::config::AppConfig;
use std::path::{PathBuf}; // Added for path manipulation

use bootstrap_config_builder::preconditions;
use bootstrap_config_builder::utils::validate_project_root::validate_project_root;
use bootstrap_config_builder::utils::construct_config_content::construct_config_content;
use bootstrap_config_builder::args::Args;
use bootstrap_config_builder::utils::find_nix_package_store_path::find_nix_package_store_path;

fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger

    let args = Args::parse();
    debug!("Raw CLI Arguments: {:?}\n", args);

    let mut app_config = if let Some(config_file_path) = &args.config_file {
        info!("Loading configuration from file: {:?}\n", config_file_path);
        let config_content = fs::read_to_string(config_file_path)
            .context(format!("Failed to read config file: {:?}\n", config_file_path))?;
        toml::from_str(&config_content)
            .context(format!("Failed to parse config file: {:?}\n", config_file_path))? 
    } else {
        AppConfig::default()
    };

    app_config.merge_with_args(&args);
    info!("Final merged configuration: {:?}\n", app_config);

    // If build_rustc_version is provided, we are in lattice generation mode
    if let Some(build_rustc_version) = app_config.build_rustc_version.clone() {
        info!("Lattice generation mode enabled for rustc version: {}\n", build_rustc_version);

        // Retrieve necessary paths and parameters from app_config
        let solana_rustc_path = app_config.solana_rustc_path.as_ref()
            .context("solana_rustc_path is required for lattice generation")?
            .to_str().context("solana_rustc_path contains non-UTF8 characters")?;
        let cargo_path = app_config.cargo_path.as_ref()
            .context("cargo_path is required for lattice generation")?
            .to_str().context("cargo_path contains non-UTF8 characters")?;
        let project_root = app_config.project_root.as_ref()
            .context("project_root is required for lattice generation")?
            .to_str().context("project_root contains non-UTF8 characters")?;
        let rust_src_flake_path = app_config.rust_src_flake_path.as_ref()
            .context("rust_src_flake_path is required for lattice generation")?
            .to_str().context("rust_src_flake_path contains non-UTF8 characters")?;
        let architecture = app_config.architecture.as_deref().unwrap_or("aarch64-linux");
        let stage = app_config.stage.as_deref().unwrap_or("stage0");
        let step = app_config.step.as_deref().unwrap_or("step1-configure");

        // Resolve the rustcPath for the current build_rustc_version
        let resolved_build_rustc_path = find_nix_package_store_path(
            "rustc",
            Some(&build_rustc_version),
        )?.context(format!("Could not find rustc path for version {}", build_rustc_version))?; // Corrected call

        // Construct the output directory
        let output_dir = PathBuf::from(format!(
            "flakes/{}/{}/{}/{}",
            build_rustc_version, architecture, stage, step
        ));
        fs::create_dir_all(&output_dir)
            .context(format!("Failed to create output directory: {:?}", output_dir))?;

        // Generate flake.nix
        let flake_nix_path = output_dir.join("flake.nix");
        let flake_nix_content = format!(
            r#"{{
  description = "Test flake for rustc {}";

  inputs = {{
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  }};

  outputs = {{ self, nixpkgs }}:
    let
      pkgs = import nixpkgs {{
        system = "{}";
      }};
      rustcPath = "{}"; # This is the *source* rustc used to build the next stage
    in
    {{
      devShells.{}.default = pkgs.mkShell {{
        buildInputs = [
          pkgs.cargo
        ];
        RUSTC = rustcPath;
      }};
    }};
}}"#,
            build_rustc_version, architecture, solana_rustc_path, architecture
        );
        fs::write(&flake_nix_path, flake_nix_content)
            .context(format!("Failed to write flake.nix to {:?}\n", flake_nix_path))?;
        info!("Generated flake.nix for rustc {} at {:?}\n", build_rustc_version, flake_nix_path);

        // Generate generated_config.toml
        let config_toml_path = output_dir.join(format!("generated_config_{}.toml", build_rustc_version));
        let config_content = construct_config_content(
            architecture, // system
            project_root, // flake_path_str
            app_config.nixpkgs_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.rust_overlay_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.rust_bootstrap_nix_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.configuration_nix_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            rust_src_flake_path,
            stage, // stage
            app_config.target.as_deref().unwrap_or_default(),
            app_config.rust_bootstrap_nix_flake_ref.as_deref().unwrap_or_default(),
            app_config.rust_src_flake_ref.as_deref().unwrap_or_default(),
            &resolved_build_rustc_path, // Use the resolved build rustc path
            cargo_path,
            app_config.rust_channel.as_deref().unwrap_or("stable"),
            app_config.rust_download_rustc.unwrap_or(false),
            app_config.rust_parallel_compiler.unwrap_or(false),
            app_config.rust_llvm_tools.unwrap_or(false),
            app_config.rust_debuginfo_level.unwrap_or(0),
            app_config.patch_binaries_for_nix.unwrap_or(false),
            app_config.vendor.unwrap_or(false),
            app_config.build_dir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.build_jobs.unwrap_or(0),
            app_config.home_dir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.cargo_home_dir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.install_prefix.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.install_sysconfdir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.dist_sign_folder.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.dist_upload_addr.as_deref().unwrap_or_default(),
            app_config.llvm_download_ci_llvm.unwrap_or(false),
            app_config.llvm_ninja.unwrap_or(false),
            app_config.change_id.as_deref().unwrap_or_default(),
        );
        fs::write(&config_toml_path, config_content)
            .context(format!("Failed to write config.toml to {:?}\n", config_toml_path))?;
        info!("Generated config.toml for rustc {} at {:?}\n", build_rustc_version, config_toml_path);

    } else {
        // Existing logic for single config generation
        info!("Starting config generation for stage {:?} and target {:?}\n", app_config.stage, app_config.target);

        // Run precondition checks
        info!("Running precondition checks...\n");
        preconditions::check_nix_command_available()?;
        info!("Nix command available.\n");

        // 1. Validate the project root
        info!("Validating project root: {:?}\n", app_config.project_root);
        let project_root = validate_project_root(app_config.project_root.as_ref().context("Project root is required")?)?;
        let flake_path_str = project_root.to_str()
            .context("Project root path contains non-UTF8 characters")?;
        info!("Project root validated: {}\n", flake_path_str);

        // 2. Use provided flake input paths
        let rust_src_flake_path_lossy = app_config.rust_src_flake_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        debug!("rust_src_flake_path: {:?}\n", rust_src_flake_path_lossy);

        preconditions::check_rust_toolchain_sysroot(
            &rust_src_flake_path_lossy,
        )?;
        info!("Rust toolchain sysroot check passed.\n");

        // 3. Construct the config.toml content
        info!("Constructing config.toml content...\n");
        let config_content = construct_config_content(
            app_config.system.as_deref().unwrap_or_default(),
            flake_path_str,
            app_config.nixpkgs_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.rust_overlay_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.rust_bootstrap_nix_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.configuration_nix_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.rust_src_flake_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.stage.as_deref().unwrap_or_default(),
            app_config.target.as_deref().unwrap_or_default(),
            app_config.rust_bootstrap_nix_flake_ref.as_deref().unwrap_or_default(),
            app_config.rust_src_flake_ref.as_deref().unwrap_or_default(),
            app_config.rustc_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.cargo_path.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.rust_channel.as_deref().unwrap_or("stable"),
            app_config.rust_download_rustc.unwrap_or(false),
            app_config.rust_parallel_compiler.unwrap_or(false),
            app_config.rust_llvm_tools.unwrap_or(false),
            app_config.rust_debuginfo_level.unwrap_or(0),
            app_config.patch_binaries_for_nix.unwrap_or(false),
            app_config.vendor.unwrap_or(false),
            app_config.build_dir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.build_jobs.unwrap_or(0),
            app_config.home_dir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.cargo_home_dir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.install_prefix.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.install_sysconfdir.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.dist_sign_folder.as_deref().map(|p| p.to_str().unwrap_or_default()).unwrap_or_default(),
            app_config.dist_upload_addr.as_deref().unwrap_or_default(),
            app_config.llvm_download_ci_llvm.unwrap_or(false),
            app_config.llvm_ninja.unwrap_or(false),
            app_config.change_id.as_deref().unwrap_or_default(),
        );
        debug!("Generated config content:\n{}\n", config_content);

        // 4. Handle output based on dry_run flag
        if app_config.dry_run.unwrap_or(false) {
            info!("Dry run enabled. Generated config will be printed to stdout.\n");
            println!("{}", config_content);
        } else {
            let output_path = app_config.output.unwrap_or_else(|| "config.toml".into());
            info!("Writing generated config to file: {:?}\n", output_path);
            fs::write(&output_path, config_content)
                .context(format!("Failed to write config to file: {:?}\n", output_path))?;
            info!("Config successfully written to {:?}\n", output_path);
        }
    }

    Ok(())
}
