# Outstanding Work for Nixification of Rust Bootstrap

This document outlines the remaining tasks to achieve the goal of having `x.py` emit Nix expressions for Rust compiler calls, effectively turning `x.py` into a Nix package generator.

## Goal

To generate a series of Nix flakes, each representing a single compiler invocation within the Rust bootstrap process, allowing the entire Rust build to be driven purely by Nix. The key is to *not* execute `rustc` or `cargo` directly from `x.py`, but instead to have `x.py` emit a Nix expression that represents that compiler call.

## Outstanding Tasks

1.  **Modify `bootstrap.py` to Emit Nix Expressions (High Priority)**
    *   **Intercept `run` Function:** The `run` function in `bootstrap.py` is responsible for executing external commands, including `rustc` and `cargo`. This function needs to be modified.
    *   **Replace Execution with Nix Expression Generation:** Instead of executing `subprocess.Popen(args, **kwargs)`, the `run` function should:
        *   Construct a JSON object containing the details of the intended command invocation:
            *   `command`: The executable (e.g., `cargo`, `rustc`).
            *   `args`: A list of arguments passed to the command.
            *   `env`: A dictionary of environment variables set for the command (consider filtering sensitive or irrelevant variables).
            *   `cwd`: The current working directory from which the command would have been executed.
            *   `type`: A string identifier, e.g., "rust_compiler_invocation", to categorize this emitted data.
        *   Print this JSON object to `stdout`.
        *   Exit successfully (`sys.exit(0)`) to prevent the actual execution of the Rust compiler and signal that the Nix expression has been emitted.
    *   **Define Nix Expression Schema:** Establish a clear, structured schema for the emitted JSON objects. This schema will be crucial for the Nix wrapper to correctly interpret the data.
        *   **Consider LLM Processing and Symbolic Representation:** The JSON schema should be designed to facilitate processing by Large Language Models (LLMs) and allow for symbolic representation (e.g., using emojis or primes as identifiers) for easier manipulation and reflection within a Nix REPL environment.

2.  **Modify `test-rust2/standalonex/flake.nix` `buildPhase` to Process Nix Expressions (High Priority)**
    *   **Run `python3 x.py build`:** Execute `x.py` (which will now emit JSON Nix expressions to `stdout`).
    *   **Capture Nix Expression Output:** Capture the `stdout` of `x.py` containing the emitted JSON objects.
    *   **Process and Generate Virtual Flakes:** Parse the captured JSON objects and dynamically generate new Nix flakes (or Nix attribute sets) for each compiler call. Each generated Nix flake should represent a single build step that, when evaluated, would execute the corresponding Rust compiler command with the specified arguments, environment, and working directory.
    *   **Organize Generated Flakes:** Store these generated flakes in a structured manner, e.g., `test-rust2/bootstrap/step001/flake.nix`, `test-rust2/bootstrap/step002/flake.nix`, etc., to represent the sequence of build steps.

3.  **Address `src/ci/channel` Panic (Likely Bypassed)**
    *   This issue, previously a critical blocker, will likely be bypassed by the new approach. Since `bootstrap.py` will no longer execute the Rust binary that panics on `src/ci/channel`, the direct cause of this panic will be removed.
    *   However, if `bootstrap.py` itself needs to read this file for its internal logic (e.g., to determine the channel for emitting the Nix expression), we might still need to ensure its accessibility or provide a default value.

4.  **Generalize `rust_root` and `build_dir` Handling (Medium Priority)**
    *   **Problem:** `x.py` (and `bootstrap.py`) expects to be run from the root of the Rust source tree, but needs to perform writable operations in a temporary directory.
    *   **Solution:** Ensure `bootstrap.py` correctly identifies and uses the read-only source root (`$src`) and the writable build directory (`$TMPDIR`). This might involve modifying `bootstrap.py` to accept parameters for these paths or to derive them robustly within the Nix build environment.

5.  **Clean Up `flake.nix` (Low Priority)**
    *   Remove all debug `echo` and `ls` commands from the `buildPhase` once the core functionality is working.
    *   Remove `RUST_BACKTRACE=full` once the panic is resolved (which should be the case with the new approach).

## Current Status

*   The `task.md` file has been updated with the new strategy.
*   The previous attempts to debug the `fs::read_to_string` panic in the Rust binary are now superseded by the goal of preventing its execution entirely.

## Next Immediate Step

Modify the `run` function in `test-rust2/standalonex/src/bootstrap/bootstrap.py` to emit JSON descriptions of compiler invocations instead of executing them. This is the foundational step for transforming `x.py` into a Nix expression generator. We will need to carefully consider the structure of the JSON output to make it easily consumable by Nix. We will also need to ensure that `sys.exit(0)` is called after printing the JSON to prevent further execution within `bootstrap.py`.