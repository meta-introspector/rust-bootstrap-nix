#!/usr/bin/env bash

METADATA_FILE="rust-bootstrap-core/full_metadata.json"

if [ ! -f "$METADATA_FILE" ]; then
    echo "Error: Metadata file not found at $METADATA_FILE" >&2
    exit 1
fi

jq -r '.packages[] | select(.source == null) | .name' "$METADATA_FILE" | sort | uniq > scripts/packages.list
jq -r '.packages[] | select(.targets[]?.kind[]? == "bin") | .name' "$METADATA_FILE" | sort | uniq > scripts/binary_packages.list
