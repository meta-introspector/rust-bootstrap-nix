## Functional Best Practices: Level 0 Declaration Extraction

This document outlines a functional approach to identifying and extracting "Level 0 Declarations" from Rust modules. Level 0 declarations are defined as atomic units of code that have no external dependencies within the project's scope, serving as the foundational layer for a dependency lattice. Initially, this focuses on `const` items due to their inherent atomicity.

### 1. Goal

The primary goal is to systematically extract all Level 0 declarations (starting with `const` items) from each Rust module within a given crate. These extracted declarations will be isolated into new, independently compilable files, which can then be integrated into the project's build system. This process aims to:
*   Establish a clear "foundation layer" for a dependency graph.
*   Promote modularity and reusability of atomic components.
*   Facilitate layered compilation and analysis.

### 2. Process Overview

The extraction process follows a series of distinct, functional steps:

1.  **Module Identification**: Discover all relevant Rust modules within the target crate.
2.  **Source Code Parsing**: Parse each module's source code into an Abstract Syntax Tree (AST).
3.  **Declaration Filtering**: Traverse the AST to identify and filter for Level 0 declarations (e.g., `const` items).
4.  **Code Regeneration**: Generate new Rust files containing only the extracted Level 0 declarations.
5.  **Build System Integration**: Update the crate's `lib.rs` (or `main.rs`) to include these new files as modules.
6.  **Validation**: Verify the compilability of the modified project.

### 3. Detailed Steps & Considerations

#### 3.1. Module Identification

*   **Technique**: Read the `src/lib.rs` (or `src/main.rs`) of the target crate. Parse its content to identify `pub mod` declarations. This provides a list of direct sub-modules. Recursively apply this process to discover nested modules.
*   **Tooling**: `syn` can be used to parse `lib.rs` and extract `syn::ItemMod` items.
*   **Considerations**: Handle conditional compilation (`#[cfg(...)]`) and re-exports (`pub use`).

#### 3.2. Source Code Parsing

*   **Technique**: For each identified module, read its corresponding `.rs` file. Use a Rust parser to convert the source text into a structured AST.
*   **Tooling**: `syn::parse_file` is the primary function for this.
*   **Considerations**: Error handling for unparseable files.

#### 3.3. Declaration Filtering (for Level 0)

*   **Technique**: Traverse the AST of each module. Identify `syn::Item` variants.
*   **Identifying `const` items**: Look for `syn::Item::Const`. These are inherently Level 0 as they cannot depend on runtime values or other non-const items.
*   **Identifying `static` items (Future Work)**: Similar to `const`, look for `syn::Item::Static`. Ensure they are initialized with constant expressions.
*   **Identifying Dependency-Free Functions (Future Work)**: This is significantly more complex. It would involve:
    *   Finding `syn::Item::Fn`.
    *   Analyzing the function body (`syn::Block`) for calls to other user-defined functions within the same crate.
    *   Checking parameter and return types for dependencies on non-primitive or non-standard library types.
    *   A simplified Level 0 function might be one that only uses primitive types and calls no other functions.
*   **Tooling**: `syn::visit::Visit` trait can be implemented for custom AST traversal.

#### 3.4. Code Regeneration

*   **Technique**: For each module, collect all identified Level 0 declarations. Use a code generation library to pretty-print these declarations into a new Rust file.
*   **File Naming**: A consistent naming convention, e.g., `src/module_name_constants.rs` or `src/module_name/constants.rs`.
*   **Tooling**: `quote!` macro for generating Rust code from `syn` items. `std::fs::write` for writing to files.
*   **Considerations**: Ensure proper formatting, imports, and visibility (`pub`).

#### 3.5. Build System Integration

*   **Technique**: Modify the `src/lib.rs` (or `src/main.rs`) of the crate to include the newly generated files as public modules.
*   **Example**: Add `pub mod module_name_constants;` to `lib.rs`.
*   **Tooling**: `syn` to parse `lib.rs`, `quote!` to modify it, `std::fs::write` to save.
*   **Considerations**: Avoid conflicts with existing module declarations. Ensure the new modules are accessible where needed. For larger projects, consider creating separate utility crates for these Level 0 declarations.

#### 3.6. Validation

*   **Technique**: After modifying the source code and adding new files, run the Rust compiler's check command.
*   **Tooling**: `cargo check` or `rustc --emit=metadata` on the entire crate.
*   **Considerations**: The validation should confirm that the new files compile correctly and that their integration into the main crate does not introduce new errors.

### 4. Challenges & Future Work

*   **Precise Dependency Analysis**: Accurately determining "no dependencies" for functions requires full semantic analysis, potentially involving a tool like `rust-analyzer` or a custom semantic analyzer.
*   **Macro Expansion**: Handling macros that generate declarations.
*   **Type Complexity**: Identifying simple vs. complex type dependencies for Level 0 classification.
*   **Automated Refactoring**: Developing a robust tool to automate the entire extraction and integration process, including handling existing code modifications.
*   **Lattice Construction**: Extending this to higher-level declarations that depend on Level 0 items.

### 5. Best Practices/Principles Applied

*   **Modularity**: Breaking down code into smaller, independently manageable units.
*   **Separation of Concerns**: Isolating atomic declarations from their usage context.
*   **Layered Architecture**: Building a clear hierarchy of dependencies.
*   **Functional Purity (for Level 0)**: Aiming for declarations that are side-effect free and self-contained.
*   **Reproducibility**: Ensuring the extraction and integration process is consistent and verifiable.

This abstract description provides a blueprint for the task, aligning with the functional best practices documentation style.
