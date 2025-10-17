# Repository Overview: `rust-bootstrap-nix`

This repository serves as a comprehensive Nix-based environment for developing, building, and testing Rust projects, with a particular focus on integrating `sccache` for build acceleration and leveraging a custom `x.py` build orchestration system. It is designed to provide reproducible build environments across different architectures (`aarch64-linux` and `x86_64-linux`).

## Core Purpose

The primary goal of `rust-bootstrap-nix` is to streamline the Rust development workflow within a Nix ecosystem. This involves:

1.  **Reproducible Toolchains:** Providing consistent and isolated Rust compiler and Cargo toolchains via Nix flakes.
2.  **Build Acceleration:** Integrating `sccache` to significantly speed up Rust compilation times.
3.  **Custom Build Orchestration:** Utilizing a Python-based `x.py` script for managing complex build processes, including dependency handling and build step execution.
4.  **Build Metadata Extraction:** Generating and processing structured JSON output from the build process for analysis and further automation.
5.  **Modular Flake Structure:** Breaking down the environment and build logic into smaller, interconnected Nix flakes for better organization and reusability.

## Key Components

The repository is structured around several key components:

*   **Nix Flakes:** A collection of `flake.nix` files that define development environments, packages, and build logic. These include the root flake, sub-flakes for JSON processing, Rust evaluation, and a standalone `x.py` environment.
*   **Shell Scripts:** Various `.sh` scripts for common tasks such as entering development shells, debugging builds, diagnosing environment issues, and updating flakes.
*   **Configuration Files:** `config.toml` files that specify build settings, toolchain paths, and vendoring options.
*   **`standalonex/` Directory:** A critical component containing the `x.py` build orchestration script, Python utilities (`test_json_output.py`, `wrap_rust.py`), and Rust source code (`src/`). This directory is central to how the Rust project is built and how build metadata is generated.
*   **`src/bootstrap/bootstrap.py`:** The core Python script within `standalonex/src/bootstrap/` that implements the detailed logic for the Rust build process, including toolchain management, environment setup, and JSON output generation.

## How it Works (High-Level)

The system leverages Nix flakes to define a hermetic build environment. The root `flake.nix` sets up a development shell with Python, Rust, and `sccache`. The `x.py` script (located in `standalonex/`) acts as the primary interface for building the Rust project. During the build, `x.py` (specifically through its `bootstrap` module) can generate JSON output containing detailed information about the compilation steps. Other flakes then consume and process this JSON data, enabling advanced analysis and automation of the Rust build process.

## Further Documentation

For more in-depth information on specific aspects of the repository, please refer to:

*   **Nix Flakes Documentation:** [`NIX_FLAKES_DOCUMENTATION.md`](./NIX_FLAKES_DOCUMENTATION.md)
*   **Scripts Documentation:** [`SCRIPTS_DOCUMENTATION.md`](./SCRIPTS_DOCUMENTATION.md)
*   **Configuration Documentation:** [`CONFIGURATION.md`](./CONFIGURATION.md)
