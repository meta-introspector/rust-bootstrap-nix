#!/usr/bin/env bash

set -euo pipefail

LOCAL_GIT_ROOT="/data/data/com.termux.nix/files/home/git/"
GITHUB_ORG="meta-introspector"
REPO_NAME="rust-bootstrap-nix"
LOCAL_REPO_PATH="${LOCAL_GIT_ROOT}${GITHUB_ORG}/${REPO_NAME}"
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo "Setting up local Git mirror for ${GITHUB_ORG}/${REPO_NAME} at ${LOCAL_REPO_PATH}"

# Create the organization directory if it doesn't exist
mkdir -p "${LOCAL_GIT_ROOT}${GITHUB_ORG}"

# If the local repository doesn't exist, clone it
if [ ! -d "${LOCAL_REPO_PATH}" ]; then
    echo "Cloning current repository to ${LOCAL_REPO_PATH}..."
    git clone --bare . "${LOCAL_REPO_PATH}"
else
    echo "Local repository already exists at ${LOCAL_REPO_PATH}. Ensuring it's up-to-date."
    # Update the bare repository with the latest from the current working directory
    (cd . && git push "${LOCAL_REPO_PATH}" "HEAD:${CURRENT_BRANCH}")
fi

# Add a local remote to the current repository if it doesn't exist
if ! git remote get-url local_mirror > /dev/null 2>&1; then
    echo "Adding local_mirror remote to current repository..."
    git remote add local_mirror "${LOCAL_REPO_PATH}"
fi

# Push the current branch to the local mirror
echo "Pushing current branch (${CURRENT_BRANCH}) to local mirror..."
git push local_mirror "${CURRENT_BRANCH}"

echo "Local mirror setup and push complete."
echo "You can now reference this repository in your flakes using: git+file://${LOCAL_REPO_PATH}?ref=${CURRENT_BRANCH}"
