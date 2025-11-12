# Rust Bootstrap Refactor Circuit: External Crate Isolation and Nixification

## Goal
To encapsulate each external Rust dependency within its own dedicated Rust crate and Nix flake, enabling better modularity, version control, and Nix integration.

## Inputs
*   Main project's `Cargo.toml`
*   Main project's `flake.nix`
*   Source code of the main project (Rust files)

## Outputs
*   New directories for each isolated external crate (e.g., `vendor/rust/isolated-crates/<crate-name>/`)
*   Updated main project `Cargo.toml`
*   Updated main project `flake.nix`
*   Modified Rust source files in the main project

## Pipeline Steps (Abstract Circuit)

### 1. Dependency Identification (Tool: `cargo metadata` / Custom Script)
*   **Purpose:** Identify all direct and transitive external dependencies of the main project.
*   **Process:**
    *   Run `cargo metadata --format-version=1` to get a JSON representation of the workspace.
    *   Parse the JSON to extract package names and their dependencies.
    *   Filter for external crates (not part of the current workspace).
*   **Output:** A list of unique external crate names and their versions.

### 2. Wrapper Crate Generation (Tool: `flake-template-generator` / Custom Script)
*   **Purpose:** Create a new Rust crate and Nix flake for each identified external dependency. This wrapper crate will simply re-export the functionality of the external crate.
*   **Process:**
    *   For each external crate `X`:
        *   Create a directory `vendor/rust/isolated-crates/X/`.
        *   Generate `vendor/rust/isolated-crates/X/Cargo.toml` with `X` as a dependency.
        *   Generate `vendor/rust/isolated-crates/X/src/lib.rs` with `pub use X::*;`.
        *   Generate `vendor/rust/isolated-crates/X/flake.nix` defining a `buildRustPackage` for the wrapper, pulling in the original external crate.
*   **Output:** New directories with `Cargo.toml`, `src/lib.rs`, and `flake.nix` for each wrapper crate.

### 3. Main Project `Cargo.toml` Update (Tool: Custom Script / `replace`)
*   **Purpose:** Modify the main project's `Cargo.toml` to replace direct dependencies on external crates with dependencies on the newly created wrapper crates.
*   **Process:**
    *   For each external crate `X`:
        *   Remove the original `X = { version = "..." }` entry from `[dependencies]`.
        *   Add `X-wrapper = { path = "./vendor/rust/isolated-crates/X" }`.
*   **Output:** Updated main project `Cargo.toml`.

### 4. Main Project `flake.nix` Update (Tool: Custom Script / `replace`)
*   **Purpose:** Modify the main project's `flake.nix` to include the new wrapper flakes as inputs and to use them in the `buildRustPackage` definition.
*   **Process:**
    *   For each external crate `X`:
        *   Add `X-wrapper.url = "path:./vendor/rust/isolated-crates/X"`.
        *   Update the `buildRustPackage` to depend on `X-wrapper.packages.${system}.default`.
*   **Output:** Updated main project `flake.nix`.

### 5. Source Code Refactoring (Tool: `prelude-generator` / `rust-decl-splitter` / Custom Script)
*   **Purpose:** Update the main project's Rust source files to use the new wrapper crates.
*   **Process:**
    *   **Initial Pass (prelude-generator):** Run `prelude-generator` to consolidate `use` statements.
    *   **Direct `use` Replacement:** Replace `use X::...` with `use X_wrapper::...`.
    *   **`rust-decl-splitter` (Conditional):** Use if complex macros or re-exports cause issues.
*   **Output:** Modified Rust source files in the main project.

## Verification Steps
*   Run `cargo check` on the main project.
*   Run `nix build` on the main project's flake.
*   Run existing tests.
