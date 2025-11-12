# QA Plan for rust-bootstrap-nix Project

This document outlines the Quality Assurance plan for the `rust-bootstrap-nix` project, focusing on key areas and testing methodologies.

## 1. Identify Key Test Areas:

*   **`bootstrap-config-builder`:**
    *   Configuration loading, merging, and override logic.
    *   Precondition checks (`preconditions.rs`).
    *   `nix-dir` tool functionality (output, filtering, JSON).
*   **Rust Bootstrap Integration with Nix:**
    *   Correct reading and utilization of Nix configuration from `ParsedConfig`.
    *   Accurate execution of Nix commands from Rust (e.g., `nix eval`, `nix build`).
    *   Successful integration of resolved Nix store paths into the Rust build process.
    *   Verification of the 8-level bootstrap process.
*   **Overall Project Build & Test Commands:**
    *   Functionality of the main `Makefile` targets.
    *   Correctness of `nix-build` and `nix-shell` commands.
*   **`devShell` Environment:**
    *   Ensuring all necessary development tools are available and correctly configured.

## 2. Determine Test Types and Tools:

*   **Unit Tests:** Leverage Rust's built-in `cargo test` for individual functions and modules, especially within `bootstrap-config-builder` and the refactored `standalonex/src/bootstrap/src/lib.rs` components.
*   **Integration Tests:**
    *   **Rust-Nix Interaction:** Write Rust tests that call the Nix interaction logic and assert on the outcomes (e.g., correct Nix command execution, valid path resolution).
    *   **Component Interaction:** Test the flow between `bootstrap-config-builder` and the main Rust bootstrap process.
*   **System/End-to-End Tests:**
    *   **Shell Scripts:** Create or enhance existing shell scripts (`test.sh`, `run_bootstrap_test.sh`, etc.) to execute the full bootstrap process and verify the final output (e.g., successful compilation, correct artifacts).
    *   **Nix Checks:** Use `nix-build --check` and potentially `nix-diff` to ensure flake outputs are consistent and correct.
*   **Static Analysis & Linting:** Ensure pre-commit hooks (`.pre-commit-config.yaml`) are comprehensive and run regularly. This includes `rustfmt`, `clippy`, and potentially `shellcheck` for shell scripts.
*   **Documentation Review:** Regularly verify that `docs/` accurately reflects the current state and functionality of the project.

## 3. Proposed Next Steps for Implementation:

*   **Review Existing Tests:** Identify current unit, integration, and system tests.
*   **Prioritize Test Cases:** Based on the `TODO.md`, focus on critical paths first (e.g., `bootstrap-config-builder` output, core Nix integration).
*   **Expand Unit Test Coverage:** For new and refactored Rust code.
*   **Develop Integration Tests:** Specifically for the Rust-Nix interface.
*   **Enhance End-to-End Scripts:** To cover the full build process and verify outputs.
