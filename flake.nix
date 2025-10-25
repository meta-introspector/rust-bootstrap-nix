{
  description = "Step 1: Generate config.toml";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake, lib, ... }@inputs:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      commonRustDeps = import ./nix/rust-deps/common-rust-deps.nix { inherit pkgs lib; };
    in
    {
      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {
        name = "generate-config";
        src = self;
        buildInputs = [ pkgs.cargo pkgs.rustc pkgs.cacert pkgs.nix ] ++ commonRustDeps.commonBuildInputs;
        buildPhase = ''
          export CARGO_HOME=$(mktemp -d)
          cargo run --bin bootstrap-config-generator -- --project-root . --rust-src-flake-path /nix/store/rhs81k02n3vg452abxl462g2i6xyadyf-source --version 1.84.1 --target aarch64-unknown-linux-gnu --stage 0
        '';
        installPhase = ''
          mkdir -p $out
          cp config.toml $out/config.toml
        '';
      };

      packages.aarch64-linux.check = pkgs.rustPlatform.buildRustPackage {
        pname = "rust-check";
        version = "0.1.0";
        src = self;
        cargoLock.lockFile = ./Cargo.lock;
        buildInputs = commonRustDeps.commonBuildInputs;
        checkPhase = ''
          echo "pkgs.openssl.dev path: ${pkgs.openssl.dev}"
          export PKG_CONFIG_PATH=${commonRustDeps.pkgConfigPath}
          export PKG_CONFIG_SYSROOT_DIR=/
          export PATH=$PATH:${pkgs.pkg-config}/bin
          cargo check --workspace
        '';
      };

      devShells.aarch64-linux.default = pkgs.mkShell {
        packages = with pkgs; [
          (pkgs.rust-bin.nightly.latest.default)
        ] ++ commonRustDeps.commonBuildInputs;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath;
        shellHook = ''
          export PATH=$PATH:${pkgs.rust-bin.nightly.latest.default}/bin
        '';
      };
    };
}
