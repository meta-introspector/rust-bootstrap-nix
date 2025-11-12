#!/usr/bin/env bash

set -euo pipefail

echo "Building bootstrapConfigBuilder using Nix..."

# Determine the current flake's GitHub reference dynamically if possible,
# or use a hardcoded one for now as per user's instruction.
# For this specific case, we'll use the current directory as the flake reference.
FLAKE_REF="."

nix build "$FLAKE_REF#packages.aarch64-linux.bootstrapConfigBuilder"
