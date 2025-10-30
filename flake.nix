{
  description = "Step 1: Generate config.toml";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake, cargo2nix, ... }@inputs:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default cargo2nix.overlays.default ];
      };
      lib = nixpkgs.lib; # Define lib here
      commonRustDeps = import ./nix/rust-deps/common-rust-deps.nix { inherit pkgs lib; };
    in
    {
      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {
        name = "generate-config";
        src = self;
        buildInputs = [ pkgs.cargo pkgs.rustc pkgs.cacert pkgs.nix pkgs.pkg-config pkgs.openssl ] ++ commonRustDeps.commonBuildInputs;
        OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
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
        buildInputs = [ pkgs.pkg-config pkgs.openssl ] ++ commonRustDeps.commonBuildInputs;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath;
        OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        CARGO_NET_OFFLINE = false;
        cargoBuildFlags = [ "--target" "aarch64-unknown-linux-gnu" ];
        CARGO_BUILD_FLAGS = "--target aarch64-unknown-linux-gnu";
        checkPhase = ''
          echo "pkgs.openssl.dev path: ${pkgs.openssl.dev}"
          export PKG_CONFIG_PATH=${commonRustDeps.pkgConfigPath}
          export PKG_CONFIG_SYSROOT_DIR=/
          export PATH=$PATH:${pkgs.pkg-config}/bin
          cargo check --workspace
        '';
      };

      packages.aarch64-linux.bootstrap = pkgs.rustPlatform.buildRustPackage {
        pname = "bootstrap";
        version = "0.0.0";
        src = self;
        cargoLock.lockFile = ./standalonex/src/bootstrap/Cargo.lock;
        cargoRoot = "standalonex/src/bootstrap";
        buildInputs = [ pkgs.pkg-config pkgs.openssl ] ++ commonRustDeps.commonBuildInputs;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath;
        OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        CARGO_NET_OFFLINE = false;
        cargoBuildFlags = [ "--target" "aarch64-unknown-linux-gnu" ];
        CARGO_BUILD_FLAGS = "--target aarch64-unknown-linux-gnu";
      };

      packages.aarch64-linux.generateCargoNix = pkgs.callPackage
        (inputs.cargo2nix.lib.importCargoLock {
          lockFile = self.outPath + "/Cargo.lock"; # Use the workspace root's Cargo.lock
        })
        { };

      devShells.aarch64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.rust-analyzer
          pkgs.openssl
          pkgs.pkg-config
          packages = with pkgs; [
          rust-bin.nightly.latest.default
          cargo-watch
          cargo-expand
          prettyplease
          rustfmt
        ];
          PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath;
          OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
          OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          RUST_SRC_PATH = "${rustSrcFlake}/library";
          };
          };
          }
