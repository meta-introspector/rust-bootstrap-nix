#!/usr/bin/env bash

PROJECT_ROOT="$(dirname "$(readlink -f "$0")")"
cd "$PROJECT_ROOT"

# Define the actual project root for prelude-generator
PRELUDE_GENERATOR_PROJECT_ROOT="$(dirname "$PROJECT_ROOT")"

OUTPUT_DIR="$PROJECT_ROOT/generated/level0_decls"



# Ensure the output directory exists
mkdir -p "$OUTPUT_DIR" || { echo "Error: Failed to create output directory $OUTPUT_DIR."; exit 1; }

# Build the prelude-generator
    echo "Building prelude-generator..."
cargo build || { echo "Error: prelude-generator build failed."; exit 1; }

# Run the prelude-generator to extract global Level 0 declarations
echo "Running prelude-generator to extract global Level 0 declarations to $OUTPUT_DIR..."
cargo run --bin prelude-generator -- \
    --extract-global-level0-decls \
    --path "$PRELUDE_GENERATOR_PROJECT_ROOT" \
    --generated-decls-output-dir "$OUTPUT_DIR"
if [ $? -eq 0 ]; then
    echo "Level 0 declarations generated and cached in $OUTPUT_DIR."
else
    echo "Error: prelude-generator failed. Output in $OUTPUT_DIR might be incomplete or corrupted."
    exit 1
fi
