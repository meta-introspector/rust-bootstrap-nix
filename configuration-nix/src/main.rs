use std::{env, fs, process::Command};

fn main() {
    let rustc_path = env::var("RUSTC_PATH").expect("RUSTC_PATH not set");
    let cargo_path = env::var("CARGO_PATH").expect("CARGO_PATH not set");
    let home_path = env::var("HOME_PATH").expect("HOME_PATH not set");
    let cargo_home_path = env::var("CARGO_HOME_PATH").expect("CARGO_HOME_PATH not set");

    let config_content = format!(
        r#"[rust]\nrustc = "{}"\ncargo = "{}"\n\n[build]\nrustc = "{}"\ncargo = "{}"\n\n[env]\nHOME = "{}"\nCARGO_HOME = "{}"\n"#,
        rustc_path, cargo_path, rustc_path, cargo_path, home_path, cargo_home_path
    );

    let config_file_path = "config.toml".to_string(); // Write to a fixed filename

    fs::write(&config_file_path, config_content).unwrap();
}
