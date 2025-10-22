# BRAINDUMP: Refactoring bootstrap-config-utils

## Overall Goal
Resolve build errors for the `bootstrap` crate and its dependencies within the `rust-bootstrap-nix` workspace, with a primary focus on making `bootstrap-config-utils` a self-contained "layer 1" crate that only reads and validates inputs, with no dependencies on `bootstrap` or `build_helper`.

## Current State (as of latest `report.txt`)

### Progress Made:
*   **`build_helper` path resolution**: The persistent issue of `cargo` failing to find `build_helper/Cargo.toml` has been resolved by temporarily moving `build_helper` to `standalonex/src/bootstrap/build_helper` and updating `Cargo.toml` files accordingly. (Note: This was a temporary measure to isolate the problem, and `build_helper` is now being removed as a dependency as per user's latest directive).
*   **Cyclic Dependency**: The cyclic dependency between `bootstrap` and `bootstrap-config-utils` has been broken.
*   **`Deserialize` Errors**: `E0252: Deserialize defined multiple times` (in `install_config.rs`) and `E0599: no function or associated item named `deserialize` found for struct `LocalTomlConfig`` (in `get_toml.rs`) have been addressed.
*   **`E0507` Ownership Error**: Fixed in `ci_config.rs`.
*   **`unclosed delimiter` Error**: Fixed in `parse_inner_build.rs`.
*   **`Path` and `fs` Imports**: `use std::path::Path;` and `use std::fs;` have been re-added to `get_toml.rs`.
*   **`BUILDER_CONFIG_FILENAME`**: Defined in `get_builder_toml.rs`.
*   **Dummy Types**: `RustOptimize` and `TargetSelection` dummy types have been defined in `lib.rs`.
*   **Type Replacements in `default_opts.rs`**: `Config` replaced with `crate::ParsedConfig`, `RustOptimize` with `crate::RustOptimize`, `TargetSelection` with `crate::TargetSelection`, and `CiConfig` with `crate::LocalCiConfig`.
*   **`ParsedConfig` Field Additions (Partial)**: The first batch of missing fields (`bypass_bootstrap_lock`, `llvm_optimize`, `ninja_in_file`, `llvm_static_stdcpp`, `llvm_libzstd`, `backtrace`, `rust_optimize_tests`, `docs`, `docs_minification`, `rust_rpath`, `rust_strip`, `rust_dist_src`, `deny_warnings`, `dist_include_mingw_linker`) have been added to `ParsedConfig` in `lib.rs`.

### Remaining Problems (from latest `report.txt`):

1.  **Duplicate field declarations in `ParsedConfig`**: Several fields (e.g., `docs_minification`, `docs`, `rust_optimize_tests`, etc.) are now declared more than once in `ParsedConfig` in `lib.rs`. This happened because some fields were already present before I added them.
2.  **`error[E0432]: unresolved import `bootstrap`**: Still present in `parse_inner_src.rs`, `parse_inner_out.rs`, `parse_inner_stage0.rs`, `parse_inner_toml.rs`, `dry_run.rs`, `try_run.rs`.
3.  **`error[E0432]: unresolved import `build_helper`**: Still present in `parse_inner_stage0.rs` and `try_run.rs`. This needs to be removed as per the user's directive.
4.  **`error[E0432]: unresolved import `crate::llvm_assertions_config` and `crate::rust_channel_git_hash_config`**: These modules are still not found.
5.  **`error[E0433]: failed to resolve: you might be missing crate `core``**: Still present in `parse_inner_build.rs`.
6.  **`error[E0560]: struct `ParsedConfig` has no field named ...`**: Still present for `channel`, `codegen_tests`, `stdout_is_tty`, `stderr_is_tty`, `src`, `ci`, `targets`. These fields need to be added to `ParsedConfig`.
7.  **`error[E0308]: mismatched types`**: Still present for various fields in `default_opts.rs` where `bool` or `PathBuf` or `String` are being assigned to `Option<T>`. These need to be wrapped in `Some()`.
8.  **`error[E0609]: no field `triple` on type `TargetSelection`**: In `get_builder_toml.rs`. `TargetSelection` is a tuple struct `(String)`, so `triple` is not a field. It should be accessed as `config.build.0`.
9.  **`error[E0277]: the trait bound `LocalLlvm: Clone` is not satisfied`**, etc.: `Clone` trait not implemented for `LocalLlvm`, `LocalRust`, `LocalTargetConfig`, `Install`. These need `#[derive(Clone)]`.
10. **`error[E0507]: cannot move out of `toml.build` which is behind a mutable reference`**: In `parse_inner_build.rs`. This requires `clone()` or `as_ref()/as_mut()`.

## Plan Moving Forward:

1.  **`bootstrap-config-builder` Refactoring Complete**: The `bootstrap-config-builder` crate has been successfully refactored to use utility functions in `utils.rs` and now correctly generates `config.toml` by querying Nix flakes. This was achieved by directly overwriting files using `write_file` after modifications were confirmed.
2.  **Clean up `ParsedConfig` duplicates**: Carefully review `lib.rs` and remove any duplicate field declarations in `ParsedConfig`.
3.  **Implement `Clone` for structs**: Add `#[derive(Clone)]` to `LocalLlvm`, `LocalRust`, `LocalTargetConfig`, and `Install` structs in `lib.rs` and `install_config.rs` respectively.
4.  **Address `default_opts.rs` field errors**: 
    *   Add remaining missing fields (`channel`, `codegen_tests`, `stdout_is_tty`, `stderr_is_tty`, `src`, `ci`, `targets`) to `ParsedConfig` in `lib.rs`.
    *   Wrap `bool`, `PathBuf`, `String` values in `Some()` where `Option<T>` is expected in `default_opts.rs`.
5.  **Fix `TargetSelection` access**: In `get_builder_toml.rs`, change `config.build.triple` to `config.build.0`.
6.  **Remove `build_helper` imports**: Go through `parse_inner_stage0.rs` and `try_run.rs` and remove `use build_helper;` and any code that relies on it.
7.  **Remove `bootstrap` imports**: Systematically go through all files in `bootstrap-config-utils` and remove `use bootstrap::...` statements. Replace `bootstrap::Config` with `crate::ParsedConfig`, `bootstrap::Flags` with `crate::LocalFlags`, `bootstrap::TomlConfig` with `crate::LocalTomlConfig`. For other `bootstrap` types/functions, either copy their definitions into `lib.rs` (if basic) or remove/refactor their usage.
8.  **Address `crate::llvm_assertions_config` and `crate::rust_channel_git_hash_config`**: Create dummy modules for these in `bootstrap-config-utils/src/` if they are truly internal to `bootstrap-config-utils` and not external dependencies.
9.  **Address `crate::core` and `crate::utils`**: Comment out or refactor code that uses these if they are not part of `bootstrap-config-utils`.
10. **Fix `E0507` in `parse_inner_build.rs`**: Change `toml.build.unwrap_or_default()` to `toml.build.clone().unwrap_or_default()`.
11. **Re-run `report.sh`** after each significant batch of changes.

---

# Refactoring Summary (BRAINDUMP2.md)

## 1. Splitting `test.rs`

The large `standalonex/src/bootstrap/src/core/build_steps/test.rs` file was split into smaller, more manageable modules.

*   **Original File Renamed:** `test.rs` was renamed to `test_temp.rs`.
*   **New `test.rs` Created:** A new `test.rs` file was created containing:
    *   Original `use` statements.
    *   `mod` declarations for each extracted `pub struct` and `fn` definition.
    *   Original macro definitions (`macro_rules! default_test!`, `macro_rules! test_book!`, etc.) and their invocations.
    *   Internal references within the macros to the extracted modules were updated with `crate::` prefix (e.g., `crate::compiletest::Compiletest`).
*   **Individual Files Created:** Each `pub struct` and `fn` definition from the original `test.rs` (excluding macros) was moved into its own `.rs` file within the `test_split/` directory.

## 2. Refactoring `Rustc<T>` Step Implementations

The common `should_run` and `make_run` methods for `Rustc<T>` across `check.rs` and `clippy.rs` were refactored.

*   **Shared `should_run` Function:** A new file `standalonex/src/bootstrap/src/core/build_steps/rustc_step_common.rs` was created with a shared function `rustc_should_run`.
*   **`check.rs` and `clippy.rs` Updated:** Both `check.rs` and `clippy.rs` were modified to use `rustc_should_run` and include the necessary `use` statement.
*   **Unified `make_run` Logic:**
    *   The `RustcTaskConfig` trait in `standalonex/src/bootstrap/src/core/types.rs` was extended with a `default_config` method.
    *   `default_config` was implemented for `CheckRustcConfig` and `LintConfig` in `types.rs`.
    *   The `make_run` method for `Rustc<T>` in both `check.rs` and `clippy.rs` was unified to use `default_config`.

## 3. Refactoring `Std` Struct and Step Implementations

The `Std` struct, which had different fields in `check.rs` and `clippy.rs`, was refactored to be generic.

*   **Generic `Std` Struct:** A new `StdTaskConfig` trait and a generic `Std<C: StdTaskConfig>` struct were introduced in `standalonex/src/bootstrap/src/core/types.rs`.
*   **Concrete `StdTaskConfig` Implementations:** `CheckStdConfig` and `ClippyStdConfig` were created in `types.rs` to hold the specific configuration for `Std` in `check.rs` and `clippy.rs` respectively.
*   **`check.rs` Updated:** The old `pub struct Std` definition was removed, and the `impl Step for Std` block was updated to `impl Step for Std<CheckStdConfig>`, with adjustments to `make_run` and `run` methods to use the generic `Std` and `CheckStdConfig`.
*   **`clippy.rs` Updated:** The old `pub struct Std` definition was removed, and the `impl Step for Std` block was updated to `impl Step for Std<ClippyStdConfig>`, with adjustments to `make_run` and `run` methods to use the generic `Std` and `ClippyStdConfig`.

## 4. `config_standalone` and `build_helper` Dependency Issues

Attempts to compile `config_standalone` as a separate crate encountered persistent issues with `build_helper` path dependencies.

*   **Problem:** Cargo repeatedly failed to resolve the `build_helper` dependency, often looking for it at incorrect or duplicated paths, despite attempts to correct relative paths in `Cargo.toml` files and clear Cargo caches.
*   **Conclusion:** The complex nested path dependency structure within the `bootstrap` project, or a potential misconfiguration of the Cargo workspace, makes it difficult to easily compile sub-modules like `config` as truly standalone crates without significant manual intervention or deeper understanding of the project's build system.
*   **Current Status:** The user will handle the build issues for `config_standalone`.

---

# Braindump: Refactoring bootstrap-config-utils

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

## Plan Moving Forward:

1.  **`bootstrap-config-builder` Refactoring Complete**: The `bootstrap-config-builder` crate has been successfully refactored to use utility functions in `utils.rs` and now correctly generates `config.toml` by querying Nix flakes. This was achieved by directly overwriting files using `write_file` after modifications were confirmed.
2.  **Clean up `ParsedConfig` duplicates**: Carefully review `lib.rs` and remove any duplicate field declarations in `ParsedConfig`.
3.  **Implement `Clone` for structs**: Add `#[derive(Clone)]` to `LocalLlvm`, `LocalRust`, `LocalTargetConfig`, and `Install` structs in `lib.rs` and `install_config.rs` respectively.
4.  **Address `default_opts.rs` field errors**: 
    *   Add remaining missing fields (`channel`, `codegen_tests`, `stdout_is_tty`, `stderr_is_tty`, `src`, `ci`, `targets`) to `ParsedConfig` in `lib.rs`.
    *   Wrap `bool`, `PathBuf`, `String` values in `Some()` where `Option<T>` is expected in `default_opts.rs`.
5.  **Fix `TargetSelection` access**: In `get_builder_toml.rs`, change `config.build.triple` to `config.build.0`.
6.  **Remove `build_helper` imports**: Go through `parse_inner_stage0.rs` and `try_run.rs` and remove `use build_helper;` and any code that relies on it.
7.  **Remove `bootstrap` imports**: Systematically go through all files in `bootstrap-config-utils` and remove `use bootstrap::...` statements. Replace `bootstrap::Config` with `crate::ParsedConfig`, `bootstrap::Flags` with `crate::LocalFlags`, `bootstrap::TomlConfig` with `crate::LocalTomlConfig`. For other `bootstrap` types/functions, either copy their definitions into `lib.rs` (if basic) or remove/refactor their usage.
8.  **Address `crate::llvm_assertions_config` and `crate::rust_channel_git_hash_config`**: Create dummy modules for these in `bootstrap-config-utils/src/` if they are truly internal to `bootstrap-config-utils` and not external dependencies.
9.  **Address `crate::core` and `crate::utils`**: Comment out or refactor code that uses these if they are not part of `bootstrap-config-utils`.
10. **Fix `E0507` in `parse_inner_build.rs`**: Change `toml.build.unwrap_or_default()` to `toml.build.clone().unwrap_or_default()`.
11. **Re-run `report.sh`** after each significant batch of changes.