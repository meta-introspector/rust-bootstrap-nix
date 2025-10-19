#!/usr/bin/env bash

export RUSTC_STAGE=0
export RUSTC_SNAPSHOT="/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/bin/rustc"
export RUSTC_SYSROOT="/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu"
export RUSTC_SNAPSHOT_LIBDIR="/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/lib"
export LD_LIBRARY_PATH="/nix/store/x9w1w2c9rycrdkp3ynmwjkyk2v40vyb0-get-libdir-1-84-1"
export RUST_BACKTRACE=full
export LD_DEBUG=files

BOOTSTRAP_RUSTC_PATH=$(nix path-info --json ./standalonex#packages.aarch64-linux.default | jq -r '.[0].path')
"$BOOTSTRAP_RUSTC_PATH/bin/rustc" --version 2>&1 | tee bootstrap_debug_direct.log
