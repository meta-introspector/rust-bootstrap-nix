#!/usr/bin/env bash

# Define the base directory (current working directory)
BASE_DIR=$(pwd)

# List of flake directories relative to BASE_DIR
FLAKE_DIRS=(
    "flakes/config"
    "flakes/xpy-json-output-flake"
    "flakes/json-processor"
    "flakes/json-processor-flake"
    "flakes/evaluate-rust"
    "." # The current directory itself
    "standalonex"
)

LOG_FILE="nix_flake_update_$(date +%Y%m%d_%H%M%S).log"

echo "Starting Nix flake updates..." | tee -a "$LOG_FILE"
echo "Logging all output to: $LOG_FILE" | tee -a "$LOG_FILE"

for dir in "${FLAKE_DIRS[@]}"; do
    FULL_PATH="$BASE_DIR/$dir"
    if [ -d "$FULL_PATH" ]; then
        echo "----------------------------------------------------" | tee -a "$LOG_FILE"
        echo "Processing flake in: $FULL_PATH" | tee -a "$LOG_FILE"

        # --- Pre-update grep for url ---
        echo "--- Pre-update grep for 'url' in flake.lock ---" | tee -a "$LOG_FILE"
        (cd "$FULL_PATH" && grep "url" flake.lock 2>/dev/null) | tee -a "$LOG_FILE"
        if [ $? -ne 0 ]; then
            echo "No 'url' found in flake.lock (or flake.lock does not exist)." | tee -a "$LOG_FILE"
        fi

        # --- Pre-update grep for NixOS ---
        echo "--- Pre-update grep for 'NixOS' in flake.lock ---" | tee -a "$LOG_FILE"
        (cd "$FULL_PATH" && grep -i "NixOS" flake.lock 2>/dev/null) | tee -a "$LOG_FILE"
        if [ $? -ne 0 ]; then
            echo "No 'NixOS' found in flake.lock (or flake.lock does not exist)." | tee -a "$LOG_FILE"
        fi

        # --- Run nix flake update ---
        echo "--- Running nix flake update --verbose ---" | tee -a "$LOG_FILE"
        (
            cd "$FULL_PATH" || exit
            timeout 10s nix flake update --verbose 2>&1 | tee -a "$LOG_FILE"
        )
        UPDATE_STATUS=$?
        if [ $UPDATE_STATUS -ne 0 ]; then
            echo "Error updating flake in $FULL_PATH. Exit code: $UPDATE_STATUS. Continuing..." | tee -a "$LOG_FILE"
        fi

        # --- Post-update grep for url ---
        echo "--- Post-update grep for 'url' in flake.lock ---" | tee -a "$LOG_FILE"
        (cd "$FULL_PATH" && grep "url" flake.lock 2>/dev/null) | tee -a "$LOG_FILE"
        if [ $? -ne 0 ]; then
            echo "No 'url' found in flake.lock (or flake.lock does not exist)." | tee -a "$LOG_FILE"
        fi

        # --- Post-update grep for NixOS ---
        echo "--- Post-update grep for 'NixOS' in flake.lock ---" | tee -a "$LOG_FILE"
        (cd "$FULL_PATH" && grep -i "NixOS" flake.lock 2>/dev/null) | tee -a "$LOG_FILE"
        if [ $? -ne 0 ]; then
            echo "No 'NixOS' found in flake.lock (or flake.lock does not exist)." | tee -a "$LOG_FILE"
        fi

    else
        echo "Warning: Directory $FULL_PATH not found. Skipping." | tee -a "$LOG_FILE"
    fi
done

echo "----------------------------------------------------" | tee -a "$LOG_FILE"
echo "All flake updates attempted." | tee -a "$LOG_FILE"