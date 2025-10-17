#!/bin/sh
echo "--- Diagnosing Build Environment ---"
echo ""
echo "--- Environment Variables ---"
echo "HOME: $HOME"
echo "CARGO_HOME: $CARGO_HOME"
echo "PATH: $PATH"
echo ""
echo "--- Tool Locations (using grep PATH) ---"
echo "curl: $(echo $PATH | tr ':' '\n' | xargs -I {} find {} -maxdepth 1 -name 'curl' 2>/dev/null)"
echo "rustc: $(echo $PATH | tr ':' '\n' | xargs -I {} find {} -maxdepth 1 -name 'rustc' 2>/dev/null)"
echo "cargo: $(echo $PATH | tr ':' '\n' | xargs -I {} find {} -maxdepth 1 -name 'cargo' 2>/dev/null)"
echo ""
echo "--- config.toml Content ---"
cat config.toml
echo ""
echo "--- Running Build with Verbose Output ---"
python x.py build -vv