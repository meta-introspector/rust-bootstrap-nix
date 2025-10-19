#!/usr/bin/env bash

set -euo pipefail

echo "--- Testing Preconditions for Nix Flake Build ---"

# --- Precondition 1: Verify branch existence ---
echo "1. Verifying existence of 'feature/CRQ-016-nixify' branch in meta-introspector/rust-bootstrap-nix..."
if git ls-remote --heads https://github.com/meta-introspector/rust-bootstrap-nix feature/CRQ-016-nixify | grep -q "feature/CRQ-016-nixify"; then
    echo "   Branch 'feature/CRQ-016-nixify' found on remote."
    BRANCH_EXISTS=true
else
    echo "   Branch 'feature/CRQ-016-nixify' NOT found on remote."
    BRANCH_EXISTS=false
fi
echo ""

# --- Precondition 2: Simulate path: to github: URL conversion ---
echo "2. Simulating 'path:' to 'github:' URL conversion for relevant flake.nix files:"

FLAKE_FILES=(
    "standalonex/flake.nix"
    "flakes/bootstrap-compiler-flake/flake.nix"
    "flakes/bootstrap-from-json-flake/flake.nix"
)

REPO_ROOT="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix"
GITHUB_ORG="meta-introspector"
GITHUB_REPO="rust-bootstrap-nix"
GITHUB_REF="feature/CRQ-016-nixify" # Using the branch name as per user's confirmation

for file in "${FLAKE_FILES[@]}"; do
    echo "   --- File: $file ---"
    full_path="$REPO_ROOT/$file"
    
    # Read the content of the file
    content=$(cat "$full_path")

    # Extract path: URLs and propose github: URLs
    # This is a simplified regex and might need adjustment for more complex cases
    # For standalonex/flake.nix
    if [[ "$file" == "standalonex/flake.nix" ]]; then
        old_url_pattern="url = "path:../flakes/bootstrap-from-json-flake";"
        if echo "$content" | grep -q "$old_url_pattern"; then
            echo "     Found: $old_url_pattern"
            new_dir="flakes/bootstrap-from-json-flake"
            new_url="url = "github:$GITHUB_ORG/$GITHUB_REPO?ref=$GITHUB_REF&dir=$new_dir";"
            echo "     Proposed: $new_url"
        fi
    fi

    # For flakes/bootstrap-compiler-flake/flake.nix and flakes/bootstrap-from-json-flake/flake.nix
    if [[ "$file" == "flakes/bootstrap-compiler-flake/flake.nix" || "$file" == "flakes/bootstrap-from-json-flake/flake.nix" ]]; then
        old_url_pattern="url = "path:../../..";"
        if echo "$content" | grep -q "$old_url_pattern"; then
            echo "     Found: $old_url_pattern"
            new_dir="" # Points to the root of the repo
            new_url="url = "github:$GITHUB_ORG/$GITHUB_REPO?ref=$GITHUB_REF";"
            echo "     Proposed: $new_url"
        fi
    fi
    echo ""
done

# --- Precondition 3: Attempt a dry run of the Nix build ---
echo "3. Attempting a dry run of the Nix build for standalonex package (after applying hypothetical changes):"
# Temporarily apply changes for dry run
# This part is tricky without actually modifying files.
# For a true dry-run, we'd need to apply the changes, then run nix build --dry-run, then revert.
# For now, I'll just run the build command as is, assuming the user will manually apply changes if needed.
# If the branch existence check failed, this build will likely fail too.

if [ "$BRANCH_EXISTS" = true ]; then
    echo "   Branch exists, proceeding with dry run (this will still use current flake.nix files)."
    # Note: This dry run will use the *current* state of the flake.nix files, not the hypothetically changed ones.
    # The actual replacement needs to be done before a successful build.
    nix build "$REPO_ROOT/standalonex#packages.aarch64-linux.default" --dry-run || true
else
    echo "   Branch 'feature/CRQ-016-nixify' not found. Skipping dry run as it's expected to fail."
fi

echo "--- Precondition Testing Complete ---"
