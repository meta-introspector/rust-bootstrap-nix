# Bootstrap Goal: Self-Hosting Prelude Generator

The primary goal is to create a self-hosting `prelude-generator`. This process involves applying the `prelude-generator` to its own source code to generate a new version of itself, effectively creating a feedback loop for development.

## Process Overview

1.  **Initial Analysis:** Apply the `prelude-generator`'s Rust and Cargo parsers to its own source code. This now includes a sophisticated, multi-stage pipeline for processing `use` statements, which extracts, classifies, and preprocesses them. The `UseStatement` struct, enhanced with trait-based detail fields (e.g., Git, Nix, Rust details), provides a granular understanding of each dependency, effectively mapping them as "points" on the "lattice of functions."
2.  **AST Generation:** Generate an Abstract Syntax Tree (AST) from the `prelude-generator`'s source code. This AST generation benefits from the enriched `use` statement data, allowing for more accurate dependency analysis and structural understanding.
3.  **Hugging Face Dataset:** Save the generated AST and the detailed `use` statement information into a Hugging Face dataset. This dataset serves as a rich, machine-readable representation of the `prelude-generator`'s internal structure and dependencies.
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
