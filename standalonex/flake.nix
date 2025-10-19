{
  description = "Standalone x.py environment";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=3487cd3843083db70ee30023f19344568ade9c9f";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rustSrcFlake, rustOverlay }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rustOverlay.overlays.default ];
      };
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

                    # Create config.toml with Nix-provided rustc and cargo paths
                    mkdir -p .cargo
                    cat > config.toml <<EOF
          rustc = "${pkgs.rust-bin.stable."1.84.1".default}/bin/rustc"
          cargo = "${pkgs.rust-bin.stable."1.84.1".default}/bin/cargo"
          EOF
                    export RUST_BOOTSTRAP_CONFIG=$(pwd)/config.toml

                    # Create dummy etc/ files for bootstrap compilation
                    mkdir -p etc
                    echo "{}" > etc/rust_analyzer_settings.json
                    echo ";; dummy eglot config" > etc/rust_analyzer_eglot.el
                    echo "# dummy helix config" > etc/rust_analyzer_helix.toml
        '';
      };

      packages.aarch64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "bootstrap";
        version = "0.1.0";

        src = ./src;
        cargoLock.lockFile = ./src/bootstrap/Cargo.lock;
        rustc = pkgs.rust-bin.stable."1.84.1".default;
        doCheck = false;
      };
    };
}
