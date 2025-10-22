# Plan: Refactor Bootstrap Configuration with a Standalone Rust Driver

This plan adopts an "outside-in" approach to development. We will first develop and test the core bootstrap configuration logic in a standalone Rust environment for speed and ease of debugging, and then package the proven solution into a Nix flake.

## Phase 1: Standalone Rust-driven Bootstrap Configuration

1.  **Isolate Logic:** The core logic from the `configuration-nix` crate will be extracted and refactored into a new, standalone Cargo project. This project will be a standard Rust binary, not a Nix flake.

2.  **"Read-Only" Nix Interaction:** The new Rust binary will be responsible for generating the `config.toml` file. It will achieve this by querying the Nix environment for necessary paths (e.g., Rust source, dependencies) without running inside a `nix shell`. This maintains a fast and responsive development cycle.

3.  **File Generation Strategy:** To avoid issues with in-place editing, all refactoring will directly overwrite files using `write_file` after modifications are confirmed to be working correctly. This replaces the previous strategy of creating `.refactored.rs` files.

4.  **Manual Execution and Verification:** The bootstrap process will be executed manually from a standard shell. We will use our new Rust executable to generate the `config.toml`, and then run the existing bootstrap scripts (like `./x.py build`) to test the generated configuration.

## Phase 2: Nix Integration

1.  **Package the Solution:** Once the standalone Rust driver is fully functional and robustly tested, it will be packaged as a new Nix flake.

2.  **Final Integration:** This new flake, which provides the bootstrap configuration executable, will be integrated into the main project's Nix infrastructure. It will replace the previous, slower, and more complex Nix-based configuration generation scripts.