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

    let config_file_path = env::var("CONFIG_OUTPUT_PATH")
        .unwrap_or_else(|_| "../../config.toml".to_string());

    fs::write(&config_file_path, config_content)
        .expect("Failed to write config.toml");

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
        .expect(&format!("Failed to execute nix command for {}", attr));

    let path = String::from_utf8_lossy(&output.stdout);
    path.trim().to_string()
}
