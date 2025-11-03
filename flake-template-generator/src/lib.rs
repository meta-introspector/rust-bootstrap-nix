pub fn generate_flake_nix_content(
    nixpkgs_url: &str,
    system_arch: &str,
    _use_rustc_wrapper: bool,
) -> String {
    format!(
        r#"{{
  
  description = "Nix flake for Rust macro expansion";

  inputs = {{
    nixpkgs.url = "{}";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.12.0"; # Use a specific release of cargo2nix
  }};

  outputs = {{ self, nixpkgs, cargo2nix }}:
    let
      pkgs = import nixpkgs {{ system = "{}"; }};
      rustPkgs = pkgs.rust-bin.stable.latest.default.override {{
        targets = [ pkgs.system ];
      }};
      rustcWrapper = pkgs.writeShellScript "rustc-wrapper" ''
        echo "$(date): rustc called with arguments: $*" >> $out/rustc_calls.log
        exec ${{rustPkgs}}/bin/rustc "$@"
      '';
      expandedCode = pkgs.runCommand "expanded-code" {{
        nativeBuildInputs = [ rustPkgs ];
        src = ./.; # The temporary directory containing Cargo.toml and src/lib.rs
        RUSTC = rustcWrapper;
        outputs = [ "/expanded_code.rs" "/rustc_calls.log" ];
      }} ''
        export RUSTFLAGS="-Zunpretty=expanded"
        cargo rustc --lib -- -Zunpretty=expanded > $out/expanded_code.rs
      '';
    in
    {{
      packages.{}.default = expandedCode;
    }};
}}
"#,
        nixpkgs_url,
        system_arch,
        system_arch
    )
}
