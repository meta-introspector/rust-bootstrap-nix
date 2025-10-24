{
  description = "A minimal flake for bootstrapping Rust";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake, ... }@inputs:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {
        name = "rust-bootstrap";
        src = ./.;
        buildInputs = [ pkgs.cargo pkgs.rustc pkgs.cacert pkgs.nix ];
        buildPhase = ''
          export CARGO_HOME=$(mktemp -d)
          cargo run --bin bootstrap-config-generator -- --project-root . --rust-src-flake-path ${rustSrcFlake}
        '';
        installPhase = ''
          mkdir -p $out/bin
        '';
      };

      devShells.aarch64-linux.default = pkgs.mkShell {
        packages = [
          pkgs.rust-bin.stable."1.84.1".default
          pkgs.cargo
        ];
      };
    };
}
