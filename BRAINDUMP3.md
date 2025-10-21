# Braindump: Refactoring bootstrap-config-utils

## Current Goal:
Refactor `bootstrap-config-utils` to be a pure parsing and configuration preparation crate. It should return a `ParsedConfig` struct that is free of direct dependencies on `bootstrap` crate types.

## Steps Taken (Summary):
*   Created workspace in the current directory (`/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix`).
*   Removed conflicting `[workspace]` sections from sub-crates (`standalonex/src/bootstrap/Cargo.toml` and `standalonex/src/bootstrap/src/core/config_utils/Cargo.toml`).
*   Defined `ParsedConfig`, `LocalFlags`, `LocalCiConfig`, `LocalBuild`, `LocalLlvm`, `LocalRust`, `LocalTargetConfig`, `LocalDist` structs in `src/lib.rs` of `bootstrap-config-utils`.
*   Modified `parse_inner` function signature in `src/parse_inner.rs` to return `ParsedConfig` and accept `LocalFlags` and `LocalTomlConfig`.
*   Removed `use crate::...` statements (referencing `bootstrap` types) from `src/parse_inner.rs`.
*   Replaced `Config::default_opts()` with `ParsedConfig::default()` in `src/parse_inner.rs`.
*   Updated `parse_inner_flags` in `src/parse_inner_flags.rs` to use `ParsedConfig` and `LocalFlags`.
*   Replaced `Ci` destructuring and `set` calls with direct assignments to `ParsedConfig` fields in `src/parse_inner.rs`.
*   Commented out the `config.dry_run` block in `src/parse_inner.rs`.
*   Replaced `config.hosts` and `config.targets` assignments with direct assignments using primitive types in `src/parse_inner.rs`.
*   Replaced assignments from `build_config` to `config` fields (e.g., `nodejs`, `npm`, `gdb`, etc.), removing `set` calls.
*   Replaced `config.verbose` and `config.verbose_tests` assignments with direct assignments using primitive types.
*   Replaced `toml.install` processing with direct assignments to `ParsedConfig` fields.
*   Replaced `config.llvm_assertions` assignment with direct assignment from `toml.llvm.assertions`.
*   Removed local `let mut` declarations for LLVM, Rust, and debug info options.
*   Replaced `toml.rust` processing with direct assignments to `ParsedConfig` fields.
*   Replaced `toml.llvm` processing with direct assignments to `ParsedConfig` fields.
*   Replaced `toml.target` processing with direct assignments to `ParsedConfig` fields.
*   Commented out `config.llvm_from_ci` block.
*   Replaced `toml.dist` processing with direct assignments to `ParsedConfig` fields.
*   Replaced `toml.rustfmt` processing with direct assignments to `ParsedConfig` fields.
*   Commented out `lld_enabled` block.
*   Commented out `config.lld_mode` block.
*   Replaced `config.rust_std_features` assignment.
*   Replaced Rust debug and overflow check assignments.
*   Replaced debug info level assignments.
*   Commented out `config.stage` block.
*   Commented out `#[cfg(not(test))]` block.

## Next Steps:
1.  **Clean up `src/parse_inner.rs`**: Remove redundant `use` statements, leftover commented code, and address any remaining fields that are not yet handled (e.g., `config.src`, `config.channel`, `config.build`, `config.out`, `config.initial_cargo_clippy`, `config.initial_rustc`, `config.initial_cargo`, `config.target_config`).
2.  **Split `src/parse_inner.rs`** into smaller, more manageable modules.
3.  **Create `bootstrap-config-processor` crate**: This crate will take the `ParsedConfig` as input and construct the actual `bootstrap::Config` object.
4.  **Move logic from `bootstrap-config-utils` to `bootstrap-config-processor`**: Transfer the logic that uses `bootstrap` crate types and performs complex configuration logic.
5.  **Refactor LLVM into its own crate**: Further isolate LLVM-specific configuration and logic into a dedicated crate.