{
  description = "A pure Nix flake to build the Rust bootstrap compiler";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rust-src = {
      url = "github:meta-introspector/rust?ref=3487cd3843083db70ee30023f19344568ade9c9f";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, rust-src }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };

      rust_1_84_1_toolchain = pkgs.rust-bin.stable.latest.default;
      rust_1_84_1_rustc_path = "${rust_1_84_1_toolchain}/bin/rustc";
      rust_1_84_1_sysroot = pkgs.runCommand "get-sysroot-1-84-1" { } "${rust_1_84_1_rustc_path} --print sysroot > $out";
      rust_1_84_1_libdir = pkgs.runCommand "get-libdir-1-84-1" { } "echo ${rust_1_84_1_sysroot}/lib/rustlib/${pkgs.stdenv.hostPlatform.config}/lib > $out";

    in
    {
      packages.aarch64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "bootstrap";
        version = "0.1.0";

        src = rust-src; # Change src to the root of rust-src

        cargoLock.lockFile = "${rust-src}/src/bootstrap/Cargo.lock";
        cargoHash = "sha256-JO1pHLT+BxJrWnydzgu7VO0bR3dRaMlm0XFyL5FqxzI=";

        # The cargo build command needs to be run from the src/bootstrap directory
        # So we will add a preBuild phase to change directory
        preBuild = ''
          cd src/bootstrap
        '';

        checkPhase = ''
          # The bootstrap binary is in $out/bin/rustc
          # We need to set the environment variables that the shim expects
          export RUSTC_STAGE=0
          export RUSTC_SNAPSHOT="${rust_1_84_1_rustc_path}"
          export RUSTC_SYSROOT="${rust_1_84_1_sysroot}"
          export RUSTC_SNAPSHOT_LIBDIR="${rust_1_84_1_libdir}"
          export LD_LIBRARY_PATH="${rust_1_84_1_libdir}"

          # Run the rustc shim and check its version
          $out/bin/rustc --version | grep "rustc 1.84.1"
        '';
      };
    };
}
