{ pkgs ? import (builtins.getFlake "github:meta-introspector/nixpkgs?rev=26833ad1dad83826ef7cc52e0009ca9b7097c79f") { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];
}
