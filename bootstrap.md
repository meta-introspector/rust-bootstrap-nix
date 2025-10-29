# Bootstrap Goal: Self-Hosting Prelude Generator

The primary goal is to create a self-hosting `prelude-generator`. This process involves applying the `prelude-generator` to its own source code to generate a new version of itself, effectively creating a feedback loop for development.

## Process Overview

1.  **Initial Analysis:** Apply the `prelude-generator`'s Rust and Cargo parsers to its own source code.
2.  **AST Generation:** Generate an Abstract Syntax Tree (AST) from the `prelude-generator`'s source code.
3.  **Hugging Face Dataset:** Save the generated AST into a Hugging Face dataset.
4.  **Self-Generation:** Use the Hugging Face dataset to generate a new version of `prelude-generator`.

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
