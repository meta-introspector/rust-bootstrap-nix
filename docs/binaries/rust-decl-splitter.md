## `rust-decl-splitter`

**Purpose:** Splits Rust declarations (functions, structs, enums, traits, modules, impl blocks) from an input directory of Rust files into separate files in an output directory. It also attempts to instrument functions with measurement calls.

**Parameters:**

*   **`<INPUT_DIRECTORY>`** (positional argument, required)
    *   **Type:** `PathBuf`
    *   **Description:** The path to the directory containing Rust files to be processed.
*   **`<OUTPUT_DIRECTORY>`** (positional argument, required)
    *   **Type:** `PathBuf`
    *   **Description:** The path to the directory where the split declaration files will be written.
