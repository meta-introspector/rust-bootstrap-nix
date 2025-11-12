{
  description = "Nix flake for Rust package: flake-template-generator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile {
          rustToolchainFile = ./rust-toolchain.toml; # Assuming rust-toolchain.toml exists
        };
      in
      {
        packages.flake-template-generator = pkgs.rustPlatform.buildRustPackage {
          pname = "flake-template-generator";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock; # Assuming Cargo.lock exists

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            # Add common build dependencies here, e.g., openssl
            # openssl
          ];

          # Environment variables for build, if needed
          # OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
          # OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      }
    );
}
