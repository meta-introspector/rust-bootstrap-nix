#!/usr/bin/env bash
pushd /data/data/com.termux.nix/files/home/rust-bootstrap-nix/standalonex/src/bootstrap
cargo run build --stage 0
#RUST_BACKTRACE=1 ./target/debug/bootstrap  check
popd
