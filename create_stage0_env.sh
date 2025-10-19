#!/usr/bin/env bash

# Host triple
HOST_TRIPLE="aarch64-unknown-linux-gnu"

# Create build directory structure
mkdir -p build/${HOST_TRIPLE}/stage0
mkdir -p build/${HOST_TRIPLE}/stage0-sysroot/lib

# Environment variables from run_rustc_shim.sh
export RUSTC_STAGE=0
export RUSTC_SNAPSHOT="/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/bin/rustc"
export RUSTC_SYSROOT="/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu"
export RUSTC_SNAPSHOT_LIBDIR="/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/lib"
export LD_LIBRARY_PATH="/nix/store/x9w1w2c9rycrdkp3ynmwjkyk2v40vyb0-get-libdir-1-84-1"
export RUST_BACKTRACE=full
export LD_DEBUG=files

# Run the bootstrap binary with --src pointing to standalonex
/nix/store/8f16fw6m01bqhzlbcc8flvj9y3fh6bhw-bootstrap-0.1.0/bin/bootstrap check --src /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/standalonex
