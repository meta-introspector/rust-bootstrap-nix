


pub fn generate_step1_flake(
    nixpkgs_ref: &str,
    rust_overlay_ref: &str,
    rust_src_flake_ref: &str,
    rust_bootstrap_nix_ref: &str,
    rust_src_flake_path: &str,
) -> String {
    format!(
        r#"{{
  description = "Step 1: Generate config.toml";

  inputs = {{
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref={}";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref={}";
    rustSrcFlake.url = "github:meta-introspector/rust?ref={}";
    rust-bootstrap-nix.url = "github:meta-introspector/rust-bootstrap-nix?ref={}";
  }};

  outputs = {{ self, nixpkgs, rust-overlay, rustSrcFlake, rust-bootstrap-nix, ... }}@inputs:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs {{
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      }};
    in
    {{
      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {{
        name = "generate-config";
        src = rust-bootstrap-nix;
        buildInputs = [ pkgs.cargo pkgs.rustc pkgs.cacert pkgs.nix ];
        buildPhase = '''
          export CARGO_HOME=$(mktemp -d)
          cargo run --bin bootstrap-config-generator -- --project-root . --rust-src-flake-path {} --version 1.84.1 --target aarch64-unknown-linux-gnu --stage 0
        ''';
        installPhase = '''
          mkdir -p $out
          cp config.toml $out/config.toml
        ''';
      }};
    }};
}}
"#,
        nixpkgs_ref,
        rust_overlay_ref,
        rust_src_flake_ref,
        rust_bootstrap_nix_ref,
        rust_src_flake_path
    )
}
