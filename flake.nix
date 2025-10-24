{
  description = "Step 1: Generate config.toml";

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
        name = "generate-config";
        src = self;
        buildInputs = [ pkgs.cargo pkgs.rustc pkgs.cacert pkgs.nix pkgs.pkg-config pkgs.openssl ];
        buildPhase = ''
          export CARGO_HOME=$(mktemp -d)
          cargo run --bin bootstrap-config-generator -- --project-root . --rust-src-flake-path /nix/store/rhs81k02n3vg452abxl462g2i6xyadyf-source --version 1.84.1 --target aarch64-unknown-linux-gnu --stage 0
        '';
        installPhase = ''
          mkdir -p $out
          cp config.toml $out/config.toml
        '';
      };

      devShells.aarch64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          (pkgs.rust-bin.nightly.latest.default)
          pkg-config
          openssl
        ];
      };
    };
}
