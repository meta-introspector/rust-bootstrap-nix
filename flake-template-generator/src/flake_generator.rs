use std::fs;
use std::path::PathBuf;

pub fn generate_flake_nix_content(
    nixpkgs_url: &str,
    cargo2nix_url: &str, // New argument
    system_arch: &str,
    use_rustc_wrapper: bool,
    _rustc_wrapper_path: Option<&PathBuf>,
) -> String {
    let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("flake_template_rust_crate.nix");
    let mut flake_content = fs::read_to_string(&template_path)
        .expect("Failed to read flake_template_rust_crate.nix");

    let rustc_wrapper_definition = if use_rustc_wrapper {
        format!(
            r#"
      rustcWrapper = pkgs.writeShellScript "rustc-wrapper" ''
        echo "$(date): rustc called with arguments: $*" >> $out/rustc_calls.log
        exec "$pkgs.rust-bin.stable.latest.default/bin/rustc" "$@"
      '';
"#
        )
    } else {
        "".to_string()
    };

    let rustc_env_var = if use_rustc_wrapper {
        format!("RUSTC = rustcWrapper;")
    } else {
        "".to_string()
    };

    let rustc_calls_log_output = if use_rustc_wrapper {
        "\"/rustc_calls.log\""
    } else {
        ""
    };

    flake_content = flake_content.replace("NIXPKGS_URL_PLACEHOLDER", nixpkgs_url);
    flake_content = flake_content.replace("CARGO2NIX_URL_PLACEHOLDER", cargo2nix_url); // New placeholder replacement
    flake_content = flake_content.replace("SYSTEM_ARCH_PLACEHOLDER", system_arch);
    flake_content = flake_content.replace("RUSTC_WRAPPER_DEFINITION_PLACEHOLDER", &rustc_wrapper_definition);
    flake_content = flake_content.replace("RUSTC_ENV_VAR_PLACEHOLDER", &rustc_env_var);
    flake_content = flake_content.replace("RUSTC_CALLS_LOG_OUTPUT_PLACEHOLDER", rustc_calls_log_output);

    flake_content
}
