#!/usr/bin/env bash

cargo run --bin prelude-generator -- --extract-global-level0-decls --path . --generated-decls-output-dir generated/level0_decls
