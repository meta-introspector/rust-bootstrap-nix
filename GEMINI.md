## Gemini Added Memories
- For Nix flake inputs, always use the pattern `github:meta-introspector/(repo name)?ref=(branch name)&dir=(path within repo)`. The current repo is `time-2025`, and the current branch is `feature/lattice-30030-homedir`, but these can vary. The `dir` parameter should be inserted as needed.
- nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
- use git commit -F commit-message.txt instead of long commands
- In the 'nix2make2nix' system, each make target, each nix flake, and each content are all represented by emoji strings or primes.
- self.url ="github:meta-introspector/time-2025?ref=feature/aimyc-001-cleanbench"
- The user has an idea to create a continuation in the Nix REPL to drive LLM jobs from inside a Nix shell, allowing the LLM to generate and reflect on Nix expressions.
- To use the local flake registry, set the NIX_REGISTRY_CONFIG environment variable to the absolute path of the registry.json file: export NIX_REGISTRY_CONFIG=/data/data/com.termux.nix/files/home/pick-up-nix2/source/github/meta-introspector/streamofrandom/2025/registry.json
- In the context of AI Life Mycology, graphs are considered to be the 'knots' or 'Quasi-Fibers'.
- The project aims for an 8-fold recursion system: 8 stages of self-hosting Rust compilation, each improving the next, culminating in a smart contract system where Rust functions and the compiler are translated to blockchain data.

### Bootstrap Configuration Builder Updates

This section details recent developments in configuring the Rust bootstrap process, particularly focusing on the `bootstrap-config-builder` crate and its integration with Nix.

**Goal:** The primary objective is to enable the generation of `config.toml` files for the Rust bootstrap process, leveraging Nix store paths for `rustc` and `cargo`, and to facilitate systematic testing of different Rust toolchain versions from the Nix store. This is a foundational step towards an 8-fold recursion system for eBPF Rust bootstrapping.

**Key Changes and Learnings:**

1.  **`bootstrap-config-builder` Refactoring:**
    *   The `bootstrap-config-builder` crate was refactored to consolidate its `main` function into `src/lib.rs`, making it a library that can also be run as a binary. The redundant `src/main.rs` file was removed.
    *   A new binary target, `bootstrap-config-generator`, was added to `bootstrap-config-builder/Cargo.toml` to explicitly run the `main` function from `src/lib.rs`. This allows for execution via `cargo run --bin bootstrap-config-generator`.
    *   A type mismatch for the `dry_run` argument in `bootstrap-config-builder/src/config.rs` was resolved, ensuring correct handling of boolean flags.
    *   The `example.toml` template content was inlined directly into `bootstrap-config-builder/src/utils/format_file.rs`. This eliminates a file system dependency and potential path resolution issues, making the configuration generation process more robust.
    *   The `template_path` argument was removed from `bootstrap-config-builder/src/utils/construct_config_content.rs` as it is no longer needed.

2.  **Nix Integration for Rust Toolchain Paths:**
    *   We successfully obtained the Nix store paths for `rustc` and `cargo` using `nix-shell` and `which` commands. These paths are crucial for ensuring the Rust bootstrap process uses precisely defined and versioned compilers from the Nix store.
        *   `rustc`: `/nix/store/yxh9cs2lshqgk6h0kp256yms3w8qwmsz-rustc-wrapper-1.89.0/bin/rustc`
        *   `cargo`: `/nix/store/ahyjafkgyn6zji9qlvv92z8gxmcmaky4-cargo-1.89.0/bin/cargo`
    *   The `bootstrap-config-generator` can now be invoked with `--rustc-path` and `--cargo-path` arguments to inject these Nix store paths into the generated `config.toml`.

3.  **Rust Source Flake Path Handling:**
    *   The `bootstrap-config-generator` expects a *local path* to the Rust source flake (e.g., `/data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src`) via the `--rust-src-flake-path` argument. It validates this path by checking for the presence of `src/ci/channel` within it.
    *   This local path is essential for the Rust bootstrap process to access the compiler's source code (sysroot).

**How to Generate `config.toml`:**

To generate a `config.toml` with specific `rustc` and `cargo` paths, and a local Rust source path, navigate to the `bootstrap-config-builder` directory and run:

```bash
car go run --bin bootstrap-config-generator -- \
    --rustc-path /nix/store/yxh9cs2lshqgk6h0kp256yms3w8qwmsz-rustc-wrapper-1.89.0/bin/rustc \
    --cargo-path /nix/store/ahyjafkgyn6zji9qlvv92z8gxmcmaky4-cargo-1.89.0/bin/cargo \
    --project-root /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix \
    --rust-src-flake-path /data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src \
    --output generated_config.toml
```

This command will create a `generated_config.toml` file in the `bootstrap-config-builder` directory, containing the specified paths and other default configuration values.

**Next Steps:**

The generated `config.toml` now needs to be integrated into the actual Rust bootstrap process. This involves understanding how the main `rust-bootstrap-nix` project consumes its `config.toml` and adapting it to use the newly generated configuration.

### `test-openssl-sys` Crate Creation and Nixification

This section details the creation of a standalone Rust test crate (`test-openssl-sys`) that uses the `openssl-sys` library, and its successful integration and build within the Nix flake system.

**Goal:** To create a minimal Rust project demonstrating `openssl-sys` usage, configured to build reliably with Nix flakes, addressing common dependency resolution challenges in a Nix environment.

**Key Challenges and Solutions:**

1.  **`Cargo.lock` Management in a Workspace:**
    *   **Problem:** When `test-openssl-sys` was initially added to the main project's Cargo workspace, running `cargo generate-lockfile` from within its directory would update the workspace's `Cargo.lock` at the root, not create a local one.
    *   **Solution:** Temporarily removed `test-openssl-sys` from the workspace, added an empty `[workspace]` table to its `Cargo.toml` to make it a standalone package for `cargo generate-lockfile`, then re-added it to the main workspace. This allowed `test-openssl-sys` to manage its own `Cargo.lock`.

2.  **Nix Flake Input Resolution for Common Dependencies:**
    *   **Problem:** Initial attempts to import `nix/rust-deps/common-rust-deps.nix` into `test-openssl-sys/flake.nix` resulted in "access to absolute path ... is forbidden in pure eval mode" errors. This was due to incorrect referencing of the parent project's source.
    *   **Solution:** Defined a `commonDepsFlake` input in `test-openssl-sys/flake.nix` that points directly to the `nix/rust-deps` directory *as a flake* within the `rust-bootstrap-nix` repository (`github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=nix/rust-deps`). The `commonRustDeps` was then imported as `commonDepsFlake.common-rust-deps`.

3.  **`openssl-sys` Library Discovery in Nix Build Environment:**
    *   **Problem:** The `openssl-sys` crate consistently failed to find OpenSSL libraries during `nix build`, reporting `pkg-config` not found or incorrect library paths, even when `PKG_CONFIG_PATH` and `pkgs.pkg-config` were seemingly correctly configured. This was due to `pkgs.openssl` resolving to the `bin` package instead of the main `openssl` package containing shared libraries, and `openssl-sys` misinterpreting environment variables.
    *   **Solution:** Explicitly set `OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";` and `OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";` in `test-openssl-sys/flake.nix`. This precisely guided `openssl-sys` to the correct OpenSSL shared libraries and header files.

**How to Use `test-openssl-sys`:**

The `test-openssl-sys` crate has been successfully built. The executable is located at `result/bin/test-openssl-sys` after a successful `nix build`.

To run the built executable:
```bash
./result/bin/test-openssl-sys
```
This command will execute the Rust program, which should print the OpenSSL version, confirming successful linking against the OpenSSL libraries provided by Nix.

To build the `test-openssl-sys` flake:
```bash
*   **New SOP Creation:** Created `docs/sops/SOP_Digital_Mycology_Experiment_Workflow.md` to define the workflow for LLM-based science experiments.
    *   Documented `docs/Nix_and_Precommit_Setup.md` detailing the project's Nix and pre-commit configurations, including Git submodule management.
    *   Created `docs/sops/SOP_Nix_Github_Meta_Introspector_Policy.md` documenting the policy for Nix flake inputs from `github:meta-introspector` and branch-only references.
    *   Created `docs/Precommit_Nix_Submodule_Overview.md` providing a table of pre-commit hooks, Nix packages, and Git submodule information.
    *   Created `docs/Precommit_Nix_Submodule_Summary.md` providing a focused summary table of pre-commit hooks, their associated submodules, branch/revision, and Nixification status.
*   **`prelude-generator` Macro Expansion and Declaration Extraction Fixes:**
    *   Resolved issues in `prelude-generator/src/use_extractor/expand_macros_and_parse.rs` related to macro expansion failures and `temp_rs_file_name` references. The logic for extracting relevant expanded code was simplified.
    *   Corrected module import paths in `prelude-generator/src/main.rs` and `prelude-generator/src/lib.rs`.
    *   Refactored declaration visitor logic, including the deletion of `prelude-generator/src/level0_decls_visitor.rs` and the introduction of `prelude-generator/src/decls_visitor.rs`.
    *   Verified fixes by successfully running `bash standalonex/level0min.sh`, confirming correct Level 0 declaration extraction.
*   **`prelude-generator` Structured Error Collection:**
    *   Implemented a structured error collection mechanism in `prelude-generator` to collect errors during macro expansion and parsing instead of crashing.
    *   Introduced `ErrorSample` and `ErrorCollection` structs in `prelude-generator/src/error_collector.rs` to store detailed error information (file path, Rustc version, error message, code snippet, etc.).
    *   Modified `prelude-generator/src/use_extractor/expand_macros_and_parse.rs` to return `Result<(syn::File, Option<ErrorSample>)>`, allowing errors to be captured and returned as `ErrorSample` instances.
    *   Integrated `ErrorCollection` into `prelude-generator/src/declaration_processing.rs` and `prelude-generator/src/command_handlers.rs` to aggregate and output collected errors to `collected_errors.json`.
    *   Verified the functionality by creating a malformed Rust file (`standalonex/min_test_project/src/malformed.rs`) and confirming that the error was correctly collected and written to `collected_errors.json`.

### Prelude Generator Configuration and Binary Path Integration

*   **`config.toml` Updates:**
    *   The root `config.toml` was updated to include Nix store paths for `rustc` and `cargo`.
    *   The `[project_binaries]` section was refactored into a `[bins]` section, where project-specific binaries are now listed by name with their relative paths (e.g., `hf_validator = "target/release/hf-validator"`).
*   **`prelude-generator` Integration:**
    *   `prelude-generator/src/args.rs`: A new command-line argument `--config-file-path` was added to allow specifying the `config.toml` location.
    *   `prelude-generator/src/config_parser.rs`: A new module was created to define the `Config` struct (reflecting the `[bins]` section) and implement logic for reading and parsing `config.toml`.
    *   `prelude-generator/src/main.rs`: Modified to read the `config.toml` (if provided) and pass the parsed configuration to the `run_category_pipeline` function.
    *   `prelude-generator/src/category_pipeline.rs`: The `HuggingFaceValidatorFunctor` was updated to accept and utilize the `hf_validator_path` directly from the parsed `config.toml`, enabling `prelude-generator` to locate and execute the `hf-validator` binary.
*   **Local Build Environment Setup:**
    *   The root `flake.nix` was enhanced with a `devShell` that provides necessary OpenSSL environment variables (`OPENSSL_LIB_DIR`, `OPENSSL_INCLUDE_DIR`) to facilitate successful local Rust builds (`cargo build --workspace --release`). This resolved previous build failures related to `openssl-sys`.

### Expanded Macro Files Location

The expanded macro files, typically named `.expand_output_*.rs`, are generated by the `run-decl-splitter` target.

*   **Initial Generation:** These files are initially created in the `generated_declarations/` directory.
*   **Moved for Compilation:** For compilation within the `generated-declarations-lib` crate, these files are subsequently moved to the `generated-declarations-lib/src/` directory.

Therefore, when referencing these files for operations like `split-expanded-bin`, ensure you are pointing to their current location within `generated-declarations-lib/src/`.

## Next Steps:
```