#!/usr/bin/env bash

export RUSTC_STAGE=0

RUST_TOOLCHAIN_BASE_PATH=$(nix eval --raw --extra-experimental-features "nix-command flakes" --expr '
  let
    standalonexFlake = builtins.getFlake "github:meta-introspector/rust-bootstrap-nix?rev=be3f35712b133efd47073a3a45203ddca533fe01&dir=standalonex";
    pkgs = import standalonexFlake.inputs.nixpkgs {
      system = "aarch64-linux";
      overlays = [ standalonexFlake.inputs.rustOverlay.overlays.default ];
    };
  in
  pkgs.rust-bin.stable."1.84.1".default
')

export RUSTC_SNAPSHOT="$RUST_TOOLCHAIN_BASE_PATH/bin/rustc"
export RUSTC_SYSROOT="$RUST_TOOLCHAIN_BASE_PATH"
export RUSTC_SNAPSHOT_LIBDIR="$RUST_TOOLCHAIN_BASE_PATH/lib"
export LD_LIBRARY_PATH="/nix/store/x9w1w2c9rycrdkp3ynmwjkyk2v40vyb0-get-libdir-1-84-1"
export RUST_BACKTRACE=full
export LD_DEBUG=files

BOOTSTRAP_RUSTC_PATH=$(nix path-info --json ./standalonex#packages.aarch64-linux.default | jq -r '.[0].path')
"$BOOTSTRAP_RUSTC_PATH/bin/rustc" --version 2>&1 | tee bootstrap_debug_direct.log
