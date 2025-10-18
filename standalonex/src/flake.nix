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
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        # Arguments for Cargo.nix
        cargoNixArgs = {
          rustPackages = rustToolchain;
          buildRustPackages = rustToolchain; # Assuming same for now
          hostPlatform = pkgs.stdenv.hostPlatform;
          lib = pkgs.lib;
          # These are usually provided by cargo2nix's internal functions
          # We need to find a way to get them or define them.
          # For now, let's try to pass dummy values or find them in pkgs.rustPlatform
          mkRustCrate = pkgs.rustPlatform.buildRustPackage; # This is a guess
          rustLib = pkgs.rustPlatform; # This is a guess
          workspaceSrc = ./.; # Current directory as workspace source
          ignoreLockHash = false; # Or true if we want to ignore
          cargoConfig = { };
          release = true; # Default value
          rootFeatures = [ "bootstrap/default" "build_helper/default" ]; # Default value
          hostPlatformCpu = null;
          hostPlatformFeatures = [ ];
          target = null;
          codegenOpts = null;
          profileOpts = null;
          cargoUnstableFlags = null;
          rustcLinkFlags = null;
          rustcBuildFlags = null;
        };

        cargoNix = import ./Cargo.nix cargoNixArgs;

        bootstrapApp = cargoNix.workspace.bootstrap;
        buildHelperApp = cargoNix.workspace.build_helper;
      in
      {
        packages = {
          bootstrap = bootstrapApp;
          build_helper = buildHelperApp;
          default = bootstrapApp; # Make bootstrap the default package
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
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
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
