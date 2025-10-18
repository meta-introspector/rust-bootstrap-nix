{
  description = "Nix flake for the Rust bootstrap workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix/v0.12.0"; # Re-add cargo2nix input
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, cargo2nix, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            cargo2nix.overlays.default # Apply cargo2nix overlay here
          ];
        };

        rustVersion = "1.75.0"; # Explicitly set rust version
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          inherit rustVersion;
          packageFun = import ./Cargo.nix;
        };

        bootstrapApp = rustPkgs.workspace.bootstrap;
        buildHelperApp = rustPkgs.workspace.build_helper;
      in
      {
        packages = {
          bootstrap = bootstrapApp;
          build_helper = buildHelperApp;
          default = bootstrapApp; # Make bootstrap the default package
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-bin.stable.${rustVersion}.default
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
            pkgs.git
            pkgs.pkg-config
            pkgs.cmake
            pkgs.libiconv # For macOS
          ];
          CARGO_HOME = "${pkgs.writeText "cargo-home" ""}"; # Prevent cargo from writing to ~/.cargo
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });
}
