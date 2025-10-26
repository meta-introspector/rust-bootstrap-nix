

use std::path::PathBuf;
use std::process::Command;

pub fn run_statix_check(
    absolute_output_dir: &PathBuf,
    output_flake_nix_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running statix check on generated flake...");
    let statix_output = Command::new("nix-shell")
        .arg("-p").arg("statix")
        .arg("--run")
        .arg(format!("statix check {}", output_flake_nix_path.display()))
        .current_dir(absolute_output_dir) // Run statix from the generated flake directory
        .output()?;

    if !statix_output.status.success() {
        eprintln!("Statix check failed!");
        eprintln!("Stdout: {}", String::from_utf8_lossy(&statix_output.stdout));
        eprintln!("Stderr: {}", String::from_utf8_lossy(&statix_output.stderr));
        return Err("Statix check failed".into());
    }
    println!("Statix check passed.");
    Ok(())
}
