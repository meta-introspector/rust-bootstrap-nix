{
  description = "Python development environment extending rust-src with sccache";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=e6c1b92d0abaa3f64032d6662cbcde980c826ff2";
#    configToml.url = "path:./config.toml";
  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake,
  #configToml
  } :
    let
      pkgs_aarch64 = import nixpkgs { system = "aarch64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain_aarch64 = pkgs_aarch64.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; };

      pkgs_x86_64 = import nixpkgs { system = "x86_64-linux"; overlays = [ rust-overlay.overlays.default ]; };
      rustToolchain_x86_64 = pkgs_x86_64.rustChannels.nightly.rust.override { targets = [ "x86_64-unknown-linux-gnu" ]; };

      # Define the sccache-enabled rustc package
      sccachedRustc = (system: pkgs: rustToolchain:
        let
          cargo_bin = "${rustToolchain}/bin/cargo";
          rustc_bin = "${rustToolchain}/bin/rustc";
          cargoHome = "$TMPDIR/.cargo";
        in
        (rustSrcFlake.packages.${system}.default).overrideAttrs (oldAttrs: {
          nativeBuildInputs = (oldAttrs.nativeBuildInputs or []) ++ [ pkgs.sccache pkgs.curl ];
          configurePhase = ''
            # Skip the default configure script
          '';
          preConfigure = (oldAttrs.preConfigure or "") + ''
            export HOME="$TMPDIR"
            export CARGO_HOME="${cargoHome}"
            mkdir -p $CARGO_HOME
            export RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"
            export SCCACHE_DIR="$TMPDIR/sccache"
            export SCCACHE_TEMPDIR="$TMPDIR/sccache-tmp"
            mkdir -p "$SCCACHE_DIR"
            mkdir -p "$SCCACHE_TEMPDIR"
            sccache --stop-server || true
            sccache --start-server
            export PATH="${pkgs.curl}/bin:$PATH"
            export CURL="${pkgs.curl}/bin/curl"
          '';
          buildPhase = ''
            echo "vendor = true" >> config.toml
            echo "rustc = \"${rustc_bin}\"" >> config.toml
            echo "cargo = \"${cargo_bin}\"" >> config.toml
            echo "--- config.toml content ---"
            cat ./config.toml
            echo "--- file listing ---"
            ls -l
            python x.py build
          '';
          preBuild = (oldAttrs.preBuild or "") + ''
            sccache --zero-stats
          '';
          postBuild = (oldAttrs.postBuild or "") + ''
            sccache --show-stats
            sccache --stop-server
          '';
        })
      );

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

      # Define packages.default to be the sccache-enabled rustc package
      packages.aarch64-linux.default = sccachedRustc "aarch64-linux" pkgs_aarch64 rustToolchain_aarch64;
      packages.x86_64-linux.default = sccachedRustc "x86_64-linux" pkgs_x86_64 rustToolchain_x86_64;
    };
}
