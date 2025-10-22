use std::process::Command;
use std::fs;
use std::env;

fn main() {
    let rustc_path = get_nix_path("rustc");
    let cargo_path = get_nix_path("cargo");

    let home_dir = env::var("HOME").unwrap_or_else(|_| "/tmp/nix-home".to_string());
    let cargo_home_dir = env::var("CARGO_HOME").unwrap_or_else(|_| format!("{}/.cargo", home_dir));

    let config_content = format!(
        "vendor = true\n\
         rustc = \"{}\"\n\
         cargo = \"{}\"\n\
         HOME = \"{}\"\n\
         CARGO_HOME = \"{}\"\n",
        rustc_path, cargo_path, home_dir, cargo_home_dir
    );

    let config_file_path = "config.toml".to_string(); // Write to a fixed filename

    fs::write(&config_file_file_path, config_content).unwrap();

    println!("Generated config.toml at {}", config_file_path);
}

fn get_nix_path(attr: &str) -> String {
    let expr = format!("(import ../get-paths.nix {{ system = \"aarch64-linux\"; }}).{}", attr);
    let output = Command::new("nix")
        .arg("eval")
        .arg("--impure")
        .arg("--raw")
        .arg("--expr")
        .arg(&expr)
        .output()
        .unwrap_or_else(|e| {
            eprintln!("Failed to execute nix command for {}: {}", attr, e);
            std::process::exit(1);
        });

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Nix command failed for {}: {}", attr, stderr);
        std::process::exit(1);
    }

    let path = String::from_utf8_lossy(&output.stdout);
    path.trim().to_string()
}
