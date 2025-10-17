#!/bin/sh

echo "--- Collecting Nix Store Paths ---"

# Get sccache path
SCCACHE_PATH=$(nix eval --impure --raw /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust/eval-rust-env#sccache)
echo "SCCACHE_PATH=$SCCACHE_PATH"

# Get curl path
CURL_PATH=$(nix eval --impure --raw /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust/eval-rust-env#curl)
echo "CURL_PATH=$CURL_PATH"

# Get rustc path
RUSTC_PATH=$(nix eval --impure --raw /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust/eval-rust-env#rustc)
echo "RUSTC_PATH=$RUSTC_PATH"

# Get cargo path
CARGO_PATH=$(nix eval --impure --raw /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust/eval-rust-env#cargo)
echo "CARGO_PATH=$CARGO_PATH"

echo "--- Nix Store Paths Collected ---"
