#!/usr/bin/env bash

set -euo pipefail

echo "Running precondition tests using Nix shell..."

# Determine the current flake's GitHub reference dynamically if possible,
# or use a hardcoded one for now as per user's instruction.
# For this specific case, the user provided the exact reference.
FLAKE_REF="github:meta-introspector/time-2025?ref=feature/CRQ-016-nixify&dir=vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix"

nix shell "$FLAKE_REF#devShells.aarch64-linux.default" -- ./test_nix_preconditions.sh