{
  description = "Flake exposing x.py JSON output directory";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrc = {
      url = "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b";
      flake = false; # Mark as non-flake input
    };
  };

  outputs = { self, nixpkgs, rustSrc }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };

      # Derivation to generate the x.py JSON output
      xpyJsonOutputDerivation = pkgs.runCommandLocal "xpy-json-output"
        {
          nativeBuildInputs = [ pkgs.python3 ];
          src = rustSrc; # The rust source code
        } ''
        mkdir -p $out
        RUST_BOOTSTRAP_DRY_RUN_NIX_JSON=1 python3 $src/x.py build --json-output $out
      '';
    in
    {
      packages.aarch64-linux.default = xpyJsonOutputDerivation; # Expose the output of the derivation
    };
}
