#!/usr/bin/env bash

# Hardcoded path to the JSON file in the Nix store
JSON_FILE="/nix/store/hdv212g3rgir248dprwg6bhkz50kkxhb-xpy-build-output-0.1.0/xpy_json_output.json"

# Check if the JSON file exists
JSON_CONTENT=$(cat "$JSON_FILE")

# Check if JSON_CONTENT is empty
if [ -z "$JSON_CONTENT" ]; then
    echo "Error: JSON content is empty from $JSON_FILE"
    exit 1
fi

# Use nix eval to parse the JSON string
nix eval --impure --raw --expr "
  let
    jsonString = builtins.fromJSON \"$JSON_CONTENT\";
  in
  jsonString.command
"
