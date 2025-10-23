# TODO List for rust-bootstrap-nix Project

This document outlines the immediate next steps and ongoing tasks for the `rust-bootstrap-nix` project.

## Work Done (Summary of recent progress):

*   **Rust Precondition Checks:** Converted the logic from `run_preconditions_test.sh` and `test_nix_preconditions.sh` into Rust, implemented in `bootstrap-config-builder/src/preconditions.rs`.
*   **`bootstrap-config-builder` Refactoring:** The `bootstrap-config-builder/src/utils.rs` module has been refactored into a more organized structure with sub-modules.
*   **Logging & Dry-Run:** Added comprehensive logging and a `--dry-run` option to the `bootstrap-config-builder` for better visibility and testing.
*   **`nix-dir` Tool:** Created a new binary tool (`nix-dir`) to inspect Nix flakes and their attributes.
*   **Error Resolution:** Successfully resolved several compilation and Nix evaluation errors encountered during development.
*   **Configuration System Refactoring**: Started refactoring `bootstrap-config-builder` to support loading configuration from `config.toml` with command-line overrides.
    *   Created `bootstrap-config-builder/src/config.rs` for `AppConfig` struct.
    *   Modified `bootstrap-config-builder/src/args.rs` to make arguments optional and add `config_file` option.
*   **Rust Workspace for `standalonex`**: Created `standalonex/src/Cargo.toml` to define a workspace and updated `standalonex/flake.nix` to correctly reference the workspace `Cargo.lock`.

## Next Steps:

### 1. Refine `nix-dir` Tool

*   **Detailed Output:** Enhance the `nix-dir` tool to provide more detailed output for flake attributes, including types and descriptions.
*   **Filtering & Searching:** Implement capabilities for filtering and searching flake attributes.
*   **JSON Output:** Add a `--json` output option for programmatic use and easier integration with other tools.

### 2. Improve `bootstrap-config-builder`

*   **Implement `config.toml` loading and merging**: This will be the primary focus.
    *   Modify `bootstrap-config-builder/src/main.rs` to parse arguments, load `config.toml`, merge configurations, and pass the final config.
    *   Implement `read_config_file` helper function.
*   **Dynamic Flake Resolution:** This will be handled by the new configuration system.
*   **Handle Missing Inputs:** This will be handled by the new configuration system.
*   **Remove `--impure` Flag:** This will be addressed as part of the overall Nix integration.
*   **Clean Up Unused Imports:** This is an ongoing task.

### 3. Integrate `bootstrap-config-builder` into the Build Process

*   **Makefile Integration:** Create a robust Makefile target to run `bootstrap-config-builder` to generate `config.toml` as a prerequisite for the main build process.
*   **`config.toml` Consumption:** Ensure the generated `config.toml` is correctly consumed and utilized by the Rust bootstrap process.

### 4. Continue with Overall Project Goals

*   **Define Packages/Applications:** Further define and refine packages and applications within the Nix flake.
*   **Build & Test Commands:** Set up comprehensive build and test commands for the entire project.
*   **Refine `devShell`:** Continue to refine the `devShell` environment for optimal development experience.