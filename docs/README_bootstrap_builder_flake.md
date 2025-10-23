# Bootstrap Builder Flake

This flake is responsible for building the Rust bootstrap compiler from source.

## Overall Goal:
Automate the generation, Nix build, and Git management of new flakes, ensuring each step is versioned and traceable within a lattice-structured branching model.

## Current Status:
*   `flake-template-generator` is updated to accept lattice-based branch naming components.
*   The Nix build step within `flake-template-generator` has been removed to prevent circular dependencies.
*   Git operations within `flake-template-generator` are now configured to use a configurable base branch.
*   The repository is currently clean, and previous untracked files have been removed.

## Detailed Plan:

### Phase 1: Generate and Commit the Initial Flake (using `flake-template-generator`)

1.  **Run `flake-template-generator`:**
    *   **Action:** Execute `make generate-flake-dir` from the project root.
    *   **Purpose:** This will:
        *   Build and run the `flake-template-generator`.
        *   Generate `flake.nix` and `config.toml` into `flakes/generated-config-flakes/`.
        *   Perform a Statix check on the generated flake.
        *   Perform Git operations:
            *   Checkout the configured base branch (e.g., `feature/CRQ-016-nixify`).
            *   Create a new branch with the lattice naming convention (e.g., `feature/solana-rust-1.83/aarch64/phase0/step1`).
            *   Add the generated files (`flakes/generated-config-flakes/config.toml` and `flakes/generated-config-flakes/flake.nix`).
            *   Commit these files with a descriptive message.
            *   Push the new branch to the remote repository.
    *   **Verification:**
        *   Check the output for successful completion of all steps, especially the Git operations.
        *   Run `git branch -a` to confirm the new lattice branch exists locally and remotely.
        *   Run `git log --oneline` to verify the commit on the new branch.

### Phase 2: Build and Test the Generated Flake (using `standalonex`'s `nix_build` logic)

1.  **Checkout the newly created lattice branch:**
    *   **Action:** `git checkout feature/solana-rust-1.83/aarch64/phase0/step1` (or the actual generated branch name).
    *   **Purpose:** Switch to the branch containing the generated flake.
2.  **Manually trigger Nix build of the generated flake:**
    *   **Action:** Navigate to `flakes/generated-config-flakes/` and run `nix build .#default`.
    *   **Purpose:** Verify that the generated flake is valid and builds correctly in isolation.
    *   **Verification:** Check for successful Nix build output.
3.  **Integrate `nix_build` into `standalonex`:**
    *   **Action:** Modify the `standalonex` bootstrap process to call the `run_nix_build` function (from `standalonex/src/bootstrap/src/core/nix_steps/nix_build.rs`) with the path to the generated flake.
    *   **Purpose:** Automate the Nix build of the generated flake as part of the `standalonex` workflow.

### Phase 3: Document Findings and Next Steps

1.  **Document findings:** Record any issues, successes, or observations in relevant `README.md` files or CRQs.
2.  **Define next steps:** Outline the subsequent phases of the Nixification process.