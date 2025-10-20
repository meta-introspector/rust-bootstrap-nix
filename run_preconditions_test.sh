#!/usr/bin/env bash

set -euo pipefail

echo "Running precondition tests using Nix shell..."

# Determine the current flake's GitHub reference dynamically if possible,
# or use a hardcoded one for now as per user's instruction.
# For this specific case, the user provided the exact reference.
FLAKE_REF="github:meta-introspector/rust-bootstrap-nix?rev=be3f35712b133efd47073a3a45203ddca533fe01&dir=standalonex"

nix shell "$FLAKE_REF#devShells.aarch64-linux.default" -- bash -c "echo 'DevShell loaded successfully'"