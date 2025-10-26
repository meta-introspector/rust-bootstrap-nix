use crate::prelude::*;
pub mod prelude;
mod prelude;
mod args;
mod config_parser;
mod flake_generator;
mod file_writer;
mod statix_checker;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let absolute_output_dir = repo_root.join(&args.output_dir);
    fs::create_dir_all(&absolute_output_dir)?;
    let config = parse_config(&args.config_path)?;
    let nixpkgs_url = if config.nix.nixpkgs_path.is_empty() {
        "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify".to_string()
    } else {
        config.nix.nixpkgs_path
    };
    let system_arch = "aarch64-linux";
    let flake_nix_content = generate_flake_nix_content(&nixpkgs_url, &system_arch);
    let config_content = fs::read_to_string(&args.config_path)?;
    write_flake_and_config(&absolute_output_dir, &flake_nix_content, &config_content)?;
    let output_flake_nix_path = absolute_output_dir.join("flake.nix");
    run_statix_check(&absolute_output_dir, &output_flake_nix_path)?;
    Ok(())
}
