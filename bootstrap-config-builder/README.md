# `bootstrap-config-builder`

This directory contains the `bootstrap-config-builder` crate, a utility for generating `config.toml` files used in the Rust bootstrap process.

## Development Environment

A Nix development shell is available for this crate. To enter the development environment, navigate to this directory and run:

```bash
nix develop
```

This will provide an environment with `rustc`, `cargo`, `rust-analyzer`, and `openssl` development libraries, allowing for smooth development and compilation of the crate.

## Build Instructions

To build the `bootstrap-config-builder` crate, ensure you are in the `nix develop` shell (as described above) and then run:

```bash
cargo build
```

## Recent Changes

This crate has recently undergone updates to improve its Nix integration and address compilation issues:

*   **Nix `devShell` Enabled:** A `devShell` has been added to `flake.nix` to provide a consistent development environment with necessary tools and dependencies.
*   **OpenSSL Dependency Resolution:** `pkgs.openssl` was added to the `devShell`'s `buildInputs`, and `PKG_CONFIG_PATH` was configured to resolve `openssl-sys` compilation errors.
*   **Import Path Correction:** The import path for `nix_eval_utils` in `src/utils/get_flake_input.rs` was corrected to `crate::utils::nix_eval_utils`.
*   **Type Inference and Sized Trait Fixes:** Type annotation for `toml::from_str` and correct iteration over `HashMap` entries were implemented in `src/bin/nix-dir.rs` to resolve type inference and `Sized` trait compilation errors.