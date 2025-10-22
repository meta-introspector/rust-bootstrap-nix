{
  description = "Nix flake for the configuration-nix Rust crate";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils"; # Corrected
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rustChannels.nightly.rust.override {
          targets = [
            (if system == "aarch64-linux" then "aarch64-unknown-linux-gnu" else "x86_64-unknown-linux-gnu")
          ];
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "configuration-nix";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildInputs = [ rustToolchain ];
        };

        apps.default = flake-utils.lib.mkApp {
          drv = pkgs.writeShellScriptBin "generate-config" ''
            ${self.packages.${system}.default}/bin/configuration-nix
          '';
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.nix
          ];
        };
      }
    );
}
