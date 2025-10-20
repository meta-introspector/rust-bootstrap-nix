{
  description = "A flake to use the built bootstrap binary";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    standalonex = {
      url = "path:../../standalonex";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rustOverlay, standalonex }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rustOverlay.overlays.default ];
      };

      bootstrap_path = standalonex.packages.aarch64-linux.default;
      rust_1_84_1_toolchain = pkgs.rust-bin.stable."1.84.1".default;
      rust_1_84_1_rustc_path = "${rust_1_84_1_toolchain}/bin/rustc";
      rust_1_84_1_sysroot = pkgs.runCommand "get-sysroot-1-84-1" { } "${rust_1_84_1_rustc_path} --print sysroot > $out";
      rust_1_84_1_libdir = pkgs.runCommand "get-libdir-1-84-1" { } "echo ${rust_1_84_1_sysroot}/lib/rustlib/${pkgs.stdenv.hostPlatform.config}/lib > $out";
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        name = "use-bootstrap-dev-shell";

        packages = [
          bootstrap_path # The built bootstrap binary
          rust_1_84_1_toolchain # The desired Rust toolchain
        ];

        shellHook = ''
          export PATH=${bootstrap_path}/bin:$PATH
          export RUSTC_STAGE=0 # Treat this as stage 0
          export RUSTC_SNAPSHOT=${rust_1_84_1_rustc_path}
          export RUSTC_SYSROOT=${rust_1_84_1_sysroot}
          export RUSTC_SNAPSHOT_LIBDIR=${rust_1_84_1_libdir}
          export LD_LIBRARY_PATH=${rust_1_84_1_libdir}
          # export RUST_BACKTRACE=full
          export LD_DEBUG=all
          echo "Bootstrap binary is available in your PATH."
        '';
      };

      rust_1_84_1_sysroot = rust_1_84_1_sysroot;
      rust_1_84_1_libdir = pkgs.runCommand "get-libdir-1-84-1" { } "echo ${rust_1_84_1_sysroot}/lib/rustlib/${pkgs.stdenv.hostPlatform.config}/lib > $out";

      bootstrap_path = bootstrap_path;
      rust_1_84_1_rustc_path = rust_1_84_1_rustc_path;
    };
}
