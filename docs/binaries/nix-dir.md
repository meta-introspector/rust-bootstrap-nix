## `nix-dir`

**Purpose:** A utility for managing Nix directories or paths, specifically for finding the store path of a Nix package.

**Parameters:**

*   **`<PACKAGE_NAME>`** (positional argument, required)
    *   **Type:** `String`
    *   **Description:** The name of the Nix package to find (e.g., `rustc`, `cargo`).
*   **`--version <VERSION>`**
    *   **Type:** `String` (optional)
    *   **Description:** The version of the package to find. If not provided, the latest available version will be used.
