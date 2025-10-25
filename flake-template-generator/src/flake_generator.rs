use crate::prelude::*


pub fn generate_flake_nix_content(
    nixpkgs_url: &str,
    system_arch: &str,
) -> String {
    format!(
        r#"{{
  description = "Dynamically generated config flake";

  inputs = {{
    nixpkgs.url = "{}";
  }};

  outputs = {{ self, nixpkgs }}:
    let
      pkgs = import nixpkgs {{ system = "{}"; }};
      configTomlContent = builtins.readFile ./config.toml;
    in
    {{
      packages.{}.default = pkgs.writeText "config.toml" configTomlContent;
    }};
}}
"#,
        nixpkgs_url,
        system_arch,
        system_arch
    )
}
