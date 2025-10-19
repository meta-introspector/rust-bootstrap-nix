#!/usr/bin/env bash

set -euo pipefail

echo "--- Testing Preconditions for Nix Flake Build ---"
echo ""

# --- Helper function for logging ---
log_success() {
    echo "✅ SUCCESS: $1"
}

log_failure() {
    echo "❌ FAILURE: $1"
    exit 1
}

log_info() {
    echo "ℹ️ INFO: $1"
}

# --- Precondition 1: Verify Nix command availability ---
log_info "1. Verifying 'nix' command availability..."
if command -v nix &> /dev/null; then
    log_success "Nix command found."
else
    log_failure "Nix command not found. Please install Nix."
fi
echo ""

# --- Precondition 2: Verify Rust toolchain sysroot exists ---
log_info "2. Verifying Rust toolchain sysroot for pkgs.rust-bin.stable.\"1.84.1\".default..."

RUST_TOOLCHAIN_PATH=$(nix eval --raw --extra-experimental-features "nix-command flakes" nixpkgs#rust-bin.stable.\"1.84.1\".default)

if [[ -d "$RUST_TOOLCHAIN_PATH/lib/rustlib/src/rust" ]]; then
    log_success "Rust toolchain sysroot found at: $RUST_TOOLCHAIN_PATH/lib/rustlib/src/rust"
else
    log_failure "Rust toolchain sysroot NOT found at: $RUST_TOOLCHAIN_PATH/lib/rustlib/src/rust"
fi
echo ""

echo "--- All Precondition Tests Complete ---"