#!/usr/bin/env bash

set -e

echo "Running tests in ...."
pushd "."
cargo test
popd

echo "Running tests in ./generated_test_runner..."
pushd "./generated_test_runner"
cargo test
popd

