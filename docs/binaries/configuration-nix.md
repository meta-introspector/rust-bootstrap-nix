## `configuration-nix`

**Purpose:** Generates a `config.toml` file based on provided parameters, likely for configuring a Nix-based Rust build environment.

**Parameters:**

*   **`<STAGE>`** (positional argument)
    *   **Type:** `String`
    *   **Description:** The bootstrap stage number (e.g., `0`, `1`, `2`).
*   **`<TARGET>`** (positional argument)
    *   **Type:** `String`
    *   **Description:** The target triple for the build (e.g., `aarch64-unknown-linux-gnu`).
*   **`--nixpkgs-path <NIXPKGS_PATH>`**
    *   **Type:** `PathBuf`
    *   **Description:** Path to the Nixpkgs flake input.
*   **`--rust-overlay-path <RUST_OVERLAY_PATH>`**
    *   **Type:** `PathBuf`
    *   **Description:** Path to the Rust overlay flake input.
*   **`--configuration-nix-path <CONFIGURATION_NIX_PATH>`**
    *   **Type:** `PathBuf`
    *   **Description:** Path to the `configurationNix` flake input.
*   **`--rust-src-flake-path <RUST_SRC_FLAKE_PATH>`**
    *   **Type:** `PathBuf`
    *   **Description:** Path to the `rustSrcFlake` input.
*   **`--rust-bootstrap-nix-flake-ref <RUST_BOOTSTRAP_NIX_FLAKE_REF>`**
    *   **Type:** `String`
    *   **Description:** The flake reference for the `rust-bootstrap-nix` repository.
*   **`--rust-src-flake-ref <RUST_SRC_FLAKE_REF>`**
    *   **Type:** `String`
    *   **Description:** The flake reference for the Rust source.
