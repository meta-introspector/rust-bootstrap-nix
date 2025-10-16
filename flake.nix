{
  description = "Python development environment with local fixes for rust-src";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    # Reference the rust-src flake at the specified commit hash
    rustSrcFlake.url = "github:meta-introspector/rust?ref=c77d5981da5d3fce70e45b3ada424dd0fb8f4fd6";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, rustSrcFlake }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        # Get the rustToolchain from the referenced rustSrcFlake's devShell
        # This assumes rustSrcFlake.devShells.${system}.default exists and has rustToolchain
        # If not, we might need to define it here based on the rust-overlay
        rustToolchain = rustSrcFlake.devShells.${system}.default.rustToolchain or (pkgs.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; });
      in
      rec {
        devShells.default = pkgs.mkShell {
          name = "python-rust-fix-dev-shell";

          packages = [
            rustToolchain # From the referenced rustSrcFlake
            pkgs.python3
            pkgs.python3Packages.pip
            pkgs.python3Packages.venv
            # Add any other Python-specific packages here
          ];

          nativeBuildInputs = [
            pkgs.binutils
            pkgs.cmake
            pkgs.ninja
            pkgs.pkg-config
            pkgs.git
            pkgs.curl
            pkgs.cacert
            pkgs.patchelf
            pkgs.nix
          ];

          buildInputs = [
            pkgs.openssl
            pkgs.glibc.out
            pkgs.glibc.static
          ];

          RUSTC_ICE = "0";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
          ]}";
        };
      });
}
