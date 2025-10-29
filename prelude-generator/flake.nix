{
  description = "Nix flake for prelude-generator";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    hf-dataset-validator.url = "../vendor/rust/hugging-face-dataset-validator-rust";
  };

  outputs = { self, nixpkgs, rust-overlay, hf-dataset-validator, ... }@inputs:
    let
      system = "aarch64-linux"; # Assuming aarch64-linux as the primary system
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      lib = nixpkgs.lib;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc
          pkgs.pkg-config
          pkgs.openssl
          hf-dataset-validator.packages.${system}.default # Add hf-dataset-validator to PATH
        ];

        # Set environment variables for openssl-sys
        OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig"; # Ensure pkg-config finds openssl.pc
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "prelude-generator";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        buildInputs = [
          pkgs.pkg-config
          pkgs.openssl
          hf-dataset-validator.packages.${system}.default # Add hf-dataset-validator to PATH during build
        ];

        # Set environment variables for openssl-sys during build
        OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };
    };
}
