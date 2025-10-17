# Configuration Documentation

This document details the various configuration files used within the `rust-bootstrap-nix` repository, primarily focusing on `config.toml` files that influence the Rust build process and environment setup.

## 1. Root `config.toml`

**File Path:** `/config.toml`

**Description:** This is the primary configuration file for the overall `rust-bootstrap-nix` environment. It explicitly defines how the Rust toolchain is sourced and how the build environment is isolated.

**Key Settings:**

*   `vendor = true`:
    *   **Purpose:** Enables vendoring for the Rust build process. This means that dependencies are expected to be present locally (e.g., in a `vendor/` directory) rather than being downloaded from the internet during the build. This is crucial for reproducible builds in a Nix environment.
*   `rustc = "/nix/store/.../bin/rustc"`:
    *   **Purpose:** Specifies the absolute path to the `rustc` (Rust compiler) executable within the Nix store. This ensures that the build uses a precisely defined and versioned compiler provided by Nix.
*   `cargo = "/nix/store/.../bin/cargo"`:
    *   **Purpose:** Specifies the absolute path to the `cargo` (Rust package manager) executable within the Nix store. Similar to `rustc`, this guarantees the use of a specific, Nix-managed `cargo` instance.
*   `HOME = "/data/data/com.termux.nix/files/usr/tmp/..."`:
    *   **Purpose:** Sets the `HOME` environment variable to a temporary, isolated directory. This prevents the build process from interacting with or polluting the user's actual home directory.
*   `CARGO_HOME = "/data/data/com.termux.nix/files/usr/tmp/.../.cargo"`:
    *   **Purpose:** Sets the `CARGO_HOME` environment variable to a temporary `.cargo` directory. This ensures that Cargo's caches, registries, and other state are kept isolated within the build environment.

**Overall Purpose:** The root `config.toml` is fundamental for establishing a hermetic and reproducible Rust build environment. It explicitly directs the build system to use Nix-provided tools and to operate within a clean, temporary workspace.

## 2. `standalonex/config.toml`

**File Path:** `/standalonex/config.toml`

**Description:** This configuration file is specific to the `standalonex` component, which is a standalone environment for the `x.py` build system. It defines the Rust toolchain paths that `x.py` should use within this isolated context.

**Key Settings:**

*   `rustc = "/nix/store/.../bin/rustc"`:
    *   **Purpose:** Similar to the root `config.toml`, this specifies the absolute path to the `rustc` executable, ensuring that the `standalonex` environment uses a Nix-provided compiler.
*   `cargo = "/nix/store/.../bin/cargo"`:
    *   **Purpose:** Specifies the absolute path to the `cargo` executable for the `standalonex` environment, guaranteeing the use of a specific, Nix-managed `cargo` instance.

**Overall Purpose:** This `config.toml` ensures that the `standalonex` build environment, particularly when running `x.py`, is correctly configured with the appropriate Nix-provided Rust toolchain binaries.

## 3. `standalonex/config.old.toml`

**File Path:** `/standalonex/config.old.toml`

**Description:** This file appears to be an older or template version of `standalonex/config.toml`. It is specifically used by the `standalonex/flake.nix`'s `buildPhase` as a base to generate the active `config.toml` by injecting the correct Nix store paths for `rustc` and `cargo` using `sed`.

**Purpose:** To serve as a template for generating the runtime `config.toml` within the `standalonex` build process, allowing for dynamic injection of Nix-specific paths.
