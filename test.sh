#!/bin/sh

set -euo pipefail

echo "--- Setting up Test Environment (Replicating Nix Build) ---"

# Define paths and variables (replicating Nix build environment)
TMPDIR=$(mktemp -d)
export HOME="$TMPDIR"
export CARGO_HOME="$HOME/.cargo"
export CARGO_TARGET_DIR="$TMPDIR/target"
export CARGO_WORKSPACE_ROOT="/nonexistent/workspace/root" # Attempt to trick cargo into not finding a workspace
mkdir -p "$CARGO_HOME"
mkdir -p "$CARGO_TARGET_DIR"

# Determine the current flake's GitHub reference dynamically if possible,
# or use a hardcoded one for now as per user's instruction.
# For this specific case, the user provided the exact reference.
FLAKE_REF="github:meta-introspector/rust-bootstrap-nix?rev=be3f35712b133efd47073a3a45203ddca533fe01&dir=standalonex"

# Run the build command within the Nix shell
nix shell "$FLAKE_REF#devShells.aarch64-linux.default" --command "./build_rust_bootstrap.sh"

echo "--- Cleaning up temporary directory ---"
rm -rf "$TMPDIR"
