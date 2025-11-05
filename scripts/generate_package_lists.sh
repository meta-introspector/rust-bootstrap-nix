#!/bin/bash

METADATA_FILE="rust-bootstrap-core/full_metadata.json"

if [ ! -f "$METADATA_FILE" ]; then
    echo "Error: Metadata file not found at $METADATA_FILE" >&2
    exit 1
fi

PACKAGES=$(jq -r '.packages[] | select(.source == null) | .name' "$METADATA_FILE" | sort | uniq | paste -s -d' ')
BINARY_PACKAGES=$(jq -r '.packages[] | select(.targets[]?.kind[]? == "bin") | .name' "$METADATA_FILE" | sort | uniq | paste -s -d' ')

echo "PACKAGES := "
for pkg in $PACKAGES; do
    echo "    $pkg "
done
echo

echo "BINARY_PACKAGES := "
for pkg in $BINARY_PACKAGES; do
    echo "    $pkg "
done
echo
