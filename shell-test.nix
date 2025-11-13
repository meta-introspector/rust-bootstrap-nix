{ pkgs ? import <nixpkgs> { } }:

pkgs.stdenv.mkDerivation {
  pname = "shell-test";
  version = "0.1.0";

  # Use the devShell from the current flake as a build input
  buildInputs = [ (import ./. { }).devShells.${pkgs.system}.default ];

  # The actual command to run in the build phase
  buildPhase = ''
    echo "Running rustc --version inside the devShell environment:"
    rustc --version
    echo "Checking for cargo-watch:"
    which cargo-watch
    echo "Checking for cargo-expand:"
    which cargo-expand
    echo "Checking for rustfmt:"
    which rustfmt
  '';

  installPhase = ''
    mkdir -p $out
    echo "DevShell test complete." > $out/result.txt
  '';
}
