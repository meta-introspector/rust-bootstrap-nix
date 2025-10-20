#!/usr/bin/env bash

LOG_FILE="bootstrap_build_output.log"

echo "Entering Nix develop shell and running bootstrap build..."
unset LD_DEBUG # Unset LD_DEBUG here to reduce verbosity

# Get the rustSrcPath using nix eval from the use-bootstrap-flake
RUST_SRC_PATH_VAL=$(nix eval --raw --extra-experimental-features "nix-command flakes" \
  "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b")

nix develop ./flakes/use-bootstrap-flake#devShells.aarch64-linux.default --command env RUST_SRC_PATH="$RUST_SRC_PATH_VAL" LOG_FILE="$LOG_FILE" bash -c ' 
  echo "Inside the develop shell."
  echo "Running bootstrap build..."

  # The RUST_SRC_PATH and LOG_FILE should now be available as environment variables.
  echo "Value of RUST_SRC_PATH: $RUST_SRC_PATH"

  # Execute the bootstrap command with 'check' and the rust source path.
  bootstrap check --src "$RUST_SRC_PATH" > "$LOG_FILE" 2>&1

  echo "Bootstrap build finished. Logs saved to $LOG_FILE"
'

echo "Script finished."
