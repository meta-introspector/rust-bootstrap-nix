# Emergency Task Update Dump: Prelude Generator and Split Expanded Lib

## Current State Summary

This document serves as a reboot checkpoint, summarizing the current state of the `prelude-generator` and `split-expanded-lib` crates, our overarching goal, and the immediate compilation errors encountered.

### `split-expanded-lib` Crate

**Status:** Successfully rebuilt and updated.

**Key Changes:**
- Added `is_public: bool` field to the `Declaration` struct.
- Updated `Declaration::new` constructor to accept the `is_public` field.
- Updated `SerializableDeclaration` struct and its `From` implementation to include `is_public`.
- Modified `DeclsVisitor` methods (`visit_item_const`, `visit_item_struct`, `visit_item_enum`, `visit_item_fn`, `visit_item_static`, `visit_item_macro`, `visit_item_mod`, `visit_item_trait`, `visit_item_trait_alias`, `visit_item_type`, `visit_item_union`, `visit_item`) to correctly determine and set the `is_public` field based on item visibility.
- Added `PublicSymbol` struct to represent public declarations in a structured format.
- Modified `extract_declarations_from_single_file` to return `Vec<PublicSymbol>` in addition to existing return values, filtering for public symbols and converting them into `PublicSymbol` instances.

### `prelude-generator` Crate

**Status:** Currently facing compilation errors after updates to `split-expanded-lib`.

**Goal:** To process expanded Rust code, extract declarations, identify public symbols, and eventually build a partial lattice for each crate.

## Overarching Goal: Public Symbol Extraction and Partial Lattice Construction

Our primary objective is to:
1.  For each crate, create a partial lattice including all public symbols.
2.  Build a global set of public symbols as a partial lattice that rolls up.
3.  Enable modules to know the symbols of included crates.
4.  Resolve all symbols in our code by consulting the partial lattice.
5.  Identify and reduce duplicate code using external symbols.
6.  Construct a detailed lattice of `syn` usage.
7.  Generate LLVM for constants, slicing the compiler into testable shards.

## Immediate Compilation Errors and Planned Fixes

The `prelude-generator` crate is currently failing to compile due to type mismatches and missing field errors, primarily stemming from the changes made to `split-expanded-lib`'s return types and struct definitions.

### Errors in `prelude-generator/src/command_handlers/decl_splitter_handler.rs`

**1. `error[E0425]: cannot find value `rustc_info` in this scope` & `error[E0425]: cannot find value `project_root` in this scope`**
   - **Cause:** These arguments were removed from the `handle_run_decl_splitter` function signature in a previous step, but are still referenced within the function body.
   - **Planned Fix:** Re-add `project_root: &PathBuf` and `rustc_info: &crate::use_extractor::rustc_info::RustcInfo` to the signature of `handle_run_decl_splitter`.

**2. `error[E0609]: no field `crate_name` on type `&args::Args`**
   - **Cause:** The `Args` struct in `prelude-generator` does not have a `crate_name` field.
   - **Planned Fix:** Remove the line attempting to access `args.crate_name`. Instead, use the `current_crate_name` variable, which is correctly derived from `file_path.file_stem().unwrap().to_string_lossy().to_string()` within the loop.

**3. `error[E0599]: no method named `is_empty` found for enum `std::option::Option<T>` in the current scope` & `error[E0277]: the trait bound `&Vec<std::string::String>: Pattern` is not satisfied`**
   - **Cause:** `args.filter_names` is an `Option<Vec<String>>`, and the code attempts to call `is_empty()` directly on the `Option` or use `Vec<String>` as a `Pattern` in `contains()`.
   - **Planned Fix:** Correctly unwrap `args.filter_names` using `as_ref().map_or(true, |filter_names| { ... })` and iterate over the `filter_names` to check each one against `file_path.to_string_lossy().contains(f)`.

**4. `error[E0308]: mismatched types: expected `split_expanded_lib::RustcInfo`, found `rustc_info::RustcInfo`**
   - **Cause:** Type mismatch between `prelude-generator`'s `RustcInfo` and `split_expanded_lib`'s `RustcInfo`.
   - **Planned Fix:** Convert `prelude-generator`'s `RustcInfo` to `split_expanded_lib::RustcInfo` before passing it to `extract_declarations_from_single_file`.

### Errors in `prelude-generator/src/lib.rs`

**1. `error[E0308]: mismatched types: expected a tuple with 5 elements, found one with 4 elements`**
   - **Cause:** The call site for `extract_all_declarations_from_file` is expecting a 4-element tuple, but the function now returns a 5-element tuple (due to the addition of `public_symbols`).
   - **Planned Fix:** Update the destructuring assignment in `prelude-generator/src/lib.rs` to match the 5-element tuple return type.

## Next Steps (Reboot Checkpoint)

1.  **Fix `handle_run_decl_splitter` signature:** Re-add `project_root` and `rustc_info` to its signature.
2.  **Fix `main.rs` call site:** Update the call to `handle_run_decl_splitter` in `main.rs` to pass the re-added arguments.
3.  **Fix `crate_name` access:** Adjust `decl_splitter_handler.rs` to correctly use `current_crate_name`.
4.  **Fix `filter_names` logic:** Implement the correct unwrapping and iteration for `args.filter_names`.
5.  **Fix `RustcInfo` type mismatch:** Implement the conversion or ensure consistent `RustcInfo` usage.
6.  **Fix `lib.rs` destructuring:** Update the destructuring assignment in `prelude-generator/src/lib.rs`.
7.  **Rebuild and Verify:** Rebuild `prelude-generator` and re-run `level0min.sh`.