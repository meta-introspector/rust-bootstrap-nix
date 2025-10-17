{
  description = "A flake for building the cc crate";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      packages.aarch64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "cc";
        version = "1.2.5";

        src = pkgs.fetchCrate {
          crateName = "cc";
          version = "1.2.5";
          sha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # Placeholder
        };

        cargoHash = ""; # Force hash mismatch to get the correct hash
      };
    };
}
