{
  description = "Python development environment extending rust-src with sccache";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=feature/CRQ-016-nixify";
    configuration-nix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=configuration-nix";
    standalonex.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=standalonex";
  };

  outputs = { self, nixpkgs, rust-overlay, rustSrcFlake, flake-utils, configuration-nix, standalonex }:
    let
      lib = nixpkgs.lib;
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

      # Define packages.default to be the sccache-enabled rustc package
      # packages.aarch64-linux.default = sccachedRustc "aarch64-linux" pkgs_aarch64 rustToolchain_aarch64;
      # packages.x86_64-linux.default = sccachedRustc "x86_64-linux" pkgs_x86_64 rustToolchain_x86_64;

      # Import the config-extractor
      configExtractor = import (self + "/examples/config-extractor.nix") {
        inherit lib;
        pkgs = pkgs_aarch64;
      };

      # Example usage: Extract config from standalonex/config.toml
      parsedConfig = configExtractor.extractConfig {
        configFilePath = self + "/standalonex/config.toml";
        extraConfig = {
          build = {
            patch-binaries-for-nix = false;
          };
        };
      };

      # Helper function to generate config.toml for a given stage
      generateConfigTomlForStage = { system, pkgs, rustToolchain, configurationNix, stageNum }:
        pkgs.runCommand "config-stage-${toString stageNum}.toml"
          {
            nativeBuildInputs = [ configurationNix.packages.${system}.default pkgs.nix ];
            RUSTC_PATH = "${rustToolchain}/bin/rustc";
            CARGO_PATH = "${rustToolchain}/bin/cargo";
            HOME_PATH = "$TMPDIR/home"; # Use a temporary home directory
            CARGO_HOME_PATH = "$TMPDIR/cargo-home"; # Use a temporary cargo home directory
          } ''
          mkdir -p $(dirname $out)
          mkdir -p $HOME_PATH
          mkdir -p $CARGO_HOME_PATH
          ${configurationNix.packages.${system}.default}/bin/configuration-nix
          mv config.toml $out
        '';

      # Generate config.toml for multiple stages
      configTomlStages_aarch64 = lib.mapAttrs' (stageNum: config: { name = "configStage${stageNum}"; value = config; }) (lib.genAttrs (map toString (lib.range 0 2)) (stageNum:
        generateConfigTomlForStage {
          system = "aarch64-linux";
          pkgs = pkgs_aarch64;
          rustToolchain = rustToolchain_aarch64; # Use the same toolchain for now
          configurationNix = configuration-nix;
          stageNum = stageNum;
        }
      ));

      # Generate config.toml for multiple stages
      configTomlStages_x86_64 = lib.mapAttrs' (stageNum: config: { name = "configStage${stageNum}"; value = config; }) (lib.genAttrs (map toString (lib.range 0 2)) (stageNum:
        generateConfigTomlForStage {
          system = "x86_64-linux";
          pkgs = pkgs_x86_64;
          rustToolchain = rustToolchain_x86_64; # Use the same toolchain for now
          configurationNix = configuration-nix;
          stageNum = stageNum;
        }
      ));
    in
    {
      packages.aarch64-linux = configTomlStages_aarch64 // {
        bootstrapConfigBuilder = pkgs_aarch64.stdenv.mkDerivation {
          pname = "rust-bootstrap-config-builder";
          version = "0.1.0";

          # No source needed, as we are just arranging existing outputs
          src = null;

          # Depend on the configTomlStages derivations
          configStage0 = configTomlStages_aarch64.configStage0;
          configStage1 = configTomlStages_aarch64.configStage1;
          configStage2 = configTomlStages_aarch64.configStage2;

          installPhase = ''
            mkdir -p $out/standalonex/src/bootstrap/stage0
            mkdir -p $out/standalonex/src/bootstrap/stage1
            mkdir -p $out/standalonex/src/bootstrap/stage2
            
            ln -s $configStage0 $out/standalonex/src/bootstrap/stage0/config.toml
            ln -s $configStage1 $out/standalonex/src/bootstrap/stage1/config.toml
            ln -s $configStage2 $out/standalonex/src/bootstrap/stage2/config.toml
          '';
        };
        default = self.inputs.standalonex.packages.${pkgs_aarch64.system}.default;
      };

      packages.x86_64-linux = configTomlStages_x86_64 // {
        bootstrapConfigBuilder = pkgs_x86_64.stdenv.mkDerivation {
          pname = "rust-bootstrap-config-builder";
          version = "0.1.0";

          # No source needed, as we are just arranging existing outputs
          src = null;

          # Depend on the configTomlStages derivations
          configStage0 = configTomlStages_x86_64.configStage0;
          configStage1 = configTomlStages_x86_64.configStage1;
          configStage2 = configTomlStages_x86_64.configStage2;

          installPhase = ''
            mkdir -p $out/standalonex/src/bootstrap/stage0
            mkdir -p $out/standalonex/src/bootstrap/stage1
            mkdir -p $out/standalonex/src/bootstrap/stage2
            
            ln -s $configStage0 $out/standalonex/src/bootstrap/stage0/config.toml
            ln -s $configStage1 $out/standalonex/src/bootstrap/stage1/config.toml
            ln -s $configStage2 $out/standalonex/src/bootstrap/stage2/config.toml
          '';
        };
        default = self.inputs.standalonex.packages.${pkgs_x86_64.system}.default;
      };

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
          pkgs_aarch64.rust-analyzer # Add rust-analyzer to the devShell
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
          pkgs_x86_64.rust-analyzer # Add rust-analyzer to the devShell
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

      apps.aarch64-linux.generateConfig = configuration-nix.apps.aarch64-linux.default;

      apps.x86_64-linux.generateConfig = configuration-nix.apps.x86_64-linux.default;
    };
}
