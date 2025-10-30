# Current Development Plan

## Recent Progress:

1.  **`prelude-generator` Refactoring & Debugging:**
    *   Successfully refactored `prelude-generator` to use `pipeline-traits` for core data structures.
    *   Resolved `tokio` runtime panics and `hf-validator` execution issues.
    *   Implemented default `config.toml` loading and `--verify-config` argument.
    *   Successfully ran `prelude-generator` to generate Hugging Face datasets.

2.  **AST Statistics Generation:**
    *   Implemented statistical analysis of AST nodes from the generated Hugging Face dataset.
    *   Generated a Rust code file (`generated/ast_statistics.rs`) containing these statistics as a `static` `AstStatistics` struct.

3.  **`ast-stats-crate` Creation:**
    *   Created a new isolated crate (`generated/crates/ast-stats-crate`) to house the generated `AST_STATISTICS`.
    *   Configured `ast-stats-crate` to correctly depend on `once_cell` and `prelude-generator` (for `AstStatistics` definition).
    *   Successfully built `ast-stats-crate`.

## Next Steps:

1.  **Integrate `AST_STATISTICS` into `ClassifyUsesFunctor`:**
    *   **Goal:** Leverage the statistical data from `ast-stats-crate::AST_STATISTICS` to inform and improve the classification logic within `prelude-generator::src::prelude_category_pipeline::prelude_category_pipeline_impls::classify_uses_functor.rs`.
    *   **Details:**
        *   Import `AST_STATISTICS` into `classify_uses_functor.rs`.
        *   Develop logic to analyze each `use` statement (and potentially other AST elements) against the statistical profiles (e.g., common patterns, typical lengths, version information).
        *   Populate the `git_details`, `nix_details`, `rust_details`, `cargo_details`, `syn_details`, `llvm_details`, and `linux_details` fields of the `UseStatement` struct based on this informed analysis. This will be a complex task requiring careful design and implementation of classification rules.
        *   Consider using pattern matching, regular expressions, or even simple heuristics guided by the statistics to identify and extract relevant information for each detail type.

2.  **Refine `PreprocessFunctor` (if necessary):**
    *   Review `PreprocessFunctor` to see if any preprocessing steps can be optimized or informed by the `AST_STATISTICS`.

3.  **Utilize Generated Data for Self-Generation:**
    *   **Goal:** Begin implementing the "Self-Generation" aspect of the `prelude-generator`, as outlined in `bootstrap.md`.
    *   **Details:** This will involve processing the classified `UseStatement` data (which now includes rich `git_details`, `nix_details`, etc.) to generate new code, configurations, or documentation. The exact nature of this generation will depend on the specific goals of the "better parsing" and "self-hosting" objectives.

4.  **Address Remaining Warnings:**
    *   Resolve any outstanding compiler warnings, especially the `ParseFunctor` warning and the `hf-dataset-validator-rust` workspace warning.
