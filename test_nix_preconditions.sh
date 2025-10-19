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

RUST_TOOLCHAIN_PATH=$(nix eval --raw --extra-experimental-features "nix-command flakes" --expr '
  let
    flake = builtins.getFlake "path:."; # Reference the current flake
    pkgs = import flake.inputs.nixpkgs {
      system = builtins.currentSystem;
      overlays = [ flake.inputs.rust-overlay.overlays.default ]; # Use the rust-overlay input
    };
  in
  pkgs.rust-bin.stable."1.84.1".default
')

if [[ -d "$RUST_TOOLCHAIN_PATH/lib/rustlib/src/rust" ]]; then
    log_success "Rust toolchain sysroot found at: $RUST_TOOLCHAIN_PATH/lib/rustlib/src/rust"
else
    log_failure "Rust toolchain sysroot NOT found at: $RUST_TOOLCHAIN_PATH/lib/rustlib/src/rust"
fi
echo ""

# --- Precondition 3: Verify Rust source flake (rustSrcFlake) exists ---
log_info "3. Verifying Rust source flake (rustSrcFlake) exists..."

RUST_SRC_FLAKE_PATH=$(nix path-info --json github:meta-introspector/rust?ref=3487cd3843083db70ee30023f19344568ade9c9f | jq -r '.[0].path')

if [[ -d "$RUST_SRC_FLAKE_PATH" ]]; then
    log_success "Rust source flake found at: $RUST_SRC_FLAKE_PATH"
    # Further check for a known file within the source
    if [[ -f "$RUST_SRC_FLAKE_PATH/src/ci/channel" ]]; then
        log_success "Known file 'src/ci/channel' found within Rust source flake."
    else
        log_failure "Known file 'src/ci/channel' NOT found within Rust source flake. Path might be incorrect or incomplete."
    fi
else
    log_failure "Rust source flake NOT found at: $RUST_SRC_FLAKE_PATH"
fi
echo ""

echo "--- All Precondition Tests Complete ---"