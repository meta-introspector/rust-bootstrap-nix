### Nix-Specific Binary Patching

The `[build]` section also includes a relevant option for Nix:

*   `patch-binaries-for-nix`:
    *   **Purpose:** This boolean option enables Nix-specific patching of binaries. This is essential for ensuring that compiled artifacts are truly relocatable within the Nix store, often involving adjustments to RPATHs and other internal paths.
    *   **Example:** `patch-binaries-for-nix = true`
