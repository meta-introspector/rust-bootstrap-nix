#!/bin/sh
echo "--- Debugging Build Environment ---"
echo ""
echo "PATH:"
echo $PATH
echo ""
echo "which curl:"
which curl
echo ""
echo "--- Creating config.toml using bootstrap-config-builder ---"
./bootstrap-config-builder/target/debug/bootstrap-config-builder \
    --project-root "$(pwd)" \
    --rustc-path "$(which rustc)" \
    --cargo-path "$(which cargo)" \
    --patch-binaries-for-nix true \
    --vendor true \
    --output config.toml
echo ""
echo "cat config.toml:"
cat config.toml
echo ""
echo "--- Running Build ---"
python x.py --config ./config.toml build
