#!/usr/bin/env bash

PRELUDE_GENERATOR_SRC_DIR="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/prelude-generator/src/"

find "$PRELUDE_GENERATOR_SRC_DIR" -name "*.rs" | while read -r file_path; do
  echo "Processing file: $file_path"
  cargo run -p prelude-generator -- --file "$file_path" --run-pipeline --config-file-path /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/config.toml
  if [ $? -ne 0 ]; then
    echo "Error running prelude-generator for $file_path. Aborting."
    exit 1
  fi
done

echo "Successfully regenerated prelude-generator using its own source."
