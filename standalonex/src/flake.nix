{
  description = "Nix flake for the Rust bootstrap workspace";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
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
          bootstrap = bootstrapApp { }; # Call the function to get the derivation
          build_helper = buildHelperApp { }; # Call the function to get the derivation
          default = bootstrapApp { }; # Call the function to get the derivation
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
