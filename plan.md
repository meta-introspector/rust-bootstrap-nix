# Current Development Plan

## Recent Progress:

1.  **`prelude-generator` Refactoring & Debugging:**
    *   Successfully refactored `prelude-generator` to use `pipeline-traits` for core data structures.
    *   Resolved `tokio` runtime panics and `hf-validator` execution issues, including the "file name too long" error by using hash-based short IDs for temporary project directories.
    *   Implemented default `config.toml` loading and `--verify-config` argument.
    *   Successfully ran `prelude-generator` to generate Hugging Face datasets.
    *   Created `generated/hf_dataset_output/mapping.toml` to map original file paths to their generated short IDs.
    *   Improved the formatting of the AST Node Type Report for better readability.

2.  **AST Statistics Generation:**
    *   Implemented statistical analysis of AST nodes from the generated Hugging Face dataset.
    *   Generated a Rust code file (`generated/ast_statistics.rs`) containing these statistics as a `static` `AstStatistics` struct.

3.  **`ast-stats-crate` Creation:**
    *   Created a new isolated crate (`generated/crates/ast-stats-crate`) to house the generated `AST_STATISTICS`.
    *   Configured `ast-stats-crate` to correctly depend on `once_cell` and `pipeline-traits`.
    *   Successfully built `ast-stats-crate`.

4.  **`pipeline-traits` Refactoring for `Info` Variants:**
    *   Refactored all `_info.rs` files in `pipeline-traits/src/use_statement_types/` to define `Details` enums with `Info` variants and corresponding `Info` structs.
    *   Updated `pipeline-traits/src/use_statement_types/mod.rs` and `pipeline-traits/src/lib.rs` to correctly re-export the new traits.
    *   Updated `prelude-generator/src/prelude_category_pipeline/prelude_category_pipeline_impls/classify_uses_functor.rs` to use the new enum and struct fields with a basic heuristic for classification.

5.  **`rust-system-composer` Modification for Batch Processing:**
    *   Modified `rust-system-composer/src/main.rs` to discover all Rust files within a specified `workspace_root` and invoke `prelude-generator` for each.
    *   Updated `rust-system-composer/Cargo.toml` to include the `walkdir` dependency.

6.  **Resolved Compilation Errors and Warnings:**
    *   Fixed cyclic dependency issues between `prelude-generator` and `ast-stats-crate`.
    *   Addressed all compilation errors and warnings, including unused imports and dead code.

7.  **Constant Extraction and Documentation:**
    *   Created `functional_numerical_constants_storage.md` and `functional_string_constants_storage.md` to document the functional objects for numerical and string constant storage.
    *   Removed the initial helper function implementations for writing numerical and string constants from `prelude-generator/src/main.rs`, moving their conceptual documentation to the new Markdown files.
    *   Updated `functional.md` to include links to the new constant storage documentation.

8.  **Declaration Grouping and "Gem" Implementation:**
    *   Began implementation of the "bag of words" and "coordinate grouping" functionality in `prelude-generator`.
    *   Modified `prelude-generator/src/decls_visitor.rs` and related files to support declaration analysis.
    *   Introduced `prelude-generator/gems.toml` to define primitive modules (or "gems") for the code lattice.
    *   Added `prelude-generator/src/gem_parser.rs` to parse the `gems.toml` configuration.
    *   Added and modified test files and scripts in `standalonex/` to validate the new functionality, producing artifacts like `standalonex/generated_min_decls/` and `standalonex/min_test_project/collected_errors.json`.

## Current Blocking Issue:

*   **Locating `rust-system-composer`'s `main` function:** The `rust-system-composer/src/main.rs` file consistently returns re-exports, and attempts to locate the actual `fn main()` function have been unsuccessful. This is preventing further progress on using `rust-system-composer` to process all code into Parquet.

## Next Steps:

1.  **Implement Bag of Words and Coordinate Grouping for Declarations:**
    *   **Goal:** Enhance `prelude-generator` to collect a "bag of words" (referenced types, functions, external identifiers) for each declaration and group these declarations into "coordinates" (modules) aiming for ~4KB chunks.
    *   **Actions:**
        *   Enhanced `DeclsVisitor` to collect referenced types, functions, and external identifiers.
        *   Introduced a `Declaration` struct to unify declaration representation with associated "bag of words."
        *   Implement grouping logic to create ~4KB chunks based on declaration dependencies.
        *   Generate unique module paths for each group.
        *   Generate a "Canonical Prelude" file with `pub use` statements for all generated group modules.
        *   Establish a symbol table for primitives and module-based "lattices" to identify new terms.

2.  **Refactor `prelude-generator/src/main.rs`**:
    *   **Action:** The constant extraction calls have been integrated into `prelude-generator/src/main.rs`.
    *   **Action:** Split the `main.rs` file into separate functions for processing structs and constants to improve modularity and readability.
    *   **Action:** Ensure all internal calls within the extracted functions and the `main` function use the `prelude_generator::` prefix where necessary.

2.  **Refine hierarchical directory structure for constants**:
    *   **Goal:** Enhance the logic within the conceptual `write_numerical_constants_to_hierarchical_structure` and `write_string_constants_to_hierarchical_structure` functions (as documented in Markdown) to create actual sorted, hierarchical directories based on constant properties (e.g., hash, value prefixes), and ensure 4KB file blocking.

3.  **Implement global index and 8D embedding for all constants**:
    *   **Goal:** Develop a mechanism to create a central index (e.g., TOML/JSON) mapping constant names/paths to their 8D coordinates and other metadata, moving beyond the hardcoded 0 for string constants.

4.  **Generate TOML report for Bag of Words**:
    *   **Goal:** Implement the functionality to write the filtered bag of words to a TOML file, including the specified stop words.

5.  **Resolve `rust-system-composer` `main` function location:**
    *   **Action:** User to provide the exact path to the file containing the `fn main()` function for the `rust-system-composer` crate.
    *   **Goal:** Once located, continue with the integration of `prelude-generator` for batch processing of all Rust files.

6.  **Refine `ClassifyUsesFunctor` (after `rust-system-composer` is functional)**:
    *   **Goal:** Leverage the statistical data from `ast-stats-crate::AST_STATISTICS` to inform and improve the classification logic within `prelude-generator::src::prelude_category_pipeline::prelude_category_pipeline_impls::classify_uses_functor.rs`.
    *   **Details:**
        *   Develop more sophisticated logic to analyze each `use` statement (and potentially other AST elements) against the statistical profiles (e.g., common patterns, typical lengths, version information).
        *   Populate the `git_details`, `nix_details`, `rust_details`, `cargo_details`, `syn_details`, `llvm_details`, and `linux_details` fields of the `UseStatement` struct based on this informed analysis.

7.  **Refine `PreprocessFunctor` (if necessary)**:
    *   Review `PreprocessFunctor` to see if any preprocessing steps can be optimized or informed by the `AST_STATISTICS`.

8.  **Utilize Generated Data for Self-Generation**:
    *   **Goal:** Begin implementing the "Self-Generation" aspect of the `prelude-generator`, as outlined in `bootstrap.md`.
    *   **Details:** This will involve processing the classified `UseStatement` data (which now includes rich `git_details`, `nix_details`, etc.) to generate new code, configurations, or documentation. The exact nature of this generation will depend on the specific goals of the "better parsing" and "self-hosting" objectives.