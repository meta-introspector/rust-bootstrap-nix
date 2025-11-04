#!/usr/bin/env bash

INPUT_FILE="../expanded/.expand_output_ast-stats-crate.rs"
OUTPUT_TOML="generated_workspace/global.toml"
LOG_FILE="split_expanded_bin_verbose.log"

echo "Running split-expanded-bin with verbose logging..."

cargo run --package split-expanded-bin -- \
    --files "${INPUT_FILE}" \
    --project-root "$(pwd)/generated_workspace" \
    --rustc-version 1.89.0 \
    --rustc-host aarch64-unknown-linux-gnu \
    --verbosity 3 \
    --output-global-toml "${OUTPUT_TOML}" 2>&1 | tee "${LOG_FILE}"

echo "Finished. Log saved to ${LOG_FILE}"

