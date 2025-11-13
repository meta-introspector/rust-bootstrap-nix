# Gemini Instructions for `standalonex` Directory

This directory (`standalonex`) is configured to use the **nightly Rust toolchain** sourced from `github:meta-introspector/nixpkgs`.

## Current Setup:
*   **Rust Toolchain:** `pkgs.rust-bin.nightly.latest.default` from `nixpkgs` input.
*   **`devShell`:** Provides `python3` and the nightly Rust toolchain (rustc, cargo) in its PATH.
*   **`buildRustPackage`s:** `default`, `bootstrap-main`, and `nix-bootstrap` are all configured to build with the nightly Rust toolchain.

## How to Enter the Development Shell:
To enter the `nix develop` shell for this directory, navigate to `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/standalonex/` and run:
```bash
nix develop
```

## Next Steps (within the `nix develop` shell):
1.  **Verify Rust Version:** Run `rustc --version` and `cargo --version` to confirm the nightly versions are active.
2.  **Build `bootstrap`:**
    ```bash
    cargo build --bin bootstrap
    ```
3.  **Build `nix_bootstrap`:**
    ```bash
    cargo build --bin nix_bootstrap
    ```
4.  **Run Tests (if applicable):** Execute any relevant tests for the `standalonex` project.
