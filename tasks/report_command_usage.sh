#!/usr/bin/env bash

# This script generates a report of the command object usage in the CodeGraph.

# Define paths
PROJECT_ROOT="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix"
CODE_GRAPH_PATH="${PROJECT_ROOT}/.gemini/generated/code_graph.json"
OUTPUT_REPORT_PATH="${PROJECT_ROOT}/.gemini/generated/command_usage_report.txt"
QUERY_TOOL_BIN="${PROJECT_ROOT}/target/debug/code-graph-query-tool"

# Ensure the output directory exists
mkdir -p "$(dirname "${OUTPUT_REPORT_PATH}")"

echo "Generating 'command-usage' report..."
"${QUERY_TOOL_BIN}" \
    --graph-path "${CODE_GRAPH_PATH}" \
    --query-type "command-usage" \
    --output-path "${OUTPUT_REPORT_PATH}"

if [ $? -eq 0 ]; then
    echo "Report generated successfully at: ${OUTPUT_REPORT_PATH}"
    echo "Content of the report:"
    cat "${OUTPUT_REPORT_PATH}"
else
    echo "Error generating report."
    exit 1
fi
