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

5.  **`rust-system-composer` Refactoring for Per-File Project Generation:**
    *   Refactored `rust-system-composer` to act as an orchestrator for generating self-contained Rust projects from expanded `.rs` files.
    *   It now iterates through input `.rs` files, creates a new project directory for each, generates a `Cargo.toml` (including `[workspace]` and `split-expanded-lib` dependency), calls `split_expanded_lib` to extract declarations, writes these declarations to the project's `src/` directory, generates `src/lib.rs` with `pub mod` statements, and updates `Cargo.toml` for `proc-macro = true` if necessary.
    *   Successfully compiled `rust-system-composer` after addressing `Cargo.toml` content generation and `use` statement issues.
    *   Successfully generated numerous per-file projects.
    *   Successfully compiled a sample generated project (`Declaration_project`), confirming the correctness of the generated `Cargo.toml`, `lib.rs`, and extracted declaration files.

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

## Next Steps:

1.  **Integrate `flake-template-generator` as a library:**
    *   **Goal:** Generate a `flake.nix` for each project using `flake-template-generator`'s library functionality.
    *   **Actions:**
        *   Add `flake-template-generator` as a dependency to `rust-system-composer/Cargo.toml`.
        *   Identify the relevant public functions in `flake-template-generator` for generating a `flake.nix`.
        *   Modify `rust-system-composer/src/main.rs` to call these library functions after the Rust project structure is in place. This will involve passing parameters like `project_dir`, `project_name`, and potentially other Nix-specific configurations.

2.  **Define Monadic I/O Traits and Arrow Composition:**
    *   **Goal:** Create a new crate (e.g., `monadic-io-traits`) that defines generic traits for monadic I/O operations and arrow composition, drawing inspiration from `functional_arrow_best_practices.md`.
    *   **Actions:**
        *   Create a new library crate `monadic-io-traits` in the workspace.
        *   Define traits like `Monad`, `Applicative`, `Functor`, `Arrow`, `Category` (if not already present in `pipeline-traits` or similar).
        *   Define associated types for input, output, and error handling.
        *   Implement basic combinators for these traits.
        *   Add `monadic-io-traits` as a dependency to `rust-system-composer` and potentially to the generated projects if they need to implement these traits.

3.  **Refactor `rust-system-composer` to use Monadic I/O:**
    *   **Goal:** Modify `rust-system-composer`'s I/O operations (file reading, writing, external command execution) to use the newly defined monadic I/O traits.
    *   **Actions:**
        *   Identify I/O-bound operations within `rust-system-composer`.
        *   Wrap these operations in types that implement the `Monad` or `Arrow` traits.
        *   Use combinators to compose these I/O operations. This will make `rust-system-composer` itself more robust and functional.

4.  **Generate Projects with Monadic I/O Integration:**
    *   **Goal:** Ensure that the generated projects can easily integrate with the monadic I/O framework.
    *   **Actions:**
        *   Modify the `Cargo.toml` generation in `rust-system-composer` to include `monadic-io-traits` as a dependency for generated projects.
        *   Potentially generate boilerplate code in `lib.rs` or other files within the generated projects to facilitate the use of monadic I/O for their specific functionalities.

5.  **Refine `split-expanded-lib`'s `get_identifier`:**
    *   Ensure that `Declaration::get_identifier()` correctly handles all cases, especially those with problematic characters or keywords, to produce valid Rust module names.

6.  **Error Handling and Reporting:**
    *   Improve error handling and reporting in `rust-system-composer`, especially for issues during file processing or project generation.

7.  **Implement Bag of Words and Coordinate Grouping for Declarations:**
    *   **Goal:** Enhance `prelude-generator` to collect a "bag of words" (referenced types, functions, external identifiers) for each declaration and group these declarations into "coordinates" (modules) aiming for ~4KB chunks.
    *   **Actions:**
        *   Enhanced `DeclsVisitor` to collect referenced types, functions, and external identifiers.
        *   Introduced a `Declaration` struct to unify declaration representation with associated "bag of words."
        *   Implement grouping logic to create ~4KB chunks based on declaration dependencies.
        *   Generate unique module paths for each group.
        *   Generate a "Canonical Prelude" file with `pub use` statements for all generated group modules.
        *   Establish a symbol table for primitives and module-based "lattices" to identify new terms.

8.  **Refactor `prelude-generator/src/main.rs`**:
    *   **Action:** The constant extraction calls have been integrated into `prelude-generator/src/main.rs`.
    *   **Action:** Split the `main.rs` file into separate functions for processing structs and constants to improve modularity and readability.
    *   **Action:** Ensure all internal calls within the extracted functions and the `main` function use the `prelude_generator::` prefix where necessary.

9.  **Refine hierarchical directory structure for constants**:
    *   **Goal:** Enhance the logic within the conceptual `write_numerical_constants_to_hierarchical_structure` and `write_string_constants_to_hierarchical_structure` functions (as documented in Markdown) to create actual sorted, hierarchical directories based on constant properties (e.g., hash, value prefixes), and ensure 4KB file blocking.

10. **Implement global index and 8D embedding for all constants**:
    *   **Goal:** Develop a mechanism to create a central index (e.g., TOML/JSON) mapping constant names/paths to their 8D coordinates and other metadata, moving beyond the hardcoded 0 for string constants.

11. **Generate TOML report for Bag of Words**:
    *   **Goal:** Implement the functionality to write the filtered bag of words to a TOML file, including the specified stop words.

12. **Refine `ClassifyUsesFunctor` (after `rust-system-composer` is functional)**:
    *   **Goal:** Leverage the statistical data from `ast-stats-crate::AST_STATISTICS` to inform and improve the classification logic within `prelude-generator::src::prelude_category_pipeline::prelude_category_pipeline_impls::classify_uses_functor.rs`.
    *   **Details:**
        *   Develop more sophisticated logic to analyze each `use` statement (and potentially other AST elements) against the statistical profiles (e.g., common patterns, typical lengths, version information).
        *   Populate the `git_details`, `nix_details`, `rust_details`, `cargo_details`, `syn_details`, `llvm_details`, and `linux_details` fields of the `UseStatement` struct based on this informed analysis.

13. **Refine `PreprocessFunctor` (if necessary)**:
    *   Review `PreprocessFunctor` to see if any preprocessing steps can be optimized or informed by the `AST_STATISTICS`.

14. **Utilize Generated Data for Self-Generation**:
    *   **Goal:** Begin implementing the "Self-Generation" aspect of the `prelude-generator`, as outlined in `bootstrap.md`.
    *   **Details:** This will involve processing the classified `UseStatement` data (which now includes rich `git_details`, `nix_details`, etc.) to generate new code, configurations, or documentation. The exact nature of this generation will depend on the specific goals of the "better parsing" and "self-hosting" objectives.
