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

**Overall Goal:** Automate the generation, Nix build, and Git management of new flakes, ensuring versioning and traceability within a lattice-structured branching model for `solana-rust-1.84.1`.

**Current State:**
*   `flake-template-generator` has been refactored into modular components (`args.rs`, `config_parser.rs`, `flake_generator.rs`, `file_writer.rs`, `statix_checker.rs`).
*   The `run_git_command` function and Git operations have been completely removed from `flake-template-generator`.
*   A new Git utility function, `create_and_push_branch`, exists in `standalonex/src/bootstrap/src/core/generate_steps/git_modules/create_branch.rs`.
*   The `flake-template-generator/Makefile` has been updated to `COMPONENT := solana-rust-1.84.1`.
*   The current Git branch is `feature/CRQ-016-nixify`.

**Detailed Plan:**

**Phase 1: Prepare the Environment and Generate the Flake**

1.  **Ensure Clean Git State (Verification):**
    *   Run `git status` to confirm no uncommitted changes on `feature/CRQ-016-nixify`.
    *   If there are uncommitted changes, either commit them or stash them.

2.  **Generate the Flake Files for `solana-rust-1.84.1`:**
    *   **Action:** Execute `make -C flake-template-generator generate-flake`.
    *   **Expected Outcome:** This will generate `flake.nix` and `config.toml` in the directory `flakes/solana-rust-1.84.1/aarch64/phase0/step1/` (relative to the project root). The `flake.nix` will use `pkgs.writeText` and the `config.toml` will reflect the `solana-rust-1.84.1` component.
    *   **Verification:** Check for the existence of the generated files and their content.

**Phase 2: Git Management using `create_and_push_branch`**

1.  **Create a Temporary Rust Binary to Call `create_and_push_branch`:**
    *   **Action:** Create a new directory `temp_git_runner` at the project root.
    *   **Action:** Create `temp_git_runner/Cargo.toml` with `standalonex` as a path dependency.
        ```toml
        [package]
        name = "temp_git_runner"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
        standalonex = { path = "../../standalonex" }
        ```
    *   **Action:** Create `temp_git_runner/src/main.rs` with the following content:
        ```rust
        use standalonex::bootstrap::src::core::generate_steps::git_modules::create_branch::create_and_push_branch;
        use std::path::PathBuf;

        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf(); // Adjust path to point to the main project root

            let base_branch = "feature/CRQ-016-nixify".to_string();
            let new_branch = "feature/solana-rust-1.84.1/aarch64/phase0/step1".to_string();
            let generated_files_path = "flakes/solana-rust-1.84.1/aarch64/phase0/step1".to_string();
            let commit_message = format!("feat: Generated seed flake {}", new_branch);

            create_and_push_branch(
                &repo_root,
                &base_branch,
                &new_branch,
                &generated_files_path,
                &commit_message,
                false, // dry_run: set to true for testing, false for actual execution
                false, // verbose
            )?;

            Ok(())
        }
        ```
    *   **Verification:** Confirm the files are created correctly.

2.  **Build and Run the Temporary Git Runner:**
    *   **Action:** Execute `cargo run --manifest-path temp_git_runner/Cargo.toml`.
    *   **Expected Outcome:** The `create_and_push_branch` function will:
        *   Checkout `feature/CRQ-016-nixify`.
        *   Check if `feature/solana-rust-1.84.1/aarch64/phase0/step1` exists.
        *   If not, create and checkout `feature/solana-rust-1.84.1/aarch64/phase0/step1`.
        *   Add the generated files from `flakes/solana-rust-1.84.1/aarch64/phase0/step1`.
        *   Commit the changes with the specified message.
        *   Push the new branch to `origin`.
    *   **Verification:**
        *   Run `git status` to confirm the current branch is `feature/solana-rust-1.84.1/aarch64/phase0/step1` and there are no uncommitted changes.
        *   Run `git log -1` to verify the commit message.
        *   (Optional) Check the remote repository to confirm the new branch exists.

3.  **Clean Up the Temporary Git Runner:**
    *   **Action:** Remove the `temp_git_runner` directory: `rm -rf temp_git_runner`.
    *   **Verification:** Confirm the directory is deleted.

**Phase 3: Nix Build of the Generated Flake**

1.  **Build the Generated Flake:**
    *   **Action:** Navigate to the generated flake directory: `cd flakes/solana-rust-1.84.1/aarch64/phase0/step1/`.
    *   **Action:** Execute `nix build .#default`.
    *   **Expected Outcome:** The Nix build should succeed, producing a `result` symlink to the built `config.toml`.
    *   **Verification:** Check for the `result` symlink and its content.

**Phase 4: Integrate Nix Build into `standalonex` (Future Step - Not part of this detailed plan, but noted for context)**

*   Modify the `standalonex` bootstrap process to call `run_nix_build` with the generated flake path.

**Phase 5: Document Findings and Next Steps (Future Step - Not part of this detailed plan, but noted for context)**

*   Record issues, successes, or observations; outline subsequent Nixification phases.