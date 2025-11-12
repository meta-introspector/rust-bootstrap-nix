#!/usr/bin/env bash

set -euo pipefail

echo "Building bootstrap binary..."
cd standalonex/src/bootstrap
cargo build --bin bootstrap