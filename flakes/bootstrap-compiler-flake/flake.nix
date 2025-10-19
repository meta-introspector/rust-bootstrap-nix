{
  description = "A flake for building the bootstrap compiler";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rust-bootstrap-nix = {
      url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, rust-bootstrap-nix }:
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

        src = "${rust-bootstrap-nix}/standalonex/src";

        cargoLock.lockFile = "${rust-bootstrap-nix}/standalonex/src/bootstrap/Cargo.lock";
      };
    };
}
