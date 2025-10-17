{
  description = "Flake exposing x.py JSON output directory";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrc = {
      url = "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b";
      flake = false; # Mark as non-flake input
    };
    ourXpy = {
      url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=standalonex";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rustOverlay, rustSrc, ourXpy }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; overlays = [ rustOverlay.overlays.default ]; };

      # Derivation to generate the x.py JSON output
      xpyJsonOutputDerivation = pkgs.runCommandLocal "xpy-json-output"
        {
          nativeBuildInputs = [ pkgs.python3 pkgs.rust-bin.stable.latest.default ];
          src = rustSrc; # The rust source code
        } ''
                mkdir -p $out
        
                # Create config.toml with Nix-provided rustc and cargo paths
                cat > config.toml <<EOF
        rustc = "${pkgs.rust-bin.stable.latest.default}/bin/rustc"
        cargo = "${pkgs.rust-bin.stable.latest.default}/bin/cargo"
        EOF
                export RUST_BOOTSTRAP_CONFIG=$(pwd)/config.toml

                RUST_BOOTSTRAP_DRY_RUN_NIX_JSON=1 python3 ${ourXpy}/standalonex/x.py build --json-output $out
      '';
    in
    {
      packages.aarch64-linux.default = xpyJsonOutputDerivation; # Expose the output of the derivation
    };
}
