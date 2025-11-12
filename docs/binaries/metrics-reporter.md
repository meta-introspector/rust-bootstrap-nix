## `metrics-reporter`

**Purpose:** Creates a temporary Rust project, copies a wrapped code file into it, modifies its `Cargo.toml` and `main.rs` to include measurement logic, runs the temporary project, captures its output, extracts JSON metrics, and generates a `rollup_report.md` with code metrics and performance data.

**Parameters:**

*   **`<WRAPPED_CODE_PATH>`** (positional argument, required)
    *   **Type:** `PathBuf`
    *   **Description:** Path to the Rust file containing the code to be wrapped and analyzed.
*   **`<ROLLUP_DATA_DIR>`** (positional argument, required)
    *   **Type:** `PathBuf`
    *   **Description:** Directory where the `rollup_report.md` will be generated.
