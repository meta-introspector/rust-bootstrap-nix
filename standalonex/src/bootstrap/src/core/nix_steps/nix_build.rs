use crate::prelude::*


use std::path::{Path, PathBuf};
use std::process::Command;

//Box<dyn std::error.Error>
pub fn run_nix_build(flake_dir: &Path) -> Result<()> {
    println!("Running Nix build on generated flake...");
    let nix_build_output = Command::new("nix")
        .arg("build")
        .arg(".#default") // Use .#default when current_dir is the flake directory
        .current_dir(flake_dir) // Run nix build from the generated flake directory
        .output()?;

    if !nix_build_output.status.success() {
        eprintln!("Nix build failed!");
        eprintln!("Stdout: {}", String::from_utf8_lossy(&nix_build_output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&nix_build_output.stderr));
        return Err("Nix build failed".into());
    }
    println!("Nix build passed. Output in result link.");
    Ok(())
}
