#!/usr/bin/env bash

set -euxo pipefail # Added 'x' for debugging

echo "Running bootstrap test tidy..."

# Get the path to the standalonex flake
STANDALONEX_FLAKE_REF="github:meta-introspector/rust-bootstrap-nix?rev=be3f35712b133efd47073a3a45203ddca533fe01&dir=standalonex"

# Get the path to the Rust source flake
RUST_SRC_FLAKE_PATH=$(nix eval --raw --extra-experimental-features "nix-command flakes" --expr '
  let
    standalonexFlake = builtins.getFlake "'"$STANDALONEX_FLAKE_REF"'";
  in
  standalonexFlake.inputs.rustSrcFlake.outPath
')

# Get the path to the built bootstrap binary
BOOTSTRAP_DRV_PATH=$(nix eval --raw --extra-experimental-features "nix-command flakes" "$STANDALONEX_FLAKE_REF#packages.aarch64-linux.default.drv")

# Get the output path of the built bootstrap package from its derivation
BOOTSTRAP_BINARY_PATH=$(nix-store --query --outputs "$BOOTSTRAP_DRV_PATH")

echo "BOOTSTRAP_DRV_PATH: $BOOTSTRAP_DRV_PATH" # Debug print
echo "BOOTSTRAP_BINARY_PATH: $BOOTSTRAP_BINARY_PATH" # Debug print

# Run the bootstrap binary with the correct --src argument
"$BOOTSTRAP_BINARY_PATH/bin/bootstrap" test tidy --src "$RUST_SRC_FLAKE_PATH"