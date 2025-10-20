#!/usr/bin/env bash

set -euxo pipefail

echo "Running bootstrap test tidy using flakes/use-bootstrap-flake..."

USE_BOOTSTRAP_FLAKE_REF="github:meta-introspector/rust-bootstrap-nix?rev=feature/CRQ-016-nixify&dir=flakes/use-bootstrap-flake"

# Get the path to the Rust source flake (from standalonex's inputs)
RUST_SRC_FLAKE_PATH=$(nix eval --raw --extra-experimental-features "nix-command flakes" --expr '
  let
    standalonexFlake = builtins.getFlake "github:meta-introspector/rust-bootstrap-nix?rev=be3f35712b133efd47073a3a45203ddca533fe01&dir=standalonex";
  in
  standalonexFlake.inputs.rustSrcFlake.outPath
')
echo RUST_SRC_FLAKE_PATH #RUST_SRC_FLAKE_PATH

# Define the flake reference for flakes/use-bootstrap-flake
#USE_BOOTSTRAP_FLAKE_REF="github:meta-introspector/rust-bootstrap-nix?rev=be3f35712b133efd47073a3a45203ddca533fe01&dir=flakes/use-bootstrap-flake"



# Run the bootstrap binary within the devShell of flakes/use-bootstrap-flake
nix develop "$USE_BOOTSTRAP_FLAKE_REF#devShells.aarch64-linux.default" --no-write-lock-file --command bash -c "bootstrap test tidy --src \"$RUST_SRC_FLAKE_PATH\""
