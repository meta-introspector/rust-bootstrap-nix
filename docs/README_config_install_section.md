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
