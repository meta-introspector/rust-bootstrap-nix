{
  description = "Standalone x.py environment";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=e6c1b92d0abaa3f64032d6662cbcde980c826ff2";
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

        nativeBuildInputs = [ pkgs.python3 ];

        phases = [ "buildPhase" "installPhase" ];

        buildPhase = ''
          echo "Current directory: $(pwd)"
          echo "TMPDIR: $TMPDIR"
          ls -la $TMPDIR

          # Create a writable temporary directory for x.py to work in
          local_build_dir=$TMPDIR/xpy_work
          echo "Creating local_build_dir: $local_build_dir"
          mkdir -p $local_build_dir
          cd $local_build_dir
          echo "Changed to local_build_dir: $(pwd)"
          ls -la .

          # Copy contents of the flake's source to the current writable directory
          echo "Copying $src/. to . (excluding config.toml)"
          cp -r $src/. .
          # Remove the read-only config.toml before copying
          rm config.toml
          ls -la .

          export RUST_SRC_STAGE0_PATH=${rustSrcFlake}/src/stage0

          # Copy config.old.toml and inject rustc/cargo paths
          echo "Copying config.old.toml to config.toml"
          cp config.old.toml config.toml
          sed -i "s|^#cargo = \".*\"|cargo = \"${pkgs.rust-bin.stable.latest.default}/bin/cargo\"|" config.toml
          sed -i "s|^#rustc = \".*\"|rustc = \"${pkgs.rust-bin.stable.latest.default}/bin/rustc\"|" config.toml

          export RUST_BOOTSTRAP_CONFIG=$(pwd)/config.toml

          # Set HOME and CARGO_HOME to writable temporary directories
          export HOME=$TMPDIR
          export CARGO_HOME=$TMPDIR/.cargo
          mkdir -p $CARGO_HOME

          echo "--- Running x.py build and capturing JSON output ---"
          # Temporarily disable 'exit on error' because x.py is expected to sys.exit(0)
          set +e
          python3 x.py build --json-output > $TMPDIR/xpy_json_output.json 2> $TMPDIR/xpy_stderr.log
          # Re-enable 'exit on error'
          set -e
          echo "--- x.py build finished. JSON output captured to $TMPDIR/xpy_json_output.json ---"

          # Read and parse the JSON output within Nix
          json_content=$(cat $TMPDIR/xpy_json_output.json)
          echo "JSON content read. Now parsing..."
          # In a real Nix expression, you would use builtins.fromJSON here.
          # For now, we just confirm it's read.
          cat $TMPDIR/xpy_json_output.json

        '';

        installPhase = ''
          mkdir -p $out
          mv $TMPDIR/xpy_json_output.json $out/xpy_json_output.json
          # Print the content of the captured output for debugging
          cat $out/xpy_json_output.json
        '';
      };
    };
}