{
  description = "A pure Nix flake to build the Rust bootstrap compiler";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rust-src = {
      url = "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, rust-src }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      packages.aarch64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "bootstrap";
        version = "0.1.0";

        src = rust-src; # Change src to the root of rust-src

        # cargoLock.lockFile = "${rust-src}/src/bootstrap/Cargo.lock";
        cargoHash = "";

        # The cargo build command needs to be run from the src/bootstrap directory
        # So we will add a preBuild phase to change directory
        preBuild = ''
          cd src/bootstrap
        '';
      };
    };
}
