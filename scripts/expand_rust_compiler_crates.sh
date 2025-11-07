#!/usr/bin/env bash

set -euo pipefail

RUST_SRC_PATH="/data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src"
EXPANDED_DIR="${RUST_SRC_PATH}/expanded_crates"
METADATA_DIR="${RUST_SRC_PATH}/metadata_cache"

mkdir -p "${EXPANDED_DIR}"
mkdir -p "${METADATA_DIR}"

# Read each Cargo.toml path from targets.txt
while IFS= read -r CARGO_TOML_PATH; do
    if [[ -z "$CARGO_TOML_PATH" ]]; then
        continue
    fi

    echo "Processing: ${CARGO_TOML_PATH}"

    CRATE_DIR=$(dirname "${CARGO_TOML_PATH}")
    CRATE_NAME=$(basename "${CRATE_DIR}") # Simple heuristic, might need refinement for complex cases

    # Generate metadata for the individual crate
    METADATA_FILE="${METADATA_DIR}/${CRATE_NAME}_metadata.json"
    if ! cargo metadata --format-version 1 --manifest-path "${CARGO_TOML_PATH}" > "${METADATA_FILE}"; then
        echo "WARNING: Failed to generate metadata for ${CARGO_TOML_PATH}. Skipping."
        continue
    fi

    # Extract package name from metadata (more robust way)
    PACKAGE_NAME=$(jq -r '.packages[] | select(.manifest_path == "'"${CARGO_TOML_PATH}"'") | .name' "${METADATA_FILE}")

    if [[ -z "$PACKAGE_NAME" ]]; then
        echo "WARNING: Could not determine package name for ${CARGO_TOML_PATH}. Skipping."
        continue
    fi

    # Check if the expanded output file already exists
    # Assuming the output file name starts with .expand_output_${PACKAGE_NAME}_ and ends with .rs
    OUTPUT_FILE_GLOB="${EXPANDED_DIR}/.expand_output_${PACKAGE_NAME}_*.rs"
    shopt -s nullglob # Enable nullglob to make globs that match nothing expand to nothing
    files=( ${OUTPUT_FILE_GLOB} )
    if [ ${#files[@]} -gt 0 ]; then
        echo "Skipping expansion for ${CARGO_TOML_PATH} (output file exists)."
        shopt -u nullglob # Disable nullglob
        continue
    fi
    shopt -u nullglob # Disable nullglob

    echo "  - Package Name: ${PACKAGE_NAME}"
    echo "  - Running expanded-code-collector for layer 0..."

    # Run expanded-code-collector for the specific package
    if ! cargo run --bin expanded-code-collector -- \
        --metadata-path "${METADATA_FILE}" \
        --output-dir "${EXPANDED_DIR}" \
        --layer 0 \
        --package "${PACKAGE_NAME}"; then
        echo "ERROR: expanded-code-collector failed for package ${PACKAGE_NAME} (${CARGO_TOML_PATH})."
    fi
    echo ""
done < "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/targets.txt"

echo "Expansion process complete. Expanded code is in ${EXPANDED_DIR}"
