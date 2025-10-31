#!/usr/bin/env bash

# Navigate to the directory containing this script
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
cd "$SCRIPT_DIR"

# Define the actual project root for prelude-generator (parent of standalonex)
PRELUDE_GENERATOR_PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Define the root of the minimal test project
MIN_TEST_PROJECT_ROOT="$SCRIPT_DIR/min_test_project"

# Build the prelude-generator
echo "Building prelude-generator..."
cargo build -p prelude-generator

# Run the prelude-generator to extract Level 0 declarations from the minimal test project
echo "Running prelude-generator to extract Level 0 declarations from minimal test project..."
cargo run --bin prelude-generator -- \
    --extract-global-level0-decls \
    --path "$MIN_TEST_PROJECT_ROOT" \
    --generated-decls-output-dir "$SCRIPT_DIR/generated_min_decls"
