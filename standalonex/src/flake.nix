{
  description = "Nix flake for the Rust bootstrap workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustPkgs = pkgs.rust-bin.stable.latest.default;
        cargoNix = import ./Cargo.nix {
          inherit pkgs rustPkgs;
          rustPackages = rustPkgs;
          buildRustPackages = rustPkgs;
          lib = pkgs.lib;
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
