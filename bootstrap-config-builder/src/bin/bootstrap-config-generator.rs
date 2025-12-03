//use crate::prelude::*;
use bootstrap_config_builder::prelude::*;
use bootstrap_config_builder::config::{AppConfig, load_canonical_config}; // Explicitly use AppConfig
use standalonex::bootstrap::core::config_standalone::Config as CanonicalConfig;
fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    debug!("Raw CLI Arguments: {:?}\n", args);
    let canonical_config: CanonicalConfig = load_canonical_config(&args)?;
    
    info!("Final merged configuration: {:?}\n", canonical_config);
    
        if let Some(build_rustc_version) = canonical_config.download_rustc_commit.clone() {
            info!(
                "Lattice generation mode enabled for rustc version: {}\n",
                build_rustc_version
            );
            let solana_rustc_path = canonical_config
                .initial_rustc // Mapped from app_config.solana_rustc_path
                .to_str()
                .context("solana_rustc_path contains non-UTF8 characters")?;
            let cargo_path = canonical_config
                .initial_cargo // Mapped from app_config.cargo_path
                .to_str()
                .context("cargo_path contains non-UTF8 characters")?;
            let project_root = canonical_config
                .src // Mapped from app_config.project_root
                .to_str()
                .context("project_root contains non-UTF8 characters")?;
            let rust_src_flake_path = canonical_config
                .rust_src_flake_path
                .as_ref()
                .context("rust_src_flake_path is required for lattice generation")?
                .to_str()
                .context("rust_src_flake_path contains non-UTF8 characters")?;
            let architecture = canonical_config.build.to_string(); // Mapped from app_config.architecture
            let stage = canonical_config.stage.to_string(); // Mapped from app_config.stage
            let step = "step1-configure".to_string(); // CanonicalConfig does not have a direct step field, hardcode for now
            
            let resolved_build_rustc_path =
                find_nix_package_store_path("rustc", Some(&build_rustc_version))?.context(format!(
                    "Could not find rustc path for version {}",
                    build_rustc_version
                ))?;
            let output_dir = PathBuf::from(format!(
                "flakes/{}/{}/{}/{}",
                build_rustc_version, architecture, stage, step
            ));
            fs::create_dir_all(&output_dir).context(format!(
                "Failed to create output directory: {:?}",
                output_dir
            ))?;
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
            fs::write(&flake_nix_path, flake_nix_content).context(format!(
                "Failed to write flake.nix to {:?}\n",
                flake_nix_path
            ))?;
            info!(
                "Generated flake.nix for rustc {} at {:?}\n",
                build_rustc_version, flake_nix_path
            );
            let config_toml_path =
                output_dir.join(format!("generated_config_{}.toml", build_rustc_version));
            let config_content = construct_config_content(
                &architecture,
                project_root,
                canonical_config
                    .nixpkgs_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .rust_overlay_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .rust_bootstrap_nix_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .configuration_nix_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                rust_src_flake_path,
                &stage,
                canonical_config.targets.get(0).map_or("".to_string(), |t| t.to_string()), // AppConfig.target is Option<String>, CanonicalConfig.targets is Vec<TargetSelection>
                canonical_config
                    .rust_info
                    .flake_ref
                    .as_str(), // Mapped from app_config.rust_bootstrap_nix_flake_ref
                canonical_config
                    .cargo_info
                    .flake_ref
                    .as_str(), // Mapped from app_config.rust_src_flake_ref
                &resolved_build_rustc_path,
                cargo_path,
                canonical_config.channel.as_str(), // Mapped from app_config.rust_channel
                canonical_config.rust_download_rustc, // Mapped from app_config.rust_download_rustc
                canonical_config.rust_parallel_compiler, // Mapped from app_config.rust_parallel_compiler
                canonical_config.llvm_tools_enabled, // Mapped from app_config.rust_llvm_tools
                canonical_config.rust_debuginfo_level_rustc.to_u8(), // Mapped from app_config.rust_debuginfo_level
                canonical_config.patch_binaries_for_nix.unwrap_or(false),
                canonical_config.vendor, // Mapped from app_config.vendor
                canonical_config
                    .out // Mapped from app_config.build_dir
                    .to_str()
                    .unwrap_or_default(),
                canonical_config.jobs.unwrap_or(0),
                canonical_config
                    .musl_root // Mapped from app_config.home_dir
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .cargo_info.git_dir // Assuming this is the correct mapping for cargo_home_dir
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .prefix // Mapped from app_config.install_prefix
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .sysconfdir // Mapped from app_config.install_sysconfdir
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .dist_sign_folder
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config.dist_upload_addr.as_deref().unwrap_or_default(),
                canonical_config.llvm_from_ci, // Mapped from app_config.llvm_download_ci_llvm
                canonical_config.ninja_in_file, // Mapped from app_config.llvm_ninja
                canonical_config.change_id.map_or("".to_string(), |id| id.to_string()), // Mapped from app_config.change_id
            );
            fs::write(&config_toml_path, config_content).context(format!(
                "Failed to write config.toml to {:?}\n",
                config_toml_path
            ))?;
            info!(
                "Generated config.toml for rustc {} at {:?}\n",
                build_rustc_version, config_toml_path
            );
        } else {
            info!(
                "Starting config generation for stage {:?} and target {:?}\n",
                canonical_config.stage, canonical_config.targets
            );
            info!("Running precondition checks...\n");
            preconditions::check_nix_command_available()?;
            info!("Nix command available.\n");
            info!("Validating project root: {:?}\n", canonical_config.src);
            let project_root = validate_project_root(
                &canonical_config.src
            )?;
            let flake_path_str = project_root
                .to_str()
                .context("Project root path contains non-UTF8 characters")?;
            info!("Project root validated: {}\n", flake_path_str);
            let rust_src_flake_path_lossy = canonical_config
                .rust_src_flake_path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            debug!("rust_src_flake_path: {:?}\n", rust_src_flake_path_lossy);
            preconditions::check_rust_toolchain_sysroot(&rust_src_flake_path_lossy)?;
            info!("Rust toolchain sysroot check passed.\n");
            info!("Constructing config.toml content...\n");
            let config_content = construct_config_content(
                &canonical_config.build.to_string(), // AppConfig.system is String, CanonicalConfig.build is TargetSelection
                flake_path_str,
                canonical_config
                    .nixpkgs_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .rust_overlay_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .rust_bootstrap_nix_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .configuration_nix_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .rust_src_flake_path
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                &canonical_config.stage.to_string(), // AppConfig.stage is Option<String>, CanonicalConfig.stage is u32
                canonical_config.targets.get(0).map_or("".to_string(), |t| t.to_string()), // AppConfig.target is Option<String>, CanonicalConfig.targets is Vec<TargetSelection>
                canonical_config
                    .rust_info
                    .flake_ref
                    .as_str(), // Mapped from app_config.rust_bootstrap_nix_flake_ref
                canonical_config
                    .cargo_info
                    .flake_ref
                    .as_str(), // Mapped from app_config.rust_src_flake_ref
                canonical_config
                    .initial_rustc // Mapped from app_config.rustc_path
                    .to_str()
                    .unwrap_or_default(),
                canonical_config
                    .initial_cargo // Mapped from app_config.cargo_path
                    .to_str()
                    .unwrap_or_default(),
                canonical_config.channel.as_str(), // Mapped from app_config.rust_channel
                canonical_config.rust_download_rustc, // Mapped from app_config.rust_download_rustc
                canonical_config.rust_parallel_compiler, // Mapped from app_config.rust_parallel_compiler
                canonical_config.llvm_tools_enabled, // Mapped from app_config.rust_llvm_tools
                canonical_config.rust_debuginfo_level_rustc.to_u8(), // Mapped from app_config.rust_debuginfo_level
                canonical_config.patch_binaries_for_nix.unwrap_or(false),
                canonical_config.vendor, // Mapped from app_config.vendor
                canonical_config
                    .out // Mapped from app_config.build_dir
                    .to_str()
                    .unwrap_or_default(),
                canonical_config.jobs.unwrap_or(0),
                canonical_config
                    .musl_root // Mapped from app_config.home_dir
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .cargo_info.git_dir // Assuming this is the correct mapping for cargo_home_dir
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .prefix // Mapped from app_config.install_prefix
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .sysconfdir // Mapped from app_config.install_sysconfdir
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config
                    .dist_sign_folder
                    .as_deref()
                    .map(|p| p.to_str().unwrap_or_default())
                    .unwrap_or_default(),
                canonical_config.dist_upload_addr.as_deref().unwrap_or_default(),
                canonical_config.llvm_from_ci, // Mapped from app_config.llvm_download_ci_llvm
                canonical_config.ninja_in_file, // Mapped from app_config.llvm_ninja
                canonical_config.change_id.map_or("".to_string(), |id| id.to_string()), // Mapped from app_config.change_id
            );
            debug!("Generated config content:\n{}\n", config_content);
            if canonical_config.dry_run.is_enabled() { // Use is_enabled() on DryRun enum
                info!("Dry run enabled. Generated config will be printed to stdout.\n");
                println!("{}", config_content);
            } else {
                let output_path = canonical_config.output.unwrap_or_else(|| "config.toml".into()); // AppConfig.output is PathBuf, CanonicalConfig.output (if it exists) is Option<PathBuf>
                info!("Writing generated config to file: {:?}\n", output_path);
                fs::write(&output_path, config_content).context(format!(
                    "Failed to write config to file: {:?}\n",
                    output_path
                ))?;
                info!("Config successfully written to {:?}\n", output_path);
            }
        }
        Ok(())
    }
