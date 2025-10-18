{
  description = "A minimal Rust project using cargo2nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, cargo2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            cargo2nix.overlays.default
          ];
        };

        rustVersion = "1.75.0"; # Explicitly set rust version
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          inherit rustVersion;
          packageFun = import ./Cargo.nix;
          workspaceSrc = ./.;
        };

        helloRustApp = rustPkgs.workspace.hello-rust { }; # Call the function to get the derivation
      in
      {
        packages = {
          hello-rust = helloRustApp;
          default = helloRustApp;
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
          CARGO_HOME = "${pkgs.writeText "cargo-home" ""}";
          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });
}
