# High-Level Plan for `prelude-generator` and Refactoring

This document outlines the next steps for improving the `prelude-generator` and integrating it into the broader Rust project refactoring pipeline.

## 1. Generate `prelude-generator` Report
*   Process the `prelude_generator_output.txt` to identify:
    *   Successfully parsed files.
    *   Files with parsing warnings (e.g., "expected square brackets", "cannot parse string into token stream").
    *   Skipped files (e.g., procedural macro crates, explicitly excluded crates, files with no `use` statements).
*   Create a summary report (`prelude_generator_summary.md`) detailing these findings.

## 2. Address `prelude.rs` Circular Imports
*   **Problem:** The current `prelude-generator` output shows `pub use crate::prelude::*;` in the generated `prelude.rs` files, leading to circular imports. This occurs because existing `prelude.rs` files (from previous incorrect runs) are being parsed by `prelude-collector`.
*   **Action:** Implement a step to delete all existing `prelude.rs` files in the workspace *before* running `prelude-generator`. This will ensure that `prelude-collector` only processes original source files and generates correct `prelude.rs` content.

## 3. Refine `prelude-collector` for `use` Statement Flattening
*   **Goal:** Ensure that grouped `use` statements are correctly flattened into individual `use` statements.
    *   **Example:** `use syn::{parse_macro_input, Ident};` should become `use syn::parse_macro_input;` and `use syn::Ident;`.
*   **Action:** Verify the output of `prelude-generator` after addressing step 2. If flattening is not as expected, adjust the `flatten_use_tree` logic in `crates/prelude-collector/src/lib.rs`.

## 4. Investigate and Mitigate `syn` Parsing Errors
*   **Problem:** Many files within the `standalonex/src/bootstrap/` directory are failing to parse with `syn` (e.g., "expected square brackets", "cannot parse string into token stream"). This is likely due to heavy use of custom macros, generated code, or non-standard syntax.
*   **Action:**
    *   Analyze the specific files causing these errors.
    *   Determine if these files can be safely excluded from `prelude-generator` processing without impacting the overall refactoring goal.
    *   If exclusion is not feasible, explore more advanced parsing strategies (e.g., using `rustc -Zunpretty=expanded` to pre-process files, or identifying specific macro patterns to ignore within `prelude-collector`). Prioritize files critical for the `bootstrap` build.

## 5. Integrate `prelude-generator` into the Refactoring Pipeline
*   Once `prelude-generator` is reliably generating correct `prelude.rs` files and modifying source files as intended, integrate it as a foundational step in the automated refactoring pipeline.
*   This pipeline will eventually involve other tools like `rust-decl-splitter`, `rust-system-composer`, and `flake-template-generator`.

## 6. Address `bootstrap` Build Failures
*   After `prelude-generator` is working correctly and has modified the necessary files, re-attempt building the `bootstrap` crate.
*   Address any remaining compilation errors. These errors should now be related to actual type/resolution issues or other build-system configurations, rather than incorrect `prelude.rs` generation.
