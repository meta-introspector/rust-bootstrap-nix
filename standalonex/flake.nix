{
  description = "Standalone x.py environment";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    bootstrap-compiler = {
      url = "path:../flakes/bootstrap-from-json-flake";
    };
  };

  outputs = { self, nixpkgs, rustSrcFlake, rustOverlay, bootstrap-compiler }:
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
          rustc = "${pkgs.rust-bin.stable.latest.default}/bin/rustc"
          cargo = "${pkgs.rust-bin.stable.latest.default}/bin/cargo"
          EOF
                    export RUST_BOOTSTRAP_CONFIG=$(pwd)/config.toml

                    # Create dummy etc/ files for bootstrap compilation
                    mkdir -p etc
                    echo "{}" > etc/rust_analyzer_settings.json
                    echo ";; dummy eglot config" > etc/rust_analyzer_eglot.el
                    echo "# dummy helix config" > etc/rust_analyzer_helix.toml
        '';
      };

      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {
        pname = "xpy-build-output";
        version = "0.1.0";

        src = self; # Use the flake's own source as input

        nativeBuildInputs = [ pkgs.python3 pkgs.jq bootstrap-compiler.packages.aarch64-linux.default ];

        phases = [ "buildPhase" "installPhase" ];

        buildPhase = ''
            # Set environment variables
            export RUST_BOOTSTRAP_JSON_OUTPUT_DIR=$out

            # Create config.toml
            cat > config.toml <<EOF
          rustc = "${pkgs.rust-bin.stable.latest.default}/bin/rustc"
          cargo = "${pkgs.rust-bin.stable.latest.default}/bin/cargo"
          EOF
            export RUST_BOOTSTRAP_CONFIG=$(pwd)/config.toml

            # Run the bootstrap compiler
            ${bootstrap-compiler.packages.aarch64-linux.default}/bin/bootstrap dist

            # List the output
            ls -la $out
        '';

        # Remove installPhase as the file is copied in buildPhase
        installPhase = "";
      };
    };
}
