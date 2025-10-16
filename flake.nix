{
  description = "Python development environment with local fixes for rust-src";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=c77d5981da5d3fce70e45b3ada424dd0fb8f4fd6";
  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake } :
    let
      system = "aarch64-linux"; # Hardcode for testing
      pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain = rustSrcFlake.devShells.${system}.default.rustToolchain or (pkgs.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; });
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "python-rust-fix-dev-shell";

        packages = [
          rustToolchain
          pkgs.python3
          pkgs.python3Packages.pip
          pkgs.python3Packages.venv
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
    };
}