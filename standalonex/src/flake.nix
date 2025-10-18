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
        # Apply the cargo2nix overlay
        pkgsWithCargo2nix = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        rustPkgs = pkgsWithCargo2nix.rust-bin.stable.latest.default;
        cargoNix = pkgsWithCargo2nix.importCargoLock {
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

        devShells.default = pkgsWithCargo2nix.mkShell {
          buildInputs = [
            rustPkgs
            pkgsWithCargo2nix.cargo
            pkgsWithCargo2nix.rustc
            pkgsWithCargo2nix.rustfmt
            pkgsWithCargo2nix.clippy
            pkgsWithCargo2nix.git
            pkgsWithCargo2nix.pkg-config
            pkgsWithCargo2nix.cmake
            pkgsWithCargo2nix.libiconv # For macOS
          ];
          CARGO_HOME = "${pkgsWithCargo2nix.writeText "cargo-home" ""}"; # Prevent cargo from writing to ~/.cargo
          RUST_SRC_PATH = "${rustPkgs}/lib/rustlib/src/rust/library";
        };
      });
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
