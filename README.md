# Project Overview: `rust-bootstrap-nix` - `prelude-generator` Development Plan

This repository provides a Nix-based development and build environment for Rust projects, with a focus on integrating `sccache` for accelerated compilation and managing the `x.py` build system. This document specifically outlines the current development plan and roadmap for the `prelude-generator` crate and its integration into the broader Rust project refactoring pipeline, with the ultimate goal of achieving a self-hosting bootstrap process.

## 1. `prelude-generator` Code Review and Development Plan

This section details observations and actionable items for the `prelude-generator` crate, guided by functional programming principles. The ultimate goal is to achieve the self-hosting bootstrap process described in `bootstrap.md`.

### 1.1. Observations

#### 1.1.1. Project Structure and Configuration
*   **`Cargo.toml`**: The `Cargo.toml` file is well-structured and clearly defines the project's dependencies. The use of path dependencies for local crates (`prelude-collector`, `hf-dataset-validator`) is appropriate for the current stage of development.
*   **`flake.nix`**: The `flake.nix` file provides a reproducible development environment, which is a significant strength. It correctly sets up the Rust toolchain, OpenSSL, and other dependencies. The use of `rust-overlay` is a good practice for managing Rust toolchains in Nix.
*   **`Makefile`**: The `Makefile` provides convenient shortcuts for common tasks like building, running, and cleaning the project. This is helpful for developers.
*   **Generated Code**: The project has a clear separation between handwritten code and generated code, with the latter being placed in the `generated/` directory. This is a good practice that helps to avoid confusion and accidental modification of generated files.

#### 1.1.2. Core Logic (`src` directory)
*   **Modularity**: The code is well-modularized, with different functionalities separated into different files. This makes the code easier to understand, maintain, and test.
*   **Functional Principles**: The code demonstrates an understanding of functional programming principles, with the use of functors (`PipelineFunctor`) to create a data processing pipeline. This is a good starting point for building a robust and extensible system.
*   **Error Handling**: The use of `anyhow::Result` for error handling is consistent and makes the code more robust.
*   **Async Operations**: The use of `tokio` for asynchronous operations is appropriate for a tool that performs I/O-intensive tasks like file system operations and running external processes.
*   **`TODOs` and Incomplete Implementations**: There are several `todo!()` macros and placeholder implementations, particularly in `hf_dataset_reader.rs`. This indicates that the project is still under development and that there are areas that need to be completed.
*   **Testing**: The project has a good testing strategy, with a combination of unit tests and integration tests. The `generated_test_runner` is an interesting approach to running all tests, but it has some issues (see "Actionable Items" below).

#### 1.1.3. Functional Programming Principles
*   **`PipelineFunctor`**: The `PipelineFunctor` trait is a good example of applying functional programming principles to the project. It allows for the creation of a flexible and composable data processing pipeline.
*   **Immutability**: The code generally favors immutability, which is a core principle of functional programming.
*   **Higher-Order Functions**: The use of higher-order functions (e.g., `map`, `filter`) is prevalent throughout the code, which is another good sign of functional programming practices.

### 1.2. Actionable Items

#### 1.2.1. Code Quality and Refactoring
*   **`HuggingFaceValidatorFunctor`**: This functor is doing too much. It is responsible for creating a temporary Git repository, running `hf-validator`, and copying the results. This should be broken down into smaller, more focused functions.
*   **`expand_macros_and_parse`**: This function is also doing too much. It is responsible for creating a temporary crate, running `cargo rustc`, and parsing the output. This should be refactored into smaller, more manageable functions.
*   **Error Handling**: While the use of `anyhow::Result` is good, some of the error messages could be more specific. For example, instead of "Failed to execute hf-validator command", it would be better to include the exit code and the stderr output of the command.
*   **Logging**: The use of `println!` for logging should be replaced with a more structured logging framework like `tracing` or `log`. This will make it easier to control the log level and to filter and analyze the logs.

#### 1.2.2. Completing `todo!()` Implementations
*   **`hf_dataset_reader.rs`**: The `reconstruct_ast_from_hf_dataset` function is a placeholder. This needs to be implemented to complete the bootstrap process.

#### 1.2.3. Testing
*   **`generated_test_runner`**: The `generated_test_runner` is currently broken. It has a number of compilation errors that need to be fixed. Additionally, the approach of generating a single `main.rs` file that calls all the tests is not ideal. It would be better to use a test runner that can discover and run the tests automatically.
*   **Test Coverage**: While the project has a good number of tests, there are still some areas that are not well-tested. For example, the `hf_dataset_reader.rs` module has no tests.

## 2. High-Level Plan for `prelude-generator` and Refactoring (Roadmap)

This section outlines the next steps for improving the `prelude-generator` and integrating it into the broader Rust project refactoring pipeline, leading to the self-hosting bootstrap process.

### 2.1. Generate `prelude-generator` Report
*   Process the `prelude_generator_output.txt` to identify:
    *   Successfully parsed files.
    *   Files with parsing warnings (e.g., "expected square brackets", "cannot parse string into token stream").
    *   Skipped files (e.g., procedural macro crates, explicitly excluded crates, files with no `use` statements).
*   Create a summary report (`prelude_generator_summary.md`) detailing these findings.

### 2.2. Address `prelude.rs` Circular Imports
*   **Problem:** The current `prelude-generator` output shows `pub use crate::prelude::*;` in the generated `prelude.rs` files, leading to circular imports. This occurs because existing `prelude.rs` files (from previous incorrect runs) are being parsed by `prelude-collector`.
*   **Action:** Implement a step to delete all existing `prelude.rs` files in the workspace *before* running `prelude-generator`. This will ensure that `prelude-collector` only processes original source files and generates correct `prelude.rs` content.

### 2.3. Refine `prelude-collector` for `use` Statement Flattening
*   **Goal:** Ensure that grouped `use` statements are correctly flattened into individual `use` statements.
    *   **Example:** `use syn::{parse_macro_input, Ident};` should become `use syn::parse_macro_input;` and `use syn::Ident;`.
*   **Action:** Verify the output of `prelude-generator` after addressing step 2. If flattening is not as expected, adjust the `flatten_use_tree` logic in `crates/prelude-collector/src/lib.rs`.

### 2.4. Investigate and Mitigate `syn` Parsing Errors
*   **Problem:** Many files within the `standalonex/src/bootstrap/` directory are failing to parse with `syn` (e.g., "expected square brackets", "cannot parse string into token stream"). This is likely due to heavy use of custom macros, generated code, or non-standard syntax.
*   **Action:**
    *   Analyze the specific files causing these errors.
    *   Determine if these files can be safely excluded from `prelude-generator` processing without impacting the overall refactoring goal.
    *   If exclusion is not feasible, explore more advanced parsing strategies (e.g., using `rustc -Zunpretty=expanded` to pre-process files, or identifying specific macro patterns to ignore within `prelude-collector`). Prioritize files critical for the `bootstrap` build.

### 2.5. Integrate `prelude-generator` into the Refactoring Pipeline
*   Once `prelude-generator` is reliably generating correct `prelude.rs` files and modifying source files as intended, integrate it as a foundational step in the automated refactoring pipeline.
*   This pipeline will eventually involve other tools like `rust-decl-splitter`, `rust-system-composer`, and `flake-template-generator`.

### 2.6. Address `bootstrap` Build Failures
*   After `prelude-generator` is working correctly and has modified the necessary files, re-attempt building the `bootstrap` crate.
*   Address any remaining compilation errors. These errors should now be related to actual type/resolution issues or other build-system configurations, rather than incorrect `prelude.rs` generation.

### 2.7. Overall Bootstrap Roadmap
1.  **Fix the `generated_test_runner`**: The first step is to fix the `generated_test_runner` so that all the tests can be run. This will provide a solid foundation for the rest of the development work.
2.  **Implement `reconstruct_ast_from_hf_dataset`**: The next step is to implement the `reconstruct_ast_from_hf_dataset` function. This is the core of the bootstrap process, and it will require a significant amount of work.
3.  **Refactor the `prelude-generator`**: Once the bootstrap process is working, the `prelude-generator` should be refactored to improve its code quality and to make it more robust and extensible.
4.  **Create the "standalone atomic wrapper"**: The final step is to create the "standalone atomic wrapper" that encapsulates the entire project. This will involve creating a Git repository with submodules, a Nix flake, and all the other components described in `bootstrap.md`.

## Further Documentation

For more detailed information on specific aspects of the project, please refer to the following documents:

*   [`bootstrap.md`](./bootstrap.md): Outlines the high-level goal of creating a self-hosting `prelude-generator`.
*   [`functional.md`](./functional.md) and [`functional_arrow_best_practices.md`](./functional_arrow_best_practices.md): Provide insights into the functional programming principles guiding this project.
*   [`docs/memos/Shellcheck_Always_After_Changes.md`](./docs/memos/Shellcheck_Always_After_Changes.md): Guidelines on using `Shellcheck` for shell script quality.
*   [`qa_report.md`](./qa_report.md): Latest quality assurance report.
*   [`lattice.md`](./lattice.md), [`rollup.md`](./rollup.md): Detailed explanation of the architectural vision, including the Introspective Rollup Workflow and Lattice of Functions.
