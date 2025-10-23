{
  description = "Standalone x.py environment";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=3487cd3843083db70ee30023f19344568ade9c9f";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rustSrcFlake, rustOverlay, ... } @ args:
    let
      configTomlPath = args.configTomlPath;
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rustOverlay.overlays.default ];
      };
      rustPlatform = pkgs.rustPlatform;

    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        name = "standalonex-dev-shell";

        packages = [
          pkgs.python3
          pkgs.rust-bin.stable."1.84.1".default
          pkgs.cargo
        ];

        shellHook = ''
          # Add the flake's source directory to PATH
          export PATH=${self}/:$PATH # self here refers to the flake's source directory in the Nix store
          echo "x.py is available in your PATH."

          # Set environment variable for src/stage0 path
          export RUST_SRC_STAGE0_PATH=${rustSrcFlake}/src/stage0
          export RUST_SRC_ROOT=${rustSrcFlake}

          # In a Nix environment, it's generally preferred to manage config.toml statically
          # or pass tool paths via environment variables to the bootstrap process,
          # rather than dynamically generating config.toml in the shellHook.
          # For example, RUSTC and CARGO environment variables can be set directly.

          # Create dummy etc/ files for bootstrap compilation
          mkdir -p etc
          echo "{}" > etc/rust_analyzer_settings.json
          echo ";; dummy eglot config" > etc/rust_analyzer_eglot.el
          echo "# dummy helix config" > etc/rust_analyzer_helix.toml
        '';
      };

      packages.aarch64-linux = {
        default = rustPlatform.buildRustPackage {
          pname = "rust-bootstrap-default";
          version = "0.1.0";
          src = pkgs.lib.cleanSource ./src;
          cargoLock.lockFile = ./src/Cargo.lock;
          rustc = pkgs.rust-bin.stable."1.84.1".default;
          doCheck = false;
          postPatch = ''
            cp ${configTomlPath} config.toml
          '';

          bootstrap-main = rustPlatform.buildRustPackage {
            pname = "bootstrap-main";
            version = "0.1.0";

            src = pkgs.lib.cleanSource ./src;
            cargoLock.lockFile = ./src/Cargo.lock;
            rustc = pkgs.rust-bin.stable."1.84.1".default;
            doCheck = false;
            cargoBuildFlags = [ "--bin" "bootstrap" ];
            postPatch = ''
              cp ${configTomlPath} config.toml
            '';
          };

          nix-bootstrap = rustPlatform.buildRustPackage {
            pname = "nix-bootstrap";
            version = "0.1.0";

            src = pkgs.lib.cleanSource ./src;
            cargoLock.lockFile = ./src/Cargo.lock;
            rustc = pkgs.rust-bin.stable."1.84.1".default;
            doCheck = false;
            cargoBuildFlags = [ "--bin" "nix_bootstrap" ];
            postPatch = ''
              cp ${configTomlPath} config.toml
            '';
          };
        };
      };
    };
}

