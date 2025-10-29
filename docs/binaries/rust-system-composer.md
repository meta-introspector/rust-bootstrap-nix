
## `rust-system-composer`

**Purpose:** Orchestrates the execution of `prelude-generator` and `rust-decl-splitter` to compose a Rust system.

**Parameters:**

*   **`--workspace-root <WORKSPACE_ROOT>`**
    *   **Type:** `PathBuf`
    *   **Description:** The path to the workspace root for `prelude-generator`.
*   **`--input-dir <INPUT_DIR>`**
    *   **Type:** `PathBuf`
    *   **Description:** The input directory for `rust-decl-splitter`.
*   **`--output-dir <OUTPUT_DIR>`**
    *   **Type:** `PathBuf`
    *   **Description:** The output directory for `rust-decl-splitter`.
*   **`--dry-run`**
    *   **Type:** `bool` (flag)
    *   **Description:** Run in dry-run mode, printing changes without modifying files.
