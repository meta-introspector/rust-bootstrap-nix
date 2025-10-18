{
  description = "A flake for building the cc crate";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, cargo2nix }:
    let
      pkgs_aarch64 = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };
      pkgs_x86_64 = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };

      bootstrapSrc = ./../../../../standalonex/src/bootstrap;

      generatedRustPackages_aarch64 = cargo2nix.buildRustPackage {
        pkgs = pkgs_aarch64;
        src = bootstrapSrc;
      };

      generatedRustPackages_x86_64 = cargo2nix.buildRustPackage {
        pkgs = pkgs_x86_64;
        src = bootstrapSrc;
      };
    in
    {
      packages.aarch64-linux.default = generatedRustPackages_aarch64.bootstrap;
      packages.x86_64-linux.default = generatedRustPackages_x86_64.bootstrap;
    };

}
