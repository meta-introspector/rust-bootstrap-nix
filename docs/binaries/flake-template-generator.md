## `flake-template-generator`

**Purpose:** Generates a new Nix flake based on a configuration file and specified parameters, including options for Git operations.

**Parameters:**

*   **`--config-path <CONFIG_PATH>`**
    *   **Type:** `PathBuf`
    *   **Description:** Path to the generated `config.toml` file.
*   **`--output-dir <OUTPUT_DIR>`**
    *   **Type:** `PathBuf`
    *   **Description:** Output directory for the new flake.
*   **`--component <COMPONENT>`**
    *   **Type:** `String`
    *   **Description:** Component for the branch name (e.g., `solana-rust-1.83`).
*   **`--arch <ARCH>`**
    *   **Type:** `String`
    *   **Description:** Architecture for the branch name (e.g., `aarch64`).
*   **`--phase <PHASE>`**
    *   **Type:** `String`
    *   **Description:** Phase for the branch name (e.g., `phase0`).
*   **`--step <STEP>`**
    *   **Type:** `String`
    *   **Description:** Step for the branch name (e.g., `step1`).
*   **`--dry-run`**
    *   **Type:** `bool` (flag)
    *   **Description:** Perform a dry run without executing Git commands. Defaults to `false`.
*   **`--verbose`**
    *   **Type:** `bool` (flag)
    *   **Description:** Show verbose output for Git operations. Defaults to `false`.
