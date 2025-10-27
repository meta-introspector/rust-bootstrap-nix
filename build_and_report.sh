#!/usr/bin/env bash

# This script enters the Nix development shell, builds the bootstrap-config-generator,
# and captures the output into report.txt.

# Define the path to the bootstrap-config-builder directory
BOOTSTRAP_CONFIG_BUILDER_DIR="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/bootstrap-config-builder"

# Define the report file path
REPORT_FILE="report.txt"

echo "Building Rust project within Nix development shell and capturing output to ${REPORT_FILE}..."

# Run nix develop with a command to execute cargo build and redirect output to a temporary file
nix develop --command bash -c "cd \"${BOOTSTRAP_CONFIG_BUILDER_DIR}\" && cargo build --bin bootstrap-config-generator > build_output.tmp 2>&1"


# Check the exit status of the nix develop command
NIX_DEVELOP_EXIT_CODE=$?

# Copy the temporary build output to the report file in the root directory
cp "${BOOTSTRAP_CONFIG_BUILDER_DIR}/build_output.tmp" "${REPORT_FILE}"

if [ ${NIX_DEVELOP_EXIT_CODE} -eq 0 ]; then
  echo "Build successful. Check ${REPORT_FILE} for details."
else
  echo "Build failed. Check ${REPORT_FILE} for errors."
fi