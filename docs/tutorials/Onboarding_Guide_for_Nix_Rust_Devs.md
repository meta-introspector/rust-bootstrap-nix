# Onboarding Guide for Nix-Rust Developers

Welcome to the `rust-bootstrap-nix` project! This guide will help you get started with our Nix-based Rust development environment, focusing on reproducibility, build acceleration, and a unique architectural vision.

## 1. Project Overview & Goals

This project aims to provide a robust and reproducible development and build environment for Rust, leveraging Nix flakes. Nix flakes are self-contained, reproducible development environments that define all dependencies and configurations. Our long-term architectural goal is to transform the Rust codebase into a "canonical form" represented as a "lattice of functions," enhancing modularity and enabling self-modifying capabilities. You can find more details on this vision in `lattice.md`.

Key features include:
*   **Reproducible Development Environments:** Consistent Python and Rust development shells via Nix flakes.
*   **`sccache` Integration:** Accelerated Rust compilation.
*   **`x.py` Build System Support:** Tools for working with the `x.py` build orchestration script.
*   **JSON Output Processing:** Analyzing build metadata.

## 2. Setting Up Your Development Environment with Nix Flakes

Our project uses Nix flakes to ensure a consistent and isolated development environment. If you don't have Nix installed, please follow the official Nix installation guide.

### Entering the Development Shell

To enter the project's development shell, navigate to the root of this repository and run:

```bash
nix develop
```

This command will:
*   Fetch all necessary dependencies (Rust toolchain, Python, `sccache`, etc.) as defined in the root `flake.nix`.
*   Set up your `PATH` and other environment variables to use the Nix-provided tools.
*   Ensure a clean and isolated build environment by setting `HOME` and `CARGO_HOME` to temporary directories.

### Verifying Your Setup

Once inside the `nix develop` shell, you can verify your Rust and Python installations:

```bash
rustc --version
cargo --version
python3 --version
```

## 3. Basic Build and Test Commands

### Building the Project

The primary way to build the Rust project is through the `x.py` build orchestration script, which is a custom script designed for our project's specific build process. From within the `nix develop` shell, you can run:

```bash
python x.py build
```

This command will utilize the `sccache`-enabled Rust toolchain provided by Nix to build the project.

### Building the Standalone Bootstrap

To build the standalone Rust bootstrap environment (especially useful for `aarch64-linux` environments), use:

```bash
nix build ./standalonex#packages.aarch64-linux.default
```

### Running Tests

Specific test commands will depend on the module you are working on. Generally, you can run Rust tests using `cargo test` from within a Rust crate's directory, or through `x.py` if it orchestrates tests.

## 4. Key Files and Directories

*   **`flake.nix` (root):** Defines the main development shell and overall project dependencies.
*   **`standalonex/`:** Contains the `x.py` build orchestration script and related utilities.
*   **`docs/`:** Project documentation, including this guide.
*   **`crates/`:** Contains various Rust crates that are part of the project.
*   **`nix/`:** Nix-specific configurations and dependencies.
*   **`config.toml`:** Main project configuration file.

## 5. Contributing to the Project

We welcome contributions! Please refer to our Change Request (CRQ) and Standard Operating Procedure (SOP) documents in `docs/crqs/` and `docs/sops/` for guidelines on how to propose changes, report bugs, and contribute code. Always ensure your changes adhere to our coding standards and pass all Nix and Rust checks.

### Code Quality and Shell Scripts

For shell scripts, we enforce strict code quality standards using `Shellcheck`. Before submitting any changes involving shell scripts, please ensure they pass `Shellcheck` analysis. Refer to `docs/memos/Shellcheck_Always_After_Changes.md` for detailed instructions on how to use `Shellcheck` and integrate it into your development workflow.

## 6. Further Reading

*   `README.md` (root): For a comprehensive overview of the repository.
*   `docs/Nix_Integration.md`: Detailed information on Nix integration.
*   `lattice.md`: In-depth explanation of the "Lattice of Functions" architectural goal.
*   `docs/sops/` and `docs/crqs/`: For project-specific processes and guidelines.