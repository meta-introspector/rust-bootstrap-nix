# Status Report: Refactoring `bootstrap-config-types`

## Date: October 26, 2025

## Current Task
Refactoring the `bootstrap-config-types` crate to improve modularity and resolve cyclic dependencies by moving the `Config` struct and its associated logic into a new crate, `bootstrap-config-builder-core`.

## Progress Made

1.  **New Crate Creation:** A new Cargo library crate, `standalonex/src/bootstrap-config-builder-core`, has been successfully created.
2.  **Code Migration:** The content of `standalonex/src/bootstrap-config-types/src/config_part4.rs` (which contained the `Config` struct implementation) has been moved to `standalonex/src/bootstrap-config-builder-core/src/lib.rs`.
3.  **Dependency Management (`bootstrap-config-builder-core`):**
    *   `standalonex/src/bootstrap-config-builder-core/Cargo.toml` has been updated to include necessary external dependencies (`anyhow`, `semver`, `clap`, `clap_complete`, `serde`, `toml`).
    *   Relative paths for internal dependencies (`config_macros`, `config_core`, `build_helper`) within `bootstrap-config-builder-core/Cargo.toml` have been corrected.
4.  **Cyclic Dependency Resolution:**
    *   The cyclic dependency between `bootstrap-config-types` and `bootstrap-config-builder-core` has been addressed by removing `bootstrap-config-builder-core` as a dependency from `bootstrap-config-types/Cargo.toml`.
    *   `standalonex/src/bootstrap-config-types/src/lib.rs` has been modified to remove redundant module declarations and now re-exports types directly from `bootstrap-config-builder-core`.

## Next Steps

1.  **Import Refactoring in `bootstrap-config-builder-core`:** Continue to refactor and correct all import statements within `standalonex/src/bootstrap-config-builder-core/src/lib.rs`. This involves:
    *   Ensuring correct `use` statements for `std` modules (e.g., `std::collections::HashMap`, `std::sync::OnceLock`).
    *   Explicitly importing types and functions from `config_core` and `build_helper` (e.g., `build_helper::ci::CiEnv`, `build_helper::util::t!`, `build_helper::git::git`).
    *   Identifying the correct location and importing `get_toml`, `get_builder_toml`, `check_incompatible_options_for_ci_rustc`, and `is_download_ci_available` functions.
2.  **Compilation and Error Resolution:** Attempt to build the project to identify and resolve any remaining compilation errors related to missing types, functions, or incorrect paths.
3.  **Code Splitting Evaluation:** Once the project compiles successfully, evaluate if `bootstrap-config-builder-core/src/lib.rs` needs further splitting into smaller, more manageable modules based on topological dependencies and logical groupings.