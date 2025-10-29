## `hf-validator`

**Purpose:** A comprehensive Rust compilation analysis toolkit that extracts semantic analysis, project structure, and LLVM IR generation data from Rust codebases for machine learning and compiler research. It acts as a dispatcher for various subcommands, each with its own set of parameters.

**Subcommands and Parameters:**

*   **`test-mock`**
    *   **Purpose:** Runs mock dataset tests.
    *   **Parameters:** None
*   **`test-solfunmeme`**
    *   **Purpose:** Runs solfunmeme dataset tests.
    *   **Parameters:** None
*   **`benchmark`**
    *   **Purpose:** Runs performance benchmarks.
    *   **Parameters:** None
*   **`export-all <OUTPUT_PATH>`**
    *   **Purpose:** Exports solfunmeme dataset to JSONL.
    *   **Parameters:**
        *   **`<OUTPUT_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output JSONL file. Defaults to `solfunmeme_export.jsonl`.
*   **`export-stats <OUTPUT_PATH>`**
    *   **Purpose:** Exports solfunmeme dataset statistics.
    *   **Parameters:**
        *   **`<OUTPUT_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output JSON file. Defaults to `solfunmeme_stats.json`.
*   **`create-sample <OUTPUT_PATH>`**
    *   **Purpose:** Creates a sample dataset.
    *   **Parameters:**
        *   **`<OUTPUT_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output sample dataset directory. Defaults to `solfunmeme_sample`.
*   **`create-hf-dataset <OUTPUT_PATH>`**
    *   **Purpose:** Creates a Hugging Face dataset.
    *   **Parameters:**
        *   **`<OUTPUT_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output Hugging Face dataset directory. Defaults to `solfunmeme-hf-dataset`.
*   **`validate-parquet <DATASET_PATH>`**
    *   **Purpose:** Validates a Parquet dataset.
    *   **Parameters:**
        *   **`<DATASET_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the dataset directory. Defaults to `solfunmeme-hf-dataset`.
*   **`demo-dataset <DATASET_PATH>`**
    *   **Purpose:** Demonstrates dataset loading.
    *   **Parameters:**
        *   **`<DATASET_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the dataset directory. Defaults to `solfunmeme-hf-dataset`.
*   **`analyze-rust-project <PROJECT_PATH> [OUTPUT_PATH]`**
    *   **Purpose:** Analyzes a Rust project with `rust-analyzer` for all processing phases.
    *   **Parameters:**
        *   **`<PROJECT_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Rust project.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `rust-analyzer-datasets`.
*   **`analyze-rust-phases <PROJECT_PATH> [PHASES_STR] [OUTPUT_PATH]`**
    *   **Purpose:** Analyzes specific Rust processing phases with `rust-analyzer`.
    *   **Parameters:**
        *   **`<PROJECT_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Rust project.
        *   **`[PHASES_STR]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Comma-separated list of phases (e.g., `parsing,name_resolution,type_inference`). Defaults to `parsing,name_resolution,type_inference`.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `rust-analyzer-phase-datasets`.
*   **`validate-rust-analyzer-datasets <DATASET_PATH>`**
    *   **Purpose:** Validates `rust-analyzer` generated datasets.
    *   **Parameters:**
        *   **`<DATASET_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the dataset directory. Defaults to `rust-analyzer-datasets`.
*   **`generate-hf-dataset <PROJECT_PATH> [OUTPUT_PATH]`**
    *   **Purpose:** Generates a HuggingFace dataset with Parquet files from `rust-analyzer` records.
    *   **Parameters:**
        *   **`<PROJECT_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Rust project.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `rust-analyzer-hf-dataset`.
*   **`analyze-cargo-project <PROJECT_PATH> [OUTPUT_PATH] [INCLUDE_DEPS]`**
    *   **Purpose:** Analyzes a Cargo project with `cargo2hf`.
    *   **Parameters:**
        *   **`<PROJECT_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Cargo project.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `cargo2hf-dataset`.
        *   **`[INCLUDE_DEPS]`** (positional argument, optional)
            *   **Type:** `bool` (`true` or `false`)
            *   **Description:** Whether to include dependencies in the analysis. Defaults to `false`.
*   **`analyze-cargo-ecosystem <PROJECT_PATH> [OUTPUT_PATH]`**
    *   **Purpose:** Analyzes a Cargo ecosystem (project + dependencies).
    *   **Parameters:**
        *   **`<PROJECT_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Cargo project.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `cargo-ecosystem-dataset`.
*   **`validate-cargo-dataset <DATASET_PATH>`**
    *   **Purpose:** Validates `cargo2hf` generated dataset.
    *   **Parameters:**
        *   **`<DATASET_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the dataset directory. Defaults to `cargo2hf-dataset`.
*   **`analyze-llvm-ir <SOURCE_PATH> [OUTPUT_PATH] [OPT_LEVELS]`**
    *   **Purpose:** Analyzes LLVM IR generation from Rust source.
    *   **Parameters:**
        *   **`<SOURCE_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Rust source file.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `llvm-ir-dataset`.
        *   **`[OPT_LEVELS]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Comma-separated list of optimization levels (e.g., `O0,O1,O2,O3`). Defaults to `O0,O1,O2,O3`.
*   **`analyze-rust-to-ir <SOURCE_PATH> [OUTPUT_PATH]`**
    *   **Purpose:** Performs comprehensive Rust → LLVM IR pipeline analysis.
    *   **Parameters:**
        *   **`<SOURCE_PATH>`** (positional argument, required)
            *   **Type:** `String`
            *   **Description:** Path to the Rust source file or project.
        *   **`[OUTPUT_PATH]`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the output dataset directory. Defaults to `rust-to-ir-dataset`.
*   **`validate-llvm-dataset <DATASET_PATH>`**
    *   **Purpose:** Validates LLVM IR analysis dataset.
    *   **Parameters:**
        *   **`<DATASET_PATH>`** (positional argument, optional)
            *   **Type:** `String`
            *   **Description:** Path to the dataset directory. Defaults to `llvm-ir-dataset`.
