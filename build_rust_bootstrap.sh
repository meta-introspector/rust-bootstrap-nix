#!/bin/sh

set -euo pipefail

pushd /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/standalonex/src/bootstrap/
cargo build
popd