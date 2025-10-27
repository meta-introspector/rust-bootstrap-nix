# Goal: Canonical Form and Lattice of Functions

The primary goal is to rewrite the existing Rust codebase into a "canonical form" represented as a "lattice of functions." This transformation will adhere to specific architectural constraints to enhance modularity, maintainability, and enable self-modifying capabilities.

## Key Constraints and Principles:

1.  **Topologically Sorted Structure:**
    *   The resulting codebase, when viewed as a dependency graph of modules and functions, must be topologically sorted. This implies a directed acyclic graph (DAG) where dependencies are processed and defined before their dependents. This ensures a clear and manageable flow of control and data.

2.  **One External Crate Per Module:**
    *   Each module in the transformed codebase should introduce at most one external crate dependency. This principle aims to:
        *   **Reduce Coupling:** Minimize interdependencies between modules and external libraries.
        *   **Improve Maintainability:** Make it easier to update or swap out external dependencies without affecting large portions of the codebase.
        *   **Enhance Testability:** Isolate external concerns, simplifying unit testing.
        *   **Facilitate Analysis:** Provide a clearer picture of where external functionalities are integrated.

3.  **Lattice of Functions:**
    *   The "lattice" refers to a structured representation of the codebase where individual functions (or other atomic declarations like structs, enums) are the nodes. The relationships between these nodes (e.g., function calls, data flow, type dependencies) form the edges of this lattice. This structure should be explicit and derivable from the code itself.

4.  **Self-Reading and Self-Rewriting:**
    *   The transformation process itself should be implemented as a meta-programming system capable of analyzing its own source code (or the target codebase) and generating the canonical, lattice-structured output. This implies a reflective or generative approach to code transformation.

## High-Level Approach:

1.  **Decomposition:** Utilize tools like `rust-decl-splitter` to break down the initial codebase into its most granular components.
2.  **Dependency Mapping:** Analyze these components to map out their internal and external dependencies, forming the basis of the "lattice."
3.  **Topological Ordering:** Apply topological sorting to establish a clear processing order based on dependencies.
4.  **Canonicalization and Refactoring:** Iteratively transform each component according to the "one external crate per module" rule and other canonical form guidelines (e.g., naming conventions, standardized interfaces).
5.  **Re-composition:** Assemble the transformed components into the final, topologically sorted, lattice-structured codebase.

This goal represents a significant step towards a highly modular, analyzable, and potentially self-optimizing Rust codebase.

## Key Components in the Lattice Transformation:

### `rust-system-composer` (`rust-system-composer/src/main.rs`)

This crate acts as the orchestrator for the entire lattice transformation process. Its primary responsibilities include:

*   **Pipeline Management:** Executes a sequence of tools, starting with `prelude-generator` and then `rust-decl-splitter`.
*   **Argument Handling:** Passes necessary configuration (e.g., `workspace-root`, `input-dir`, `output-dir`) to the underlying tools.
*   **Error Handling:** Provides centralized error reporting and ensures the pipeline halts if any sub-tool fails.

### `rust-decl-splitter` (`rust-decl-splitter/src/main.rs`)

This tool is fundamental to the "Decomposition" step of the lattice transformation. Its core function is to break down monolithic Rust source files into individual files, each containing a single declaration (function, struct, enum, trait, or `impl` block). This fine-grained decomposition is crucial for:

*   **Granularity:** Creating the individual "nodes" of the "lattice of functions."
*   **Modularity:** Enabling independent analysis and transformation of each declaration.
*   **Re-composition:** Providing the building blocks for re-assembling the codebase in a canonical, topologically sorted manner.

### `prelude-generator` (`prelude-generator/src/main.rs`)

This crate is responsible for generating prelude files, which are essential for the "Self-Reading" aspect of the transformation. It leverages `prelude-collector` to:

*   **Macro Expansion:** Expands all macros in the source code, providing a complete and unambiguous AST for analysis.
*   **AST Generation:** Parses the expanded code into an Abstract Syntax Tree, which is the foundation for understanding code structure and dependencies.
*   **Environment Awareness:** Gathers `rustc` and environment information to ensure accurate code analysis.

By providing a comprehensive and expanded view of the code, `prelude-generator` enables subsequent tools to perform accurate dependency analysis and apply transformation rules effectively.
