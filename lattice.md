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
5.  **Re-composition:** Assemble the transformed components into the final, topologically sorted, lattice-structured codebase, generating `Cargo.toml`, `flake.nix`, and `lib.rs` for each self-contained unit within the `generated/` directory.
6.  **Introspection and AI Analysis:** Apply the "Introspective Rollup Workflow" to key functions and components to gather performance metrics and generate AI-driven summaries and suggestions.

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

### `prelude-generator`'s Role in Lattice Construction (Bag of Words & Coordinate Grouping)

The `prelude-generator` plays a pivotal role in constructing the "Lattice of Functions" by implementing a "bag of words" approach for each declaration and grouping them into "coordinates" with module paths. This process is fundamental for understanding the dependencies and relationships between code components, which are the "nodes" and "edges" of our lattice.

**Key Contributions:**

*   **Declaration-Level Bag of Words:** The `DeclsVisitor` within `prelude-generator` is enhanced to collect a "bag of words" for each `syn::Item` (declaration). This includes:
    *   All referenced types (`syn::Ident`).
    *   All referenced functions (`syn::Ident`).
    *   Identified external identifiers (those not defined within the current file or project).
    *   This "bag of words" is stored alongside the declaration in a generic `Declaration` struct.
*   **Structured Declaration Representation:** The introduction of a `Declaration` struct provides a unified way to represent any `syn::Item` along with its associated "bag of words" and other metadata. This allows for consistent processing and analysis of all declaration types.
*   **Coordinate Grouping Logic:** Declarations will be grouped into "coordinates" based on their "bag of words." The grouping criteria will aim for ~4KB chunks (a single disk block) to optimize storage and retrieval. This grouping forms the basis for defining the "nodes" of the lattice.
*   **Module Path Generation:** Unique and descriptive module paths will be generated for each group of declarations (e.g., `prelude::group_hash_XXXX`), facilitating modular organization.
*   **Canonical Prelude Generation:** A main `prelude.rs` file will be created, containing `pub use` statements for all generated group modules, providing a canonical entry point for accessing these grouped declarations.
*   **Symbol Table Integration:** A symbol table will be established to register primitives and populate with modules (Rust standard library, project modules, external crates) as "lattices" or "gems." This baseline will aid in identifying new terms and types, crucial for understanding codebase evolution.

By implementing these features, `prelude-generator` directly contributes to the "Dependency Mapping" and "Canonicalization and Refactoring" steps of the high-level approach, providing the granular, dependency-aware building blocks necessary for the construction of the "Lattice of Functions."

## Use Statement Processing Pipeline

To further enhance the analysis and transformation capabilities of the system, a sophisticated, multi-stage pipeline for processing `use` statements has been implemented within the `prelude-generator`. This pipeline is designed to be robust, debuggable, and re-runnable, drawing inspiration from Makefile-like systems.

### Pipeline Stages

The pipeline is divided into the following stages:

1.  **Stage 1: Classify**
    *   **Input:** All Rust files in the project.
    *   **Process:** Extracts all unique `use` statements and attempts to parse them using `syn`.
    *   **Output:** A `stage_1_classify_output.toml` file containing a list of all `use` statements, classified as either:
        *   `ParsesDirectly`: The statement was successfully parsed by `syn`.
        *   `SynError`: The statement failed to parse with `syn`.

2.  **Stage 2: Preprocess**
    *   **Input:** The `stage_1_classify_output.toml` file.
    *   **Process:** For each `SynError` statement, this stage attempts to compile it in a temporary crate using `rustc`.
    *   **Output:** A `stage_2_preprocess_output.toml` file containing the results, with statements classified as either:
        *   `ParsesWithPreprocessing`: The statement compiled successfully with `rustc`.
        *   `FailsToCompile`: The statement failed to compile with `rustc`.

### Makefile-like System and Reporting

The pipeline's state and the results of each stage are managed through a series of TOML files, creating a Makefile-like system:

*   **`pipeline_state.toml`:** This file acts as the main state file for the pipeline. It contains a summary of each stage that has been run and a list of all the files that have been processed.
*   **Stage Output Files:** Each stage generates a TOML file (e.g., `stage_1_classify_output.toml`) that contains the detailed results of that stage's processing. This allows for easy inspection and debugging of each stage's output.

This system allows the pipeline to be re-run at any time. It will automatically pick up where it left off, only processing the stages and files that have not yet been processed.

### Batch Processing

To handle large codebases, the pipeline supports batch processing. The `--batch-size` command-line argument allows you to specify the number of files or statements to process in each run. The pipeline will keep track of the processed items and will automatically process the next batch on the subsequent run.

This combination of staged processing, detailed reporting, and batching makes the `use` statement processing pipeline a powerful and flexible tool for analyzing and transforming Rust code.

### Category-Theory-Based Pipeline

To further enhance the modularity and extensibility of the `use` statement processing pipeline, it has been refactored to use concepts from category theory. The pipeline is now modeled as a series of **functors** that map between different **categories** of code representation.

#### Categories of Code Representation

The pipeline defines the following categories, where each category represents a different stage of code processing:

*   `Category<RawFile>`: The objects are raw Rust files, represented as a tuple of `(file_path, content)`.
*   `Category<ParsedFile>`: The objects are parsed Abstract Syntax Trees (`syn::File`).
*   `Category<UseStatements>`: The objects are lists of `use` statements (strings).
*   `Category<ClassifiedUseStatements>`: The objects are the `UseStatement` structs, which now leverage a trait-based composition to hold detailed information about each `use` statement through optional fields like `git_details`, `nix_details`, `rust_details`, etc. These details are represented by specific traits (e.g., `GitInfo`, `NixInfo`) defined in the `pipeline-traits` crate.

#### Functors

Each step in the pipeline is implemented as a functor that maps between these categories:

*   **`ParseFunctor`**: `Category<RawFile> -> Category<ParsedFile>`
    *   This functor takes a `RawFile` and produces a `ParsedFile`. It handles the logic of trying a direct `syn` parse and falling back to macro expansion if necessary.
*   **`ExtractUsesFunctor`**: `Category<ParsedFile> -> Category<UseStatements>`
    *   This functor takes a `ParsedFile` and extracts all the `use` statements.
*   **`ClassifyUsesFunctor`**: `Category<UseStatements> -> Category<ClassifiedUseStatements>`
    *   This functor takes a list of `use` statements and classifies them, populating the initial trait-based detail fields within the `UseStatement` struct based on parsing success or failure.
*   **`PreprocessFunctor`**: `Category<ClassifiedUseStatements> -> Category<ClassifiedUseStatements>`
    *   This functor takes the `ClassifiedUseStatements`, refines the classification for statements that initially failed to parse, and further enriches the trait-based detail fields within the `UseStatement` struct by attempting compilation with `rustc`.

#### Pipeline Composition

The entire pipeline can be expressed as a composition of these functors. For example, to parse a file, extract its `use` statements, and classify them, you would compose the functors like this:

```rust
let parse_functor = ParseFunctor;
let extract_uses_functor = ExtractUsesFunctor;
let classify_uses_functor = ClassifyUsesFunctor;

let parsed_file = parse_functor.map(raw_file)?;
let use_statements = extract_uses_functor.map(parsed_file)?;
let classified_uses = classify_uses_functor.map(use_statements)?;
```

This approach makes the pipeline much more modular, extensible, and easier to reason about. Each functor is a self-contained unit of logic that can be tested independently, and new stages can be added to the pipeline by simply creating new functors and composing them.

The enhancement of the `UseStatement` struct with trait-based detail fields directly contributes to the "Lattice of Functions" goal. By categorizing and enriching the information associated with each `use` statement, we are effectively defining more granular "points" or "nodes" on the lattice. These detailed `UseStatement` nodes provide a richer understanding of the code's dependencies, allowing for more precise analysis, transformation, and ultimately, a more accurate and actionable representation of the codebase's underlying structure. This fine-grained data about each dependency type (Git, Nix, Rust, etc.) allows the system to build a more comprehensive and interconnected lattice, where each connection is not just a simple dependency, but a dependency with rich, contextual metadata.
