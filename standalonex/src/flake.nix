{
  description = "Nix flake for the Rust bootstrap workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix/v0.12.0";
  };

  outputs = { self, nixpkgs, flake-utils, cargo2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        cargoNixPkgs = import cargo2nix { inherit pkgs; };
        rustPkgs = pkgs.rust-bin.stable.latest.default;
        cargoNix = cargoNixPkgs.lib.${system}.importCargoLock {
          lockFile = ./Cargo.lock;
          cargoToml = ./Cargo.toml;
          inherit rustPkgs;
        };
      in
      {
        packages = {
          bootstrap = cargoNix.workspace.bootstrap;
          build_helper = cargoNix.workspace.build_helper;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustPkgs
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
          RUST_SRC_PATH = "${rustPkgs}/lib/rustlib/src/rust/library";
        };
      });
}
