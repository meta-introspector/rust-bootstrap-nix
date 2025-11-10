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

### Phase I: Enabling the Lattice Infrastructure (P1/P2 Blocking Tasks)

These initial steps resolve the critical blocking issues (P1) and implement the core structural logic (P2) required to define and process a Level 0 node.

| Step | Priority & Task (Component Added) | Rationale and Lattice Contribution |
| :--- | :--- | :--- |
| **1** | **Resolve `rust-system-composer` `main()` location (Task 02\_06)**. | **Status:** **Critical Blocking Issue.** The `rust-system-composer` acts as the orchestrator for the entire pipeline. Resolving its main function location is necessary to **unblock the batch processing pipeline** required to analyze the full codebase and extract declarations. |
| **2** | **Implement Bag of Words (BoW) and Coordinate Grouping logic (Task 02\_01)**. | **Contribution:** **Defines the Node Structure.** This implements the core mechanism for converting a declaration into a quantifiable lattice node. The BoW (referenced identifiers/types) is the **foundation for calculating the declaration's 8D coordinate** and grouping declarations into highly efficient, dependency-optimized **~4KB chunks**. |

---

### Phase II: Constructing Canonical Layer 0 (Foundation)

With the infrastructure running and the node definition finalized, we proceed to extract and index the most basic, atomic units, specifically constants, which form the base of the lattice.

| Step | Priority & Task (Component Added) | Rationale and Lattice Contribution |
| :--- | :--- | :--- |
| **3** | **Extract Base-Level 0 Declarations (Task 05\_08 / 01\_10)**. | **Component Added: The first content layer (Level 0 Declarations, initially `const` items).** This task involves completing the extraction and **isolation of atomic units without internal dependencies**. These extracted declarations must be placed in **layer-specific directories** (e.g., `generated_declarations/0/`) and structured according to the **4KB file blocking** constraint. |
| **4** | **Implement global index and 8D embedding for all constants (Task 02\_04)**. | **Contribution: Indexed Lattice Coordinates.** This step takes the raw Level 0 declarations from Step 3 and **maps them into the 8-dimensional conceptual space**. This creates a central index (e.g., TOML/JSON) linking constant names/paths to their canonical **8D coordinates**, moving beyond hardcoded values and formally defining their verifiable position within the lattice. |

---

### Phase III: Enabling Self-Reflection and Topological Ordering

Once Level 0 is indexed, the subsequent high-priority steps focus on establishing the system's ability to self-host and then decomposing the dependent code units (Level 1 and above).

| Step | Priority & Task (Component Added) | Rationale and Lattice Contribution |
| :--- | :--- | :--- |
| **5** | **Implement `reconstruct_ast_from_hf_dataset` (Task P3 / 05\_04)**. | **Contribution: The Self-Hosting Loop.** This implements the core bootstrap placeholder necessary to realize the goal of the **Self-Hosting Prelude Generator**. It allows the system to use its own structured, analyzed data (AST and UseStatement metadata saved to Hugging Face) to **generate a new version of itself**, ensuring reflexivity and data-driven improvement. |
| **6** | **Decompose codebase into granular components (Task 06\_01)**. | **Contribution: All Lattice Nodes.** This executes the large-scale splitting using `rust-decl-splitter` to break the rest of the monolithic files into **single declaration units**. This process creates the comprehensive set of individual **"nodes"** (Level 1+) needed for the full topological sort, where their dependencies (mapped via BoW in Step 2) rely on the foundational Level 0 nodes established in Step 3. |

This planned reconstruction ensures that architectural stability (Steps 1-2) is achieved before the fundamental data is defined (Steps 3-4), allowing the system to verify and utilize its newly generated structure (Step 5) before undertaking the full decomposition of all higher layers (Step 6).

***

**Analogy:** This plan is structured like building a massive, self-aware library. First, you must fix the **central control system** (Step 1) and implement the **cataloging standards** (Step 2: the 8D coordinate/4KB rule). Only then can you find and precisely label the **most atomic, fundamental axioms** (Steps 3-4: Level 0 Constants). Once those axioms are perfectly cataloged, the library learns to read and organize **its own catalog** (Step 5: Self-Reconstruction), finally allowing the automatic, rigorous breakdown of all complex papers and treatises that rely on those axioms (Step 6: Decomposition of all remaining Levels).
## Further Documentation

For more detailed information on specific aspects of the project, please refer to the following documents:

*   [`bootstrap.md`](./bootstrap.md): Outlines the high-level goal of creating a self-hosting `prelude-generator`.
*   [`functional.md`](./functional.md) and [`functional_arrow_best_practices.md`](./functional_arrow_best_practices.md): Provide insights into the functional programming principles guiding this project.
*   [`docs/memos/Shellcheck_Always_After_Changes.md`](./docs/memos/Shellcheck_Always_After_Changes.md): Guidelines on using `Shellcheck` for shell script quality.
*   [`qa_report.md`](./qa_report.md): Latest quality assurance report.
*   [`lattice.md`](./lattice.md), [`rollup.md`](./rollup.md): Detailed explanation of the architectural vision, including the Introspective Rollup Workflow and Lattice of Functions.

## Command Object Usage Reporting

To facilitate the refactoring of external command execution (currently relying on `std::process::Command`) towards static Rust trait implementations, a new reporting feature has been introduced. This report identifies all instances where the `Command` object is used within the analyzed codebase.

### How to Use

To generate the `Command` object usage report, simply run the following Makefile target from the project root:

```bash
make generate-command-usage-report
```

The report will be generated at `rust-system-composer/.gemini/generated/command_usage_report.txt`.

### Report Content

The generated report will list:
- Node ID: The identifier of the node in the code graph where `Command` usage was found.
- Expression: The full expression string where `Command` was used.
- Program Called: The name of the program being called (extracted using a heuristic).
- Classification: A heuristic classification of whether the program is "Local" or "External".

This report is a crucial step towards identifying and systematically replacing `std::process::Command` calls with more controlled and statically verifiable trait-based program invocations.

## Command Object Usage Reporting

To facilitate the refactoring of external command execution (currently relying on `std::process::Command`) towards static Rust trait implementations, a new reporting feature has been introduced in `rust-system-composer`. This report identifies all instances where the `Command` object is used within the analyzed codebase.

### How to Use

To generate the `Command` object usage report, run `rust-system-composer` with the `layered-compose` subcommand and provide the `--command-report-output-path` argument:

```bash
cargo run -p rust-system-composer -- layered-compose --command-report-output-path command_usage_report.txt
```

Replace `command_usage_report.txt` with your desired output file path.

### Report Content

The generated report will list:
- Nodes in the code graph that directly reference the `Command` type.
- Expression nodes where the expression string contains "Command".
- Edges in the code graph that connect to or from "Command" related nodes.

This report is a crucial step towards identifying and systematically replacing `std::process::Command` calls with more controlled and statically verifiable trait-based program invocations.
