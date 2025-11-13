#!/usr/bin/env bash

set -euo pipefail

# Directory of the standalonex flake
STANDALONEX_FLAKE_DIR="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/standalonex"
BUILD_COMMAND_SCRIPT="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/build_bootstrap_command.sh"

echo "Entering nix develop shell for standalonex and building bootstrap..."

# Call nix develop and execute the build command script within its environment
nix develop "$STANDALONEX_FLAKE_DIR" --command "$BUILD_COMMAND_SCRIPT"

if [ $? -eq 0 ]; then
    echo "Bootstrap build successful!"
else
    echo "Bootstrap build failed."
    exit 1
fi