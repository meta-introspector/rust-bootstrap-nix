#!/usr/bin/env bash

set -euo pipefail

LOCAL_GIT_ROOT="/data/data/com.termux.nix/files/home/git/"
GITHUB_ORG="meta-introspector"
REPO_NAMES=("rust-overlay" "rust" "naersk")

echo "Setting up local Git mirrors for meta-introspector repositories..."

# Define a mapping of repository names to their original paths
declare -A ORIGINAL_REPO_PATHS_MAP
ORIGINAL_REPO_PATHS_MAP["nixpkgs"]="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/nixpkgs"
ORIGINAL_REPO_PATHS_MAP["rust-overlay"]="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/nix/rust-overlay"
ORIGINAL_REPO_PATHS_MAP["rust"]="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src"
ORIGINAL_REPO_PATHS_MAP["naersk"]="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/nix/naersk"

for REPO_NAME in "${REPO_NAMES[@]}"; do
    LOCAL_REPO_PATH="${LOCAL_GIT_ROOT}${GITHUB_ORG}/${REPO_NAME}"
    ORIGINAL_REPO_PATH="${ORIGINAL_REPO_PATHS_MAP[${REPO_NAME}]}"

    echo "--- Processing ${REPO_NAME} ---"

    # Create the organization directory if it doesn't exist
    mkdir -p "${LOCAL_GIT_ROOT}${GITHUB_ORG}"

    # Check if ORIGINAL_REPO_PATH exists
    if [ ! -d "${ORIGINAL_REPO_PATH}" ]; then
        echo "Warning: Original repository for ${REPO_NAME} not found at ${ORIGINAL_REPO_PATH}. Cannot clone or push. Skipping."
        continue
    fi

    # If the local bare repository doesn't exist, clone it
    if [ ! -d "${LOCAL_REPO_PATH}" ]; then
        echo "Cloning ${REPO_NAME} from ${ORIGINAL_REPO_PATH} to ${LOCAL_REPO_PATH}..."
        git clone --bare "${ORIGINAL_REPO_PATH}" "${LOCAL_REPO_PATH}"
    else
        echo "Local bare repository for ${REPO_NAME} already exists at ${LOCAL_REPO_PATH}."
    fi

    # Ensure the original repository has a remote pointing to the local mirror
    (
        cd "${ORIGINAL_REPO_PATH}" || exit

        CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
        if ! git remote get-url local_mirror > /dev/null 2>&1; then
            echo "Adding local_mirror remote to ${ORIGINAL_REPO_PATH}..."
            git remote add local_mirror "${LOCAL_REPO_PATH}"
        fi
        echo "Pushing all branches of ${REPO_NAME} to local mirror..."
        git push local_mirror --all
    )

    echo "--- Finished processing ${REPO_NAME} ---"
done

echo "All local mirror setups and pushes complete."