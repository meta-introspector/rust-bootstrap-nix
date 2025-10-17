{
  description = "Flake exposing x.py JSON output directory";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrc = {
      url = "github:meta-introspector/rust?ref=e6c1b92d0abaa3f64032d6662cbcde980c826ff2";
      flake = false; # Mark as non-flake input
    };
  };

  outputs = { self, nixpkgs, rustSrc }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };

      # Derivation to generate the x.py JSON output
      xpyJsonOutputDerivation = pkgs.runCommandLocal "xpy-json-output" {
        nativeBuildInputs = [ pkgs.python3 ];
        src = rustSrc; # The rust source code
      } ''
        mkdir -p $out
        python3 $src/x.py build --json-output $out
      '';
    in
    {
      packages.aarch64-linux.default = xpyJsonOutputDerivation; # Expose the output of the derivation
    };
}