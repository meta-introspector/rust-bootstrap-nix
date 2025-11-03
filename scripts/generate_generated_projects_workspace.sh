#!/usr/bin/env bash

GENERATED_PROJECTS_ROOT="/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/generated_projects"
CARGO_TOML="${GENERATED_PROJECTS_ROOT}/Cargo.toml"

# Start Cargo.toml content
CARGO_TOML_CONTENT="[workspace]\nmembers = [\n"

# Iterate through subdirectories and add them as members
for dir in "${GENERATED_PROJECTS_ROOT}"/*; do
    if [ -d "$dir" ]; then
        project_name=$(basename "$dir")
        if [ "$project_name" != "target" ]; then
            CARGO_TOML_CONTENT+="    \"${project_name}\",\n"
        fi
    fi
done

# Close Cargo.toml content
CARGO_TOML_CONTENT+="]\n"

# Write to file
echo -e "${CARGO_TOML_CONTENT}" > "${CARGO_TOML}"

echo "Generated ${CARGO_TOML} with workspace members."

