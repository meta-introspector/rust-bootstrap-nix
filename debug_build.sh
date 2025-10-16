#!/bin/sh
echo "--- Debugging Build Environment ---"
echo ""
echo "PATH:"
echo $PATH
echo ""
echo "which curl:"
which curl
echo ""
echo "--- Creating config.toml ---"
echo "patch-binaries-for-nix = true" > config.toml
echo "vendor = true" >> config.toml
echo "rustc = \"$(which rustc)\"" >> config.toml
echo "cargo = \"$(which cargo)\"" >> config.toml
echo ""
echo "cat config.toml:"
cat config.toml
echo ""
echo "--- Running Build ---"
python x.py --config ./config.toml build
