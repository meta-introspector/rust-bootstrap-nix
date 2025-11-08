#!/usr/bin/env bash

set -euo pipefail

RUST_SRC_PATH="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src"
EXPANDED_DIR="${RUST_SRC_PATH}/expanded_crates"
METADATA_DIR="${RUST_SRC_PATH}/metadata_cache"
LOG_DIR="${PWD}/logs/expand_rust_compiler_crates"

FAILED_METADATA=()
FAILED_PACKAGE_NAME=()
FAILED_COLLECTOR=()

mkdir -p "${EXPANDED_DIR}"
mkdir -p "${METADATA_DIR}"
mkdir -p "${LOG_DIR}"

# Read each Cargo.toml path from targets.txt
while IFS= read -r CARGO_TOML_PATH; do
    if [[ -z "$CARGO_TOML_PATH" ]]; then
        continue
    fi

    echo "Processing: ${CARGO_TOML_PATH}"

    CRATE_DIR=$(dirname "${CARGO_TOML_PATH}")
    CRATE_NAME=$(basename "${CRATE_DIR}") # Simple heuristic, might need refinement for complex cases

    METADATA_LOG="${LOG_DIR}/${CRATE_NAME}_metadata.log"
    COLLECTOR_LOG="${LOG_DIR}/${CRATE_NAME}_collector.log"

    # Generate metadata for the individual crate
    METADATA_FILE="${METADATA_DIR}/${CRATE_NAME}_metadata.json"
    if ! cargo metadata --format-version 1 --manifest-path "${CARGO_TOML_PATH}" > "${METADATA_FILE}" 2> "${METADATA_LOG}"; then
        echo "WARNING: Failed to generate metadata for ${CARGO_TOML_PATH}. See ${METADATA_LOG} for details. Skipping."
        FAILED_METADATA+=("${CARGO_TOML_PATH}")
        continue
    fi

    # Extract package name from metadata (more robust way)
    PACKAGE_NAME=$(jq -r '.packages[] | select(.manifest_path == "'"${CARGO_TOML_PATH}"'") | .name' "${METADATA_FILE}")

    if [[ -z "$PACKAGE_NAME" ]]; then
        echo "WARNING: Could not determine package name for ${CARGO_TOML_PATH}. Skipping."
        FAILED_PACKAGE_NAME+=("${CARGO_TOML_PATH}")
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
    if ! cargo run --manifest-path "${RUST_SRC_PATH}/vendor/rust/rust-bootstrap-nix/crates/expanded-code-collector/Cargo.toml" --bin expanded-code-collector -- \
        --metadata-path "${METADATA_FILE}" \
        --output-dir "${EXPANDED_DIR}" \
        --layer 0 \
        --package "${PACKAGE_NAME}" > "${COLLECTOR_LOG}" 2>&1; then
        echo "ERROR: expanded-code-collector failed for package ${PACKAGE_NAME} (${CARGO_TOML_PATH}). See ${COLLECTOR_LOG} for details."
        FAILED_COLLECTOR+=("${CARGO_TOML_PATH}")
    fi
    echo ""
done < "${PWD}/priority_cargos.txt"

echo "Expansion process complete. Expanded code is in ${EXPANDED_DIR}"

if [ ${#FAILED_METADATA[@]} -gt 0 ]; then
    echo "--- Failed Metadata Generation ---"
    for item in "${FAILED_METADATA[@]}"; do
        echo "$item"
    done
fi

if [ ${#FAILED_PACKAGE_NAME[@]} -gt 0 ]; then
    echo "--- Failed Package Name Determination ---"
    for item in "${FAILED_PACKAGE_NAME[@]}"; do
        echo "$item"
    done
fi

if [ ${#FAILED_COLLECTOR[@]} -gt 0 ]; then
    echo "--- Failed Expanded-Code-Collector ---"
    for item in "${FAILED_COLLECTOR[@]}"; do
        echo "$item"
    done
fi
