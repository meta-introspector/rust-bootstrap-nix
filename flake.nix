{
  description = "Python development environment extending rust-src with sccache";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b";

  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake }:
    let
      pkgs_aarch64 = import nixpkgs { system = "aarch64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain_aarch64 = pkgs_aarch64.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; };

      pkgs_x86_64 = import nixpkgs { system = "x86_64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain_x86_64 = pkgs_x86_64.rustChannels.nightly.rust.override { targets = [ "x86_64-unknown-linux-gnu" ]; };

      # Define the sccache-enabled rustc package
      # sccachedRustc = (system: pkgs: rustToolchain:
      #   let
      #     cargo_bin = "${rustToolchain}/bin/cargo";
      #     rustc_bin = "${rustToolchain}/bin/rustc";
      #     cargoHome = "$TMPDIR/.cargo";
      #     compiler_date = "2024-11-28";
      #     build_triple = if system == "aarch64-linux" then "aarch64-unknown-linux-gnu" else "x86_64-unknown-linux-gnu";
      #   in
      #   (rustSrcFlake.packages.${system}.default).overrideAttrs (oldAttrs: {
      #     nativeBuildInputs = (oldAttrs.nativeBuildInputs or [ ]) ++ [ pkgs.sccache pkgs.curl ];
      #     configurePhase = "# Skip the default configure script";
      #     preConfigure = pkgs.lib.concatStringsSep "\n" [
      #       (oldAttrs.preConfigure or "")
      #       "export RUSTC_WRAPPER=\"${pkgs.sccache}/bin/sccache\""
      #       "export SCCACHE_DIR=\"$TMPDIR/sccache\""
      #       "export SCCACHE_TEMPDIR=\"$TMPDIR/sccache-tmp\""
      #       "mkdir -p \"$SCCACHE_DIR\""
      #       "mkdir -p \"$SCCACHE_TEMPDIR\""
      #       "sccache --stop-server || true"
      #       "sccache --start-server"
      #       "export PATH=\"${pkgs.curl}/bin:$PATH\""
      #       "export CURL=\"${pkgs.curl}/bin/curl\""
      #     ];
      #     buildPhase = pkgs.lib.concatStringsSep "\n" [



      #       "mkdir -p \"$TMPDIR/.cargo\""
      #       "mkdir -p \"build/${build_triple}/stage0\""
      #       "echo \"${compiler_date}\" > \"build/${build_triple}/stage0/.rustc-stamp\""
      #       "export HOME=\"$TMPDIR\""
      #       "export CARGO_HOME=\"$TMPDIR/.cargo\""
      #       "python x.py build"
      #     ];
      #     preBuild = (oldAttrs.preBuild or "") + "sccache --zero-stats";
      #     postBuild = (oldAttrs.postBuild or "") + "sccache --show-stats\nsccache --stop-server";
      #   })
      # );

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
          pkgs_aarch64.which # Add which to the devShell
          pkgs_aarch64.statix # Add statix to the devShell
        ];

        # Set HOME and CARGO_HOME for the devShell
        shellHook = ''
          export HOME="$TMPDIR"
          export CARGO_HOME="$HOME/.cargo"
          mkdir -p $CARGO_HOME
        '';

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
          pkgs_x86_64.which # Add which to the devShell
          pkgs_x86_64.statix # Add statix to the devShell
        ];

        # Set HOME and CARGO_HOME for the devShell
        shellHook = ''
          export HOME="$TMPDIR"
          export CARGO_HOME="$HOME/.cargo"
          mkdir -p $CARGO_HOME
        '';

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

      # Define packages.default to be the sccache-enabled rustc package
      # packages.aarch64-linux.default = sccachedRustc "aarch64-linux" pkgs_aarch64 rustToolchain_aarch64;
      # packages.x86_64-linux.default = sccachedRustc "x86_64-linux" pkgs_x86_64 rustToolchain_x86_64;
    };
}
