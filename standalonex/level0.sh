#!/usr/bin/env bash

PROJECT_ROOT="$(dirname "$(readlink -f "$0")")"
cd "$PROJECT_ROOT"

# Define the actual project root for prelude-generator
PRELUDE_GENERATOR_PROJECT_ROOT="$(dirname "$PROJECT_ROOT")"

# Build the prelude-generator
echo "Building prelude-generator..."
cargo build

# Run the prelude-generator to extract global Level 0 declarations
echo "Running prelude-generator to extract global Level 0 declarations..."
cargo run --bin prelude-generator -- --extract-global-level0-decls --path "$PRELUDE_GENERATOR_PROJECT_ROOT" --generated-decls-output-dir generated/level0_decls
