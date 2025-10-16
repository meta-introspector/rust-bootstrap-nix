{
  description = "Python development environment extending rust-src";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=3a285b7f77cacbe11c1b652d9990a9f5d2b64abc";
  };
  
  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake }:
    let
      pkgs_aarch64 = import nixpkgs { system = "aarch64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain_aarch64 = pkgs_aarch64.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; };

      pkgs_x86_64 = import nixpkgs { system = "x86_64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain_x86_64 = pkgs_x86_64.rustChannels.nightly.rust.override { targets = [ "x86_64-unknown-linux-gnu" ]; };
    in
    {
      devShells.aarch64-linux.default = pkgs_aarch64.mkShell {
        name = "python-rust-fix-dev-shell";

        packages = [
          rustToolchain_aarch64
          pkgs_aarch64.python3
          pkgs_aarch64.python3Packages.pip
          pkgs_aarch64.git
          pkgs_aarch64.curl
        ];

        nativeBuildInputs = [
          pkgs_aarch64.binutils
          pkgs_aarch64.cmake
          pkgs_aarch64.ninja
          pkgs_aarch64.pkg-config
          pkgs_aarch64.nix
        ];

        buildInputs = [
          pkgs_aarch64.openssl
          pkgs_aarch64.glibc.out
          pkgs_aarch64.glibc.static
        ];

        RUSTC_ICE = "0";
        LD_LIBRARY_PATH = "${pkgs_aarch64.lib.makeLibraryPath [
          pkgs_aarch64.stdenv.cc.cc.lib
        ]}";
      };

      devShells.x86_64-linux.default = pkgs_x86_64.mkShell {
        name = "python-rust-fix-dev-shell";

        packages = [
          rustToolchain_x86_64
          pkgs_x86_64.python3
          pkgs_x86_64.python3Packages.pip
          pkgs_x86_64.git
          pkgs_x86_64.curl
        ];

        nativeBuildInputs = [
          pkgs_x86_64.binutils
          pkgs_x86_64.cmake
          pkgs_x86_64.ninja
          pkgs_x86_64.pkg-config
          pkgs_x86_64.nix
        ];

        buildInputs = [
          pkgs_x86_64.openssl
          pkgs_x86_64.glibc.out
          pkgs_x86_64.glibc.static
        ];

        RUSTC_ICE = "0";
        LD_LIBRARY_PATH = "${pkgs_x86_64.lib.makeLibraryPath [
          pkgs_x86_64.stdenv.cc.cc.lib
        ]}";
      };

      # Define packages.default to be the rustSrcFlake's default package
      packages.aarch64-linux.default = rustSrcFlake.packages.aarch64-linux.default;
      packages.x86_64-linux.default = rustSrcFlake.packages.x86_64-linux.default;
    };
}
