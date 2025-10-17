# Plan for `evaluate-rust` Flake

This document outlines the detailed plan for the `evaluate-rust` Nix flake, which will be responsible for taking a `commandInfo` (parsed JSON build step) and the Rust source code, and recursively generating Nix packages for each build target, integrating `naersk` for Rust-specific builds.

## Goal

To create a dynamic, recursive Nix build system that introspects the Rust bootstrap process, generating a "virtual Rust bootstrap introspector lattice of flakes" where each flake represents a build step and correctly models its dependencies.

## `evaluate-rust/flake.nix` Structure

### Inputs

*   `nixpkgs`: Standard Nixpkgs for basic packages and utilities.
*   `naersk`: For Rust-specific build logic and `cargo2nix` functionality.
*   `self`: To allow recursive calls to the flake's own library functions.

### Outputs

*   `lib`: A library attribute set containing functions for evaluating commands and generating packages.

## `lib.evaluateCommand` Function

This will be the core recursive function.

### Parameters

*   `commandInfo`: A Nix attribute set representing a single parsed JSON build step (e.g., `{ command = "rustc", args = ["--version"], ... }`).
*   `rustSrc`: The path to the Rust source code (a Nix path).
*   `currentDepth`: An integer representing the current recursion depth (initial call will be 0).
*   `maxDepth`: An integer representing the maximum recursion depth (e.g., 8).

### Logic

1.  **Base Case for Recursion:**
    *   If `currentDepth >= maxDepth`, return an empty list or a simple derivation indicating the recursion limit has been reached for this path.
    *   If `commandInfo` does not represent a build command that can be further broken down (e.g., it's a simple `rustc` invocation without `cargo`), create a simple `pkgs.runCommand` derivation for this step and return it in a list.

2.  **Analyze `commandInfo`:**
    *   **Identify `cargo build` commands:** Check `commandInfo.command` for "cargo" and `commandInfo.args` for "build".
    *   **If `cargo build`:**
        *   Use `naersk.lib.${system}.buildRustPackage` (or similar `rust2nix` functionality) to analyze the `Cargo.toml` within `rustSrc` (or a sub-path specified in `commandInfo.cwd`).
        *   Extract all build targets (binaries, libraries, tests, examples) from the `cargo build` command.
        *   For each extracted cargo target, create a new `commandInfo` object representing the build of that specific target.
        *   Recursively call `self.lib.evaluateCommand` for each of these new `commandInfo` objects, incrementing `currentDepth`.
        *   Combine the results (lists of derivations) from the recursive calls.
    *   **If other build commands (e.g., `rustc` directly):**
        *   Create a `pkgs.runCommand` derivation that executes the command specified in `commandInfo.command` with its `args` and `env` against `rustSrc`.
        *   Return this single derivation in a list.

3.  **Derivation Creation:**
    *   Each derivation should:
        *   Take `rustSrc` as its source.
        *   Set up the environment (`env` from `commandInfo`).
        *   Execute the command (`command` and `args` from `commandInfo`).
        *   Produce an output (e.g., a placeholder file, or the actual compiled artifact if possible).
        *   Have a descriptive name derived from `commandInfo` (e.g., `rustc-build-my-crate`).

## `json-processor/flake.nix` Integration

### Inputs

*   `evaluateRustFlake`: Input for the new `evaluate-rust` flake.

### Logic

1.  In the `builtins.map` loop that processes `parsedJsons`:
    *   For each `json` object (representing a `commandInfo`), call `evaluateRustFlake.lib.evaluateCommand` with `json`, `rustSrc`, `currentDepth = 0`, and `maxDepth = 8`.
    *   The result of `evaluateCommand` will be a list of derivations.
    *   Combine all these lists of derivations into a single flat list.
2.  The `packages.aarch64-linux` output will then be an attribute set where each attribute is one of these generated derivations, named appropriately. The `default` package will `symlinkJoin` all of them.

## Next Steps

1.  Create `evaluate-rust/flake.nix` with the basic structure and `lib.evaluateCommand` function.
2.  Implement the base cases and initial `pkgs.runCommand` for simple commands.
3.  Integrate `naersk` for `cargo build` commands and implement recursive calls.
4.  Modify `json-processor/flake.nix` to use `evaluate-rust`.
5.  Test the entire pipeline.
