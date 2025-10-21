# Braindump 5: Refactoring bootstrap-config-utils - New Strategy

## Current Goal:
Refactor `bootstrap-config-utils` to be a pure parsing and configuration preparation crate. It should return a `ParsedConfig` struct that is free of direct dependencies on `bootstrap` crate types.

## Progress Made:
*   Removed conflicting `[workspace]` sections.
*   Defined `ParsedConfig`, `LocalFlags`, `LocalCiConfig`, `LocalBuild`, `LocalLlvm`, `LocalRust`, `LocalTargetConfig`, `LocalDist` structs in `src/lib.rs` of `bootstrap-config-utils`.
*   Modified `parse_inner` function signature.
*   Removed `use crate::...` statements (referencing `bootstrap` types) from `src/parse_inner.rs`.
*   Replaced `Config::default_opts()` with `ParsedConfig::default()` in `src/parse_inner.rs`.
*   Updated `parse_inner_flags` in `src/parse_inner_flags.rs` to use `ParsedConfig` and `LocalFlags`.
*   Removed various commented-out code blocks from `src/parse_inner.rs`.
*   Removed redundant `use std::env;` from `src/parse_inner.rs`.
*   Removed blocks using undefined `cargo_clippy` and `rustc` from `src/parse_inner.rs`.
*   Removed lines using undefined `set` function and variables from `src/parse_inner.rs`.
*   Introduced `ConfigApplicator` trait in `src/lib.rs`.
*   Created `src/ci_config.rs` with `CiConfigApplicator` implementing `ConfigApplicator`.
*   Updated `src/lib.rs` to declare `pub mod ci_config;`.
*   Updated `parse_inner.rs` to use `ci_config::CiConfigApplicator` via the `ConfigApplicator` trait.
*   Created `src/build_config.rs` with `BuildConfigApplicator` implementing `ConfigApplicator`.
*   Updated `src/lib.rs` to declare `pub mod build_config;`.
*   Updated `parse_inner.rs` to use `build_config::BuildConfigApplicator` via the `ConfigApplicator` trait.
*   Created `src/install_config.rs` with `InstallConfigApplicator` implementing `ConfigApplicator`.
*   Updated `src/lib.rs` to declare `pub mod install_config;`.
*   Updated `parse_inner.rs` to use `install_config::InstallConfigApplicator` via the `ConfigApplicator` trait.
*   Added `pub install: Option<install_config::Install>,` to `LocalTomlConfig` in `src/lib.rs`.
*   Created `src/llvm_assertions_config.rs` with `LlvmAssertionsConfigApplicator` implementing `ConfigApplicator`.
*   Updated `src/lib.rs` to declare `pub mod llvm_assertions_config;`.
*   Updated `parse_inner.rs` to use `llvm_assertions_config::LlvmAssertionsConfigApplicator` via the `ConfigApplicator` trait.
*   Created `src/rust_channel_git_hash_config.rs` with `RustChannelGitHashConfigApplicator` implementing `ConfigApplicator`.
*   Updated `src/lib.rs` to declare `pub mod rust_channel_git_hash_config;`.
*   Updated `parse_inner.rs` to use `rust_channel_git_hash_config::RustChannelGitHashConfigApplicator` via the `ConfigApplicator` trait.

## Challenges Encountered:
*   Frequent API errors with the `replace` tool due to strict string matching requirements, especially with large code blocks and evolving file content. This has significantly slowed down the refactoring process.
*   Difficulty in maintaining a consistent state due to the `replace` tool's limitations.

## Proposed New Strategy:
1.  **Focus on `write_file` for entire files:** Instead of trying to use `replace` for incremental changes within a file, we will use `write_file` to completely overwrite files when significant changes are made. This will reduce the chances of `old_string` mismatches.
2.  **Batch changes:** Group related changes together and apply them in a single `write_file` operation for a given file.
3.  **Prioritize functional correctness over perfect modularity in the short term:** Get the code compiling and working with the new structure, even if some modules are still a bit large. We can refine modularity later.
4.  **Re-evaluate the "nix config generator" idea:** Once `bootstrap-config-utils` is stable and modular, we can revisit the idea of an external Nix config generator crate.
