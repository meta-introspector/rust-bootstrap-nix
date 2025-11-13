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
*   `Category<ClassifiedUseStatements>`: The objects are the `UseStatement` structs, which contain the `use` statement and an optional error.

#### Functors

Each step in the pipeline is implemented as a functor that maps between these categories:

*   **`ParseFunctor`**: `Category<RawFile> -> Category<ParsedFile>`
    *   This functor takes a `RawFile` and produces a `ParsedFile`. It handles the logic of trying a direct `syn` parse and falling back to macro expansion if necessary.
*   **`ExtractUsesFunctor`**: `Category<ParsedFile> -> Category<UseStatements>`
    *   This functor takes a `ParsedFile` and extracts all the `use` statements.
*   **`ClassifyUsesFunctor`**: `Category<UseStatements> -> Category<ClassifiedUseStatements>`
    *   This functor takes a list of `use` statements and classifies them as either `ParsesDirectly` or `SynError`.
*   **`PreprocessFunctor`**: `Category<ClassifiedUseStatements> -> Category<ClassifiedUseStatements>`
    *   This functor takes the `ClassifiedUseStatements`, finds the `SynError`s, and tries to compile them with `rustc`, updating their classification.

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

# Rust-Driven Nix Flake Generation: Building a Flake Lattice

This document outlines the plan to dynamically generate Nix flakes using Rust, starting with a core configuration and building up a "flake lattice" incrementally. The goal is to leverage Rust for managing and templating Nix configurations, enabling a more programmatic and verifiable approach to Nix flake development.

## Overall Goal

To create a system where Rust programs generate and manage Nix flake definitions, allowing for:
*   Dynamic configuration of Nix builds.
*   Programmatic generation of Nix expressions.
*   A "flake lattice" where each flake adds a feature or dependency, building upon previous ones.

### The `generated/` Directory: Output for the Flake Lattice

The `generated/` directory will serve as the primary output location for the dynamically generated components of the flake lattice. For each individual function or declaration that is transformed into a self-contained unit (as part of the "lattice of functions" goal), the `generated/` directory will contain:

*   `Cargo.toml`: The Rust package manifest for the generated library.
*   `flake.nix`: The Nix flake definition for building and integrating this specific generated library.
*   `lib.rs`: The Rust source code for the generated function or declaration, encapsulated as a library.

This approach ensures that each node in our "flake lattice" is a fully functional, reproducible, and independently manageable unit, built and integrated via Nix.

## Immediate Focus: Generating a Seed Config Flake

Our immediate goal is to successfully generate a minimal, functional Nix flake directory using Rust. This flake will contain a `flake.nix` generated from a Rust template and will expose a `config.toml` file (generated by `bootstrap-config-builder`) as a Nix package. This will serve as our "seed config" flake.

## Detailed Plan: Small, Verifiable Steps

### Phase 1: Ensure `bootstrap-config-builder` can generate `config.toml`

This phase verifies that our Rust component responsible for generating the core configuration is working correctly in isolation.

1.  **Verify `bootstrap-config-builder` output:**
    *   **Action:** Navigate to the `bootstrap-config-builder` directory.
    *   **Command:** `cargo run --bin bootstrap-config-generator -- --output generated_config.toml`
    *   **Verification:** Confirm that `generated_config.toml` is created in the `bootstrap-config-builder` directory and contains the expected TOML content.
    *   **Debugging:** If this fails, debug errors within `bootstrap-config-builder/src/` (Rust code) until a valid `config.toml` is produced.

### Phase 2: Create a New Rust Crate for Generating `flake.nix` from a Template

This phase involves building the Rust component that will take our generated `config.toml` and embed it into a dynamically created `flake.nix`.

1.  **Create a new Rust crate:**
    *   **Action:** Create a new Rust project (e.g., `flake-template-generator`) within the main project's `vendor/rust/` directory.
    *   **Purpose:** This crate will be responsible for:
        *   Reading the `config.toml` generated in Phase 1.
        *   Reading a predefined `flake.nix` template.
        *   Substituting placeholders in the template with values from `config.toml` or other dynamic data.
        *   Writing the resulting `flake.nix` to a specified output directory.
2.  **Define a basic `flake.nix` template:**
    *   **Action:** Create a minimal `flake.nix` template file (e.g., `flake_template.nix`) within the `flake-template-generator` crate's resources.
    *   **Content (Example):**
        ```nix
        {
          description = "Dynamically generated config flake";

          outputs = { self, nixpkgs }:
            let
              pkgs = import nixpkgs { system = "aarch64-linux"; }; # Assuming aarch64-linux for now
              configTomlContent = builtins.readFile ./config.toml;
            in
            {
              packages.aarch64-linux.default = pkgs.runCommand "generated-config-toml" { } ''
                mkdir -p $out
                echo "${configTomlContent}" > $out/config.toml
              '';
              # Add other outputs as needed, e.g., devShells
            };
        }
        ```
    *   **Placeholders:** The template might include placeholders for system architecture, flake inputs, or other dynamic values that the Rust generator will fill in. For this initial step, we'll keep it simple.

### Phase 3: Integrate and Test the New Flake Generation

This phase executes the Rust generator and verifies that the dynamically created flake is valid and functional.

1.  **Run `flake-template-generator`:**
    *   **Action:** Execute the `flake-template-generator` Rust binary.
    *   **Command (Example):** `cargo run --bin flake-template-generator -- --config-path ../bootstrap-config-builder/generated_config.toml --output-dir target/generated-flake`
    *   **Verification:** Confirm that a new directory (e.g., `target/generated-flake`) is created, containing a `flake.nix` and a `config.toml`.
2.  **Build the generated flake:**
    *   **Action:** Navigate to the newly generated flake directory (e.g., `target/generated-flake`).
    *   **Command:** `nix build .#default` (assuming the template exposes the config as the default package).
    *   **Verification:** Confirm that the build succeeds and the output (`result`) contains the `config.toml` file.
    *   **Debugging:** If this fails, debug the generated `flake.nix` and the `flake-template-generator` Rust code.

### Phase 4: Integrate the Generated Flake into the Root Project

Once the seed config flake can be reliably generated and built, we will integrate it into the main project's root `flake.nix`.

1.  **Add the generated flake as an input to the root `flake.nix`:**
    *   **Action:** In the root `flake.nix`, add an input pointing to the dynamically generated flake directory (e.g., `configFlake.url = "./target/generated-flake";`).
2.  **Consume the generated config:**
    *   **Action:** Modify the root `flake.nix` to consume the `config.toml` from `configFlake.packages.${pkgs.system}.default` (or whatever the output is named).
    *   **Purpose:** This will replace the direct dependency on `bootstrap-config-builder` for the `config.toml` content.
3.  **Run `nix build .#default` (root):**
    *   **Verification:** Confirm that the main project can now build using the dynamically generated config.

This structured approach ensures that each component is tested and verified before integration, making the debugging process much more manageable.

# rust-bootstrap-nix

This repository provides a Nix-based development and build environment for Rust projects, with a focus on integrating `sccache` for accelerated compilation and managing the `x.py` build system. It includes various Nix flakes for environment setup, JSON output processing, and build command evaluation, alongside shell scripts for debugging, development, and testing.

## Key Features

*   **Reproducible Development Environments:** Utilizes Nix flakes to define consistent Python and Rust development shells.
*   **`sccache` Integration:** Accelerates Rust compilation through `sccache` caching.
*   **`x.py` Build System Support:** Provides tools and environments for working with the `x.py` build orchestration script.
*   **JSON Output Processing:** Includes flakes for capturing and analyzing JSON metadata generated by the build process.

## Dynamic Nix-based Configuration

This project now features a dynamic, Nix-based configuration system that allows for precise control over the Rust bootstrap process. This system is designed to create a "lattice of Nix flakes," enabling reproducible builds and making it easy to experiment with different versions of Rust and its dependencies.

### How it Works

The core of this system is the `generated_config.toml` file, which is generated by the `bootstrap-config-builder` utility. This file contains the exact Nix store paths for all the tools and dependencies required for the build, including `rustc`, `cargo`, `nixpkgs`, `rust-overlay`, and the Rust source code itself.

The Rust bootstrap process has been modified to read this `generated_config.toml` file and use the paths within it to configure the build. If the `generated_config.toml` file is not present, the bootstrap process will dynamically fetch the required Nix store paths using `nix flake prefetch` and `nix path-info`.

### Usage

To use this new system, you can either:

1.  **Generate `generated_config.toml` manually:** Run the `bootstrap-config-builder` with the desired `--rustc-path`, `--cargo-path`, and `--rust-src-flake-path` arguments.
    *   **Using Makefile target:** You can use the `generate-seed-config` Makefile target to generate the `generated_config.toml` in the `bootstrap-config-builder/` directory.
        ```bash
        make generate-seed-config
        ```
2.  **Generate the flake directory:** Use the `generate-flake-dir` Makefile target to create the `target/generated-flake` directory containing the dynamically generated `flake.nix` and `config.toml`.
    ```bash
    make generate-flake-dir
    ```
3.  **Let the bootstrap process handle it:** If `generated_config.toml` is not present, the bootstrap process will automatically resolve the Nix paths for you.

For more detailed information, please refer to the `docs/Nix_Integration.md` file.

## Building the Standalone Bootstrap

To build the standalone Rust bootstrap environment, which is particularly useful for "Nix on Droid" (aarch64-linux) environments, use the following Nix command:

```bash
nix build ./standalonex#packages.aarch64-linux.default
```

This command will build the default package defined within the `standalonex/flake.nix` for the `aarch64-linux` architecture.

## Repository Overview

# Repository Overview: `rust-bootstrap-nix`

This repository serves as a comprehensive Nix-based environment for developing, building, and testing Rust projects, with a particular focus on integrating `sccache` for build acceleration and leveraging a custom `x.py` build orchestration system. It is designed to provide reproducible build environments across different architectures (`aarch64-linux` and `x86_64-linux`).

## Core Purpose

The primary goal of `rust-bootstrap-nix` is to streamline the Rust development workflow within a Nix ecosystem. This involves:

1.  **Reproducible Toolchains:** Providing consistent and isolated Rust compiler and Cargo toolchains via Nix flakes.
2.  **Build Acceleration:** Integrating `sccache` to significantly speed up Rust compilation times.
3.  **Custom Build Orchestration:** Utilizing a Python-based `x.py` script for managing complex build processes, including dependency handling and build step execution.
4.  **Build Metadata Extraction:** Generating and processing structured JSON output from the build process for analysis and further automation.
5.  **Modular Flake Structure:** Breaking down the environment and build logic into smaller, interconnected Nix flakes for better organization and reusability.

## Key Components

The repository is structured around several key components:

*   **Nix Flakes:** A collection of `flake.nix` files that define development environments, packages, and build logic. These include the root flake, sub-flakes for JSON processing, Rust evaluation, and a standalone `x.py` environment.
*   **Shell Scripts:** Various `.sh` scripts for common tasks such as entering development shells, debugging builds, diagnosing environment issues, and updating flakes.
*   **Configuration Files:** `config.toml` files that specify build settings, toolchain paths, and vendoring options.
*   **`standalonex/` Directory:** A critical component containing the `x.py` build orchestration script, Python utilities (`test_json_output.py`, `wrap_rust.py`), and Rust source code (`src/`). This directory is central to how the Rust project is built and how build metadata is generated.
*   **`src/bootstrap/bootstrap.py`:** The core Python script within `standalonex/src/bootstrap/` that implements the detailed logic for the Rust build process, including toolchain management, environment setup, and JSON output generation.

## How it Works (High-Level)

The system leverages Nix flakes to define a hermetic build environment. The root `flake.nix` sets up a development shell with Python, Rust, and `sccache`. The `x.py` script (located in `standalonex/`) acts as the primary interface for building the Rust project. During the build, `x.py` (specifically through its `bootstrap` module) can generate JSON output containing detailed information about the compilation steps. Other flakes then consume and process this JSON data, enabling advanced analysis and automation of the Rust build process.

## Architectural Goal: Lattice of Functions


Beyond the immediate build and development environment, a core architectural goal of this project is to transform the Rust codebase into a "canonical form" represented as a "lattice of functions." This involves rewriting the code to adhere to specific constraints, such as a topologically sorted dependency graph and limiting each module to at most one external crate dependency. This approach aims to enhance modularity, maintainability, and enable self-modifying capabilities.

Key components facilitating this transformation include:

*   **`rust-system-composer`**: Orchestrates the entire lattice transformation pipeline.
*   **`rust-decl-splitter`**: Decomposes Rust source files into individual declarations (functions, structs, enums, etc.), forming the granular nodes of the lattice.
*   **`prelude-generator`**: Generates prelude files by expanding macros and creating an Abstract Syntax Tree (AST), providing a comprehensive view of the code for analysis.

For a more detailed explanation of this architectural vision and its principles, please refer to `lattice.md`.

## Code Quality and Best Practices

To ensure high code quality and maintainability, especially for shell scripts, we adhere to specific best practices. For detailed guidelines on using `Shellcheck` and integrating it into your workflow, please refer to `docs/memos/Shellcheck_Always_After_Changes.md`.

## Configuration Documentation


# Configuration Documentation

This document details the various configuration files used within the `rust-bootstrap-nix` repository, primarily focusing on `config.toml` files that influence the Rust build process and environment setup.

## 1. Root `config.toml`

**File Path:** `/config.toml`

**Description:** This is the primary configuration file for the overall `rust-bootstrap-nix` environment. It explicitly defines how the Rust toolchain is sourced and how the build environment is isolated.

**Key Settings:**

*   `vendor = true`:
    *   **Purpose:** Enables vendoring for the Rust build process. This means that dependencies are expected to be present locally (e.g., in a `vendor/` directory) rather than being downloaded from the internet during the build. This is crucial for reproducible builds in a Nix environment.
*   `rustc = "/nix/store/.../bin/rustc"`:
    *   **Purpose:** Specifies the absolute path to the `rustc` (Rust compiler) executable within the Nix store. This ensures that the build uses a precisely defined and versioned compiler provided by Nix.
*   `cargo = "/nix/store/.../bin/cargo"`:
    *   **Purpose:** Specifies the absolute path to the `cargo` (Rust package manager) executable within the Nix store. Similar to `rustc`, this guarantees the use of a specific, Nix-managed `cargo` instance.
*   `HOME = "/data/data/com.termux.nix/files/usr/tmp/..."`:
    *   **Purpose:** Sets the `HOME` environment variable to a temporary, isolated directory. This prevents the build process from interacting with or polluting the user's actual home directory.
*   `CARGO_HOME = "/data/data/com.termux.nix/files/usr/tmp/.../.cargo"`:
    *   **Purpose:** Sets the `CARGO_HOME` environment variable to a temporary `.cargo` directory. This ensures that Cargo's caches, registries, and other state are kept isolated within the build environment.

**Overall Purpose:** The root `config.toml` is fundamental for establishing a hermetic and reproducible Rust build environment. It explicitly directs the build system to use Nix-provided tools and to operate within a clean, temporary workspace.

## 2. `standalonex/config.toml`

**File Path:** `/standalonex/config.toml`

**Description:** This configuration file is specific to the `standalonex` component, which is a standalone environment for the `x.py` build system. It defines the Rust toolchain paths that `x.py` should use within this isolated context.

**Key Settings:**

*   `rustc = "/nix/store/.../bin/rustc"`:
    *   **Purpose:** Similar to the root `config.toml`, this specifies the absolute path to the `rustc` executable, ensuring that the `standalonex` environment uses a Nix-provided compiler.
*   `cargo = "/nix/store/.../bin/cargo"`:
    *   **Purpose:** Specifies the absolute path to the `cargo` executable for the `standalonex` environment, guaranteeing the use of a specific, Nix-managed `cargo` instance.

**Overall Purpose:** This `config.toml` ensures that the `standalonex` build environment, particularly when running `x.py`, is correctly configured with the appropriate Nix-provided Rust toolchain binaries.

## 3. `standalonex/config.old.toml`

**File Path:** `/standalonex/config.old.toml`

**Description:** This file appears to be an older or template version of `standalonex/config.toml`. It is specifically used by the `standalonex/flake.nix`'s `buildPhase` as a base to generate the active `config.toml` by injecting the correct Nix store paths for `rustc` and `cargo` using `sed`.

**Purpose:** To serve as a template for generating the runtime `config.toml` within the `standalonex` build process, allowing for dynamic injection of Nix-specific paths.

## Configuring Relocatable Installation Paths for Nix

For Nix-based builds and to ensure the resulting artifacts are relocatable, it's crucial to properly configure the installation paths. The `[install]` section in your `config.toml` allows you to define a base prefix for all installed components.

### `[install]` Section

This section controls where the built artifacts will be placed.

*   `prefix`:
    *   **Purpose:** Specifies the base directory for all installed components. In a Nix environment, this will typically be a path within the Nix store (e.g., `/nix/store/...-rust-toolchain`). All other installation paths (like `bindir`, `libdir`, etc.) will be derived from this prefix unless explicitly overridden.
    *   **Example:** `prefix = "/nix/store/some-hash-my-rust-package"`

*   `bindir`:
    *   **Purpose:** Specifies the directory for executable binaries.
    *   **Behavior:** If `prefix` is set and `bindir` is *not* explicitly defined, `bindir` will automatically default to `prefix/bin`. This ensures that your executables are placed correctly within the specified installation prefix.
    *   **Example (explicitly set):** `bindir = "/usr/local/bin"` (overrides the default `prefix/bin`)

*   `libdir`, `sysconfdir`, `docdir`, `mandir`, `datadir`:
    *   **Purpose:** These fields specify directories for libraries, configuration files, documentation, manual pages, and data files, respectively.
    *   **Behavior:** If `prefix` is set, these paths are typically expected to be relative to the `prefix` unless an absolute path is provided.

### Nix-Specific Binary Patching

The `[build]` section also includes a relevant option for Nix:

*   `patch-binaries-for-nix`:
    *   **Purpose:** This boolean option enables Nix-specific patching of binaries. This is essential for ensuring that compiled artifacts are truly relocatable within the Nix store, often involving adjustments to RPATHs and other internal paths.
    *   **Example:** `patch-binaries-for-nix = true`

### Example `config.toml` for Relocatable Nix Builds

```toml
# config.toml
[install]
prefix = "/nix/store/some-hash-my-rust-package"
# bindir will automatically be set to "/nix/store/some-hash-my-rust-package/bin"
# libdir = "lib" # would resolve to /nix/store/some-hash-my-rust-package/lib

[build]
patch-binaries-for-nix = true
```

This configuration ensures that your Rust project builds and installs in a manner compatible with Nix's strict path requirements, promoting reproducibility and relocatability.

## Preconditions for Nix Flake Build

The `test_nix_preconditions.sh` script verifies essential environmental setups required for a successful Nix-based build of the Rust bootstrap. Ensuring these preconditions are met helps in maintaining a reproducible and stable build environment.

### 1. Nix Command Availability

*   **Check:** Verifies that the `nix` command-line tool is installed and accessible in the system's `PATH`.
*   **Importance:** Nix is fundamental to this build system, as it manages dependencies, builds packages, and ensures reproducibility. Without the `nix` command, the build process cannot proceed.

### 2. Rust Toolchain Sysroot Existence

*   **Check:** Evaluates the Nix store path for the `pkgs.rust-bin.stable."1.84.1".default` Rust toolchain (including its source) and confirms that the Rust source directory exists within it.
*   **Importance:** The Rust bootstrap process often requires access to the Rust compiler's source code (sysroot) for various build stages and internal operations. This precondition ensures that the necessary source components are available from the Nix-managed Rust toolchain.

### 3. Rust Source Flake (rustSrcFlake) Existence

*   **Check:** Evaluates the Nix store path for the `rustSrcFlake` input (which represents the Rust compiler's source code) as defined in `standalonex/flake.nix`, and verifies that this path exists and contains a known file (`src/ci/channel`).
*   **Importance:** The `bootstrap` binary needs to know the location of the Rust compiler's source tree to perform its build tasks. This precondition ensures that the `rustSrcFlake` input is correctly resolved and available, providing the necessary source for the bootstrap process.

## Nix Flakes Documentation

# Nix Flakes Documentation

## 1. Root `flake.nix`

**File Path:** `/flake.nix`

**Description:** This flake defines a Python and Rust development environment, with a strong emphasis on integrating `sccache` for accelerated Rust compilation. It supports both `aarch64-linux` and `x86_64-linux` systems. The core functionality revolves around providing a customized Rust toolchain that leverages `sccache` during the build process, particularly when running `python x.py build`.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   A custom `nixpkgs` instance, likely providing specific package versions or configurations tailored for the `meta-introspector` ecosystem.
*   `rust-overlay`: `github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify`
    *   A custom Nix overlay for Rust, also sourced from `meta-introspector`, suggesting specialized Rust toolchain management.
*   `rustSrcFlake`: `github:meta-introspector/rust?ref=e6c1b92d0abaa3f64032d6662cbcde980c826ff2`
    *   Points to a specific commit of a `rust` repository within `meta-introspector` organization. This appears to be the foundational Rust source that this flake extends and builds upon.

**Outputs:**

*   **`devShells.<system>.default` (for `aarch64-linux` and `x86_64-linux`):**
    *   Provides a comprehensive development environment.
    *   **Packages Included:**
        *   `rustToolchain` (nightly channel, with specific targets configured)
        *   `python3`
        *   `python3Packages.pip`
        *   `git`
        *   `curl`
        *   `which`
    *   **`shellHook`:** Sets `HOME` and `CARGO_HOME` to `$TMPDIR/.cargo` respectively, ensuring a clean and isolated build environment within the shell.
    *   **`nativeBuildInputs`:** `binutils`, `cmake`, `ninja`, `pkg-config`, `nix`. These are tools required during the build phase.
    *   **`buildInputs`:** `openssl`, `glibc.out`, `glibc.static`. These are runtime dependencies.
    *   **Environment Variables:** `RUSTC_ICE` is set to "0", and `LD_LIBRARY_PATH` is configured.

*   **`sccachedRustc` Function:**
    *   A local function that takes `system`, `pkgs`, and `rustToolchain` as arguments.
    *   Its primary role is to wrap the `rustSrcFlake`'s default package with `sccache` capabilities.
    *   **Modifications:**
        *   Adds `pkgs.sccache` and `pkgs.curl` to `nativeBuildInputs`.
        *   **`preConfigure`:** Injects environment variables (`RUSTC_WRAPPER`, `SCCACHE_DIR`, `SCCACHE_TEMPDIR`) to enable `sccache` and starts the `sccache` server.
        *   **`buildPhase`:** Significantly customizes the build process. It creates a `config.toml` file with `vendor = true`, and sets `rustc` and `cargo` paths to the provided `rustToolchain` binaries. It also sets `HOME` and `CARGO_HOME` for the build and executes `python x.py build`. This indicates that `x.py` is a central build orchestration script.
        *   **`preBuild` and `postBuild`:** Integrates `sccache` statistics reporting (`sccache --zero-stats`, `sccache --show-stats`, `sccache --stop-server`).

*   **`packages.<system>.default` (for `aarch64-linux` and `x86_64-linux`):**
    *   These outputs provide the `sccache`-enabled Rust compiler package, which is the result of applying the `sccachedRustc` function to the respective system's `rustToolchain`.

**Overall Purpose:** The root `flake.nix` serves as the entry point for setting up a robust, reproducible, and performance-optimized (via `sccache`) development and build environment for a Rust project that likely uses `python x.py build` as its primary build mechanism. It heavily relies on custom `meta-introspector` Nix inputs for its base components.

## 2. `flakes/config/flake.nix`

**File Path:** `/flakes/config/flake.nix`

**Description:** This flake is designed to read and process JSON output, specifically `xpy_json_output.json`, which is expected to be generated by the `rust-bootstrap-nix` project. It parses this JSON content and makes it available as a Nix package.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.
*   `rustBootstrapNix`: `github:meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001`
    *   **Self-Reference:** This input refers to the main `rust-bootstrap-nix` repository itself, specifically pointing to the `feature/bootstrap-001` branch. This establishes a dependency on the outputs of the main project's flake.

**Outputs:**

*   **`packages.aarch64-linux.default`:**
    *   This output creates a derivation named `processed-json-output`.
    *   It reads the `xpy_json_output.json` file from the `rustBootstrapNix.packages.aarch64-linux.default` (which is the `sccache`-enabled Rust compiler package from the root flake).
    *   The content of `xpy_json_output.json` is parsed as JSON using `builtins.fromJSON`.
    *   The parsed JSON content is then written to `$out/output.txt` within the derivation.

**Overall Purpose:** This flake acts as a consumer of the `xpy_json_output.json` produced by the main `rust-bootstrap-nix` build process. It allows for the structured consumption and further processing of this JSON data within the Nix ecosystem.

## 3. `flakes/evaluate-rust/flake.nix`

**File Path:** `/flakes/evaluate-rust/flake.nix`

**Description:** This flake provides a library function `evaluateCommand` designed for recursively evaluating Rust build commands and generating Nix packages. It aims to integrate `naersk` for `cargo build` commands and provides a generic mechanism for other commands.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.
*   `naersk`: `github:meta-introspector/naersk?ref=feature/CRQ-016-nixify`
    *   This input is for `rust2nix` functionality, indicating that this flake intends to use `naersk` to convert Rust projects into Nix derivations.

**Outputs:**

*   **`lib.evaluateCommand` function:** This is the primary output, a recursive function with the following parameters:
    *   `commandInfo`: An attribute set containing `command` (the executable, e.g., "cargo", "rustc"), `args` (a list of arguments), and `env` (environment variables).
    *   `rustSrc`: The source code of the Rust project.
    *   `currentDepth`: The current recursion depth.
    *   `maxDepth`: The maximum recursion depth to prevent infinite loops.

    **Function Logic:**
    *   **Base Case (Recursion Limit):** If `currentDepth` reaches `maxDepth`, it returns a derivation indicating that the recursion limit was reached.
    *   **`cargo build` Case:** If the command is `cargo` and includes the `build` argument, it uses `naersk.lib.${pkgs.system}.buildPackage` to create a Nix derivation. It passes `cargoBuildFlags` and `env` directly to `naersk`. This is a key integration point for Rust projects.
    *   **Other Commands Case:** For any other command (e.g., `rustc` directly), it creates a simple `pkgs.runCommand` derivation. It executes the command with its arguments and environment variables, capturing stdout and stderr to `output.txt`.

**Overall Purpose:** This flake provides a powerful, recursive mechanism to analyze and build Rust projects within Nix. By integrating `naersk`, it can effectively handle `cargo build` commands, transforming them into reproducible Nix derivations. The recursive nature suggests it might be used to trace and build dependencies or stages of a complex Rust build process.

## 4. `flakes/json-processor/flake.nix`

**File Path:** `/flakes/json-processor/flake.nix`

**Description:** This flake defines a Nix package that provides a Python environment with `jq` and `python3` installed. It's intended for processing JSON data, likely in a command-line or scripting context.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.

**Outputs:**

*   **`packages.aarch64-linux.default` and `packages.x86_64-linux.default`:**
    *   These outputs define a Nix package for each architecture.
    *   The package is a `pkgs.mkShell` (which is typically used for development shells, but can also be used to create environments with specific tools).
    *   **Packages Included:**
        *   `pkgs.jq`: A lightweight and flexible command-line JSON processor.
        *   `pkgs.python3`: The Python 3 interpreter.

**Overall Purpose:** This flake provides a convenient, reproducible environment for working with JSON data using `jq` and Python. It's a utility flake that can be imported by other flakes or used directly to get a shell with these tools.

## 5. `flakes/json-processor-flake/flake.nix`

**File Path:** `/flakes/json-processor-flake/flake.nix`

**Description:** This flake is very similar to `flakes/config/flake.nix` but specifically targets the `standalonex` flake within the `rust-bootstrap-nix` repository. Its purpose is to read and process the `xpy_json_output.json` generated by the `standalonex` flake.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.
*   `standalonex`: `github:meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=standalonex`
    *   **Self-Reference:** This input directly references the `standalonex` sub-flake within the `rust-bootstrap-nix` repository, specifically pointing to the `feature/bootstrap-001` branch and the `standalonex` directory. This demonstrates how sub-flakes within the same repository can expose their outputs for consumption by other flakes.

**Outputs:**

*   **`packages.aarch64-linux.default`:**
    *   This output creates a derivation named `processed-json-output`.
    *   It reads the `xpy_json_output.json` file from the `standalonex.packages.aarch64-linux.default` (which is the default package output of the `standalonex` flake).
    *   The content of `xpy_json_output.json` is parsed as JSON using `builtins.fromJSON`.
    *   The parsed JSON content is then written to `$out/output.txt` within the derivation.

**Overall Purpose:** This flake serves as a dedicated consumer and processor for the JSON output specifically from the `standalonex` component of the `rust-bootstrap-nix` project. It highlights the modularity of Nix flakes, allowing specific parts of a larger project to expose their outputs for consumption by other flakes.

## 6. `flakes/xpy-json-output-flake/flake.nix`

**File Path:** `/flakes/xpy-json-output-flake/flake.nix`

**Description:** This flake is specifically designed to execute the `x.py build --json-output` command from the `rustSrc` input and expose the resulting JSON output directory as a Nix package. This is a crucial flake for understanding the build process and its generated metadata.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.
*   `rustSrc`: `github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b`
    *   This input points to a specific commit of the `rust` repository within `meta-introspector`. It's marked as `flake = false`, indicating it's treated as a plain source input rather than another Nix flake. This `rustSrc` is where the `x.py` script resides.

**Outputs:**

*   **`packages.aarch64-linux.default`:**
    *   This output is a derivation named `xpy-json-output-derivation`.
    *   It uses `pkgs.runCommandLocal` to execute a local command.
    *   **`nativeBuildInputs`:** Includes `pkgs.python3` because `x.py` is a Python script.
    *   **`src`:** The `rustSrc` input is used as the source for this derivation.
    *   **Build Phase:**
        *   It creates an output directory `$out`.
        *   It then executes `python3 $src/x.py build --json-output $out`. This command is responsible for running the `x.py` build script and directing its JSON output to the `$out` directory of this derivation.

**Overall Purpose:** This flake provides a way to capture and expose the structured JSON output generated by the `x.py` build system of the `rustSrc` project. This output likely contains metadata about the build, such as compilation steps, dependencies, or configuration, which can then be consumed and analyzed by other Nix flakes (like the `json-processor` flakes we've seen).

## 7. `minimal-flake/flake.nix`

**File Path:** `/minimal-flake/flake.nix`

**Description:** This flake provides a very basic Python development environment and a simple "hello world" Python script packaged as a Nix derivation. It serves as a minimal example or a starting point for Python-centric Nix flakes.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.

**Outputs:**

*   **`devShell`:**
    *   A development shell named `minimal-python-dev-shell`.
    *   **Packages Included:** `python3` and `git`. This provides a basic environment for Python development and version control.

*   **`packages.<system>.helloPython`:
    *   A Nix package named `helloPython` for the `aarch64-linux` system.
    *   It uses `pkgs.writeScriptBin` to create an executable script.
    *   The script is a simple Python program that prints "Hello from Nix Python!".

**Overall Purpose:** This flake demonstrates how to set up a minimal Python development environment and package a simple Python script using Nix. It's likely used for quick testing, as a template, or to illustrate basic Nix flake concepts for Python projects.

## 8. `standalonex/flake.nix`

**File Path:** `/standalonex/flake.nix`

**Description:** This flake defines a standalone environment for working with `x.py`, which appears to be a custom build system for Rust projects. It provides a development shell with necessary tools and a package that executes `test_json_output.py` to generate and validate JSON output, likely related to the `x.py` build process.

**Inputs:**

*   `nixpkgs`: `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`
    *   Standard `nixpkgs` from `meta-introspector`.
*   `rustSrcFlake`: `github:meta-introspector/rust?ref=e6c1b92d0abaa3f64032d6662cbcde980c826ff2`
    *   The same `rust` source flake used in the root `flake.nix`, providing the `src/stage0` path.
*   `rustOverlay`: `github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify`
    *   The same `rust-overlay` used in the root `flake.nix`.

**Outputs:**

*   **`devShells.aarch64-linux.default`:**
    *   A development shell named `standalonex-dev-shell`.
    *   **Packages Included:** `pkgs.python3`.
    *   **`shellHook`:**
        *   Adds the flake's source directory (`${self}/`) to `PATH`, making `x.py` directly executable.
        *   Sets `RUST_SRC_STAGE0_PATH` to the `src/stage0` directory from `rustSrcFlake`.
        *   Creates a `config.toml` file with paths to `rustc` and `cargo` from `pkgs.rust-bin.stable.latest.default`.
        *   Sets `RUST_BOOTSTRAP_CONFIG` to the path of the generated `config.toml`.
        *   Creates dummy `etc/` files (`rust_analyzer_settings.json`, `rust_analyzer_eglot.el`, `rust_analyzer_helix.toml`) which are likely expected by `x.py` or related tools.

*   **`packages.aarch64-linux.default`:**
    *   A Nix package named `xpy-build-output`.
    *   **`src`:** Uses the flake's own source (`self`) as input.
    *   **`nativeBuildInputs`:** `pkgs.python3` and `pkgs.jq`.
    *   **`phases`:** Explicitly defines `buildPhase` and `installPhase`.
    *   **`buildPhase`:** This is the most complex part:
        *   It creates a writable temporary directory (`$TMPDIR/xpy_work`) and copies the flake's source into it.
        *   It then copies `config.old.toml` to `config.toml` and uses `sed` to inject the correct `rustc` and `cargo` paths into `config.toml`.
        *   Sets `RUST_BOOTSTRAP_CONFIG` to the path of the modified `config.toml`.
        *   Sets `HOME` and `CARGO_HOME` to writable temporary directories.
        *   Executes `python3 test_json_output.py --output-dir $out` to generate JSON files.
        *   Validates the generated JSON files using `jq`.
    *   **`installPhase`:** Is empty, as the output is generated directly in the `buildPhase`.

**Overall Purpose:** This flake is a self-contained environment for testing and generating output from the `x.py` build system. It meticulously sets up the necessary environment variables, configuration files, and dependencies to run `test_json_output.py`, which in turn uses `x.py` to produce JSON output. This output is then validated and exposed as a Nix package. This flake is crucial for understanding how the `x.py` build system is exercised and how its metadata is captured.

## Standalone x.py Environment

# Standalone x.py Environment

This directory contains a standalone version of the `x.py` script from the Rust compiler build system.
It is packaged as a Nix flake that can be built and tested independently.

## JSON Output Generation

The flake provides a package that builds the Rust compiler in a "dry run" mode.
In this mode, the build commands are not actually executed, but are captured in JSON files.
This is useful for analyzing the build process and for creating alternative build systems.

To build the package and generate the JSON files, run the following command from this directory:

```bash
nix build
```

The generated JSON files will be in the `result` directory.

### Sample JSON Output

Here is a sample of one of the generated JSON files:

```json
{
  "command": "/nix/store/lrr9mf5sg6qbas19z1ixjna024zkqws4-rust-default-1.90.0/bin/cargo",
  "args": [
    "build",
    "--manifest-path",
    "/nix/store/qsclyr4nsd25i5p9al261blrki1l9w31-source/standalonex/src/bootstrap/Cargo.toml"
  ],
  "env": {
    "SHELL": "/nix/store/hxmi7d6vbdgbzklm4icfk2y83ncw8la9-bash-5.3p3/bin/bash",
    "RUST_BOOTSTRAP_JSON_OUTPUT_DIR": "/nix/store/sc437kd47w1bajlcrdmmgdg0ng57f1l5-xpy-build-output-0.1.0",
    "..."
  },
  "cwd": "/nix/store/qsclyr4nsd25i5p9al261blrki1l9w31-source/standalonex",
  "type": "rust_compiler_invocation"
}
```

### Field Explanations

-   `command`: The command to be executed.
-   `args`: A list of arguments for the command.
-   `env`: A dictionary of environment variables for the command.
-   `cwd`: The working directory in which the command should be executed.
-   `type`: The type of the invocation. In this case, it's a rust compiler invocation.

## Bootstrap Builder Flake

# Bootstrap Builder Flake

This flake is responsible for building the Rust bootstrap compiler from source.

## Plan:
1.  Create a `flake.nix` file in this directory that builds the `bootstrap` compiler from the rust source.
2.  The `rust-src` will be an input to this flake, using a github URL with a specific git hash.
3.  The build will use `pkgs.rustPlatform.buildRustPackage`.
4.  After the `bootstrap` compiler is built, it will be used by the `standalonex` flake to generate the JSON output of the full Rust build process.
5.  The findings will then be documented in the `README.md` of the `standalonex` directory.

## build_helper

Types and functions shared across tools in this workspace.

---
**Note:** This `README.md` is a consolidation of several documentation files for easier access. The original files were:
- `CONFIGURATION.md`
- `NIX_FLAKES_DOCUMENTATION.md`
- `OVERVIEW.md`
- `standalonex/README.md`
- `flakes/bootstrap-builder/README.md`
- `standalonex/src/build_helper/README.md`