# Bootstrap Goal: Self-Hosting Prelude Generator

The primary goal is to create a self-hosting `prelude-generator`. This process involves applying the `prelude-generator` to its own source code to generate a new version of itself, effectively creating a feedback loop for development.

## Process Overview

1.  **Initial Analysis:** Apply the `prelude-generator`'s Rust and Cargo parsers to its own source code. This now includes a sophisticated, multi-stage pipeline for processing `use` statements, which extracts, classifies, and preprocesses them. The `UseStatement` struct, enhanced with trait-based detail fields (e.g., Git, Nix, Rust details), provides a granular understanding of each dependency, effectively mapping them as "points" on the "lattice of functions."
2.  **AST Generation:** Generate an Abstract Syntax Tree (AST) from the `prelude-generator`'s source code. This AST generation benefits from the enriched `use` statement data, allowing for more accurate dependency analysis and structural understanding.
3.  **Hugging Face Dataset:** Save the generated AST and the detailed `use` statement information into a Hugging Face dataset. This dataset serves as a rich, machine-readable representation of the `prelude-generator`'s internal structure and dependencies. The process now includes generating a `mapping.toml` file to track original file paths against their short, hash-based IDs, resolving previous issues with excessively long file paths during dataset generation.
4.  **Self-Generation:** Use the Hugging Face dataset to generate a new version of `prelude-generator`. This self-generation process leverages the comprehensive structural and dependency information to produce an optimized and potentially self-modifying version of the tool.

## Standalone Atomic Wrapper

The output of this process will be a "standalone atomic wrapper" that encapsulates the entire project. This wrapper will include:

*   A Git repository with submodules.
*   A Nix flake for reproducible builds.
*   A `Cargo.toml` file defining the project's dependencies.
*   A `Cargo.lock` file for locked dependencies.
*   A `flake.lock` file for locked Nix inputs.
*   A library that wraps other Cargo crates.

## Dependency Handling

If the `prelude-generator` depends on other Cargo crates, they will be "folded in" to the standalone atomic wrapper, ensuring that all dependencies are self-contained.

## Integration with Hugging Face Dataset Validator

The `vendor/rust/hugging-face-dataset-validator-rust` will be integrated into our source scanning process. This validator will not be used directly as a runtime dependency but rather as a tool to extract and process code.

The core principle of this integration is the canonical extraction of functions and their organization into a dependency lattice:

1.  **Canonical Function Extraction**: Each function within the codebase (including those from `hugging-face-dataset-validator-rust` and other sources) will be extracted in a canonical form. This involves standardizing their representation to facilitate consistent analysis and comparison.
2.  **Dependency Level Assignment (Lattice)**: Each extracted function will be assigned a dependency level.
    *   **Level 0 (Foundation)**: Functions that do not depend on any other functions within the analyzed scope will be classified as "Level 0" or "Foundation" functions. These form the base of our dependency lattice.
    *   **Higher Levels**: Functions depending on Level 0 functions will be Level 1, and so on, creating a hierarchical dependency graph.
3.  **Initial Extraction of Level 0 Decls**: The first step will be to extract all base-level 0 declarations (decls) from all relevant source codes. This will establish the foundational layer of our dependency lattice.

This approach allows us to consume and transform the code into a structured "lattice" representation, enabling deeper analysis and understanding of the codebase's interdependencies.
