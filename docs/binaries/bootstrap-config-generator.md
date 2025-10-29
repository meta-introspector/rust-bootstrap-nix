## `bootstrap-config-generator`

**Purpose:** Generates `config.toml` files for the Rust bootstrap process, leveraging Nix store paths for `rustc` and `cargo`, and facilitating systematic testing of different Rust toolchain versions from the Nix store.

**Parameters:**

*   **`--config-file <CONFIG_FILE>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to a TOML configuration file to load. Arguments provided on the command line will override values from this file.
*   **`--output <OUTPUT>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the output `config.toml` file. If not provided, defaults to `config.toml` in the current directory.
*   **`--dry-run`**
    *   **Type:** `bool` (flag)
    *   **Description:** If set, the generated configuration will be printed to stdout instead of being written to a file.
*   **`--system <SYSTEM>`**
    *   **Type:** `String` (optional)
    *   **Description:** The Nix system architecture (e.g., `x86_64-linux`, `aarch64-linux`).
*   **`--project-root <PROJECT_ROOT>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** The root directory of the Rust project.
*   **`--nixpkgs-path <NIXPKGS_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the Nixpkgs source.
*   **`--rust-overlay-path <RUST_OVERLAY_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the Rust overlay.
*   **`--rust-bootstrap-nix-path <RUST_BOOTSTRAP_NIX_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the `rust-bootstrap-nix` project.
*   **`--configuration-nix-path <CONFIGURATION_NIX_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the `configuration-nix` project.
*   **`--rust-src-flake-path <RUST_SRC_FLAKE_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the Rust source flake.
*   **`--stage <STAGE>`**
    *   **Type:** `String` (optional)
    *   **Description:** The bootstrap stage (e.g., `stage0`, `stage1`).
*   **`--target <TARGET>`**
    *   **Type:** `String` (optional)
    *   **Description:** The compilation target (e.g., `x86_64-unknown-linux-gnu`).
*   **`--rust-bootstrap-nix-flake-ref <RUST_BOOTSTRAP_NIX_FLAKE_REF>`**
    *   **Type:** `String` (optional)
    *   **Description:** Git reference for the `rust-bootstrap-nix` flake.
*   **`--rust-src-flake-ref <RUST_SRC_FLAKE_REF>`**
    *   **Type:** `String` (optional)
    *   **Description:** Git reference for the Rust source flake.
*   **`--rustc-path <RUSTC_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the `rustc` executable.
*   **`--cargo-path <CARGO_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the `cargo` executable.
*   **`--rust-channel <RUST_CHANNEL>`**
    *   **Type:** `String` (optional)
    *   **Description:** Rust toolchain channel (e.g., `stable`, `beta`, `nightly`).
*   **`--rust-download-rustc`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to download `rustc`.
*   **`--rust-parallel-compiler`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to use a parallel compiler.
*   **`--rust-llvm-tools`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to include LLVM tools.
*   **`--rust-debuginfo-level <RUST_DEBUGINFO_LEVEL>`**
    *   **Type:** `u32` (optional)
    *   **Description:** Debug information level.
*   **`--patch-binaries-for-nix`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to patch binaries for Nix.
*   **`--vendor`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to vendor dependencies.
*   **`--build-dir <BUILD_DIR>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** The build directory.
*   **`--build-jobs <BUILD_JOBS>`**
    *   **Type:** `u32` (optional)
    *   **Description:** Number of parallel build jobs.
*   **`--home-dir <HOME_DIR>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** The home directory.
*   **`--cargo-home-dir <CARGO_HOME_DIR>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** The Cargo home directory.
*   **`--install-prefix <INSTALL_PREFIX>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Installation prefix.
*   **`--install-sysconfdir <INSTALL_SYSCONFDIR>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** System configuration directory for installation.
*   **`--dist-sign-folder <DIST_SIGN_FOLDER>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Folder for distribution signing.
*   **`--dist-upload-addr <DIST_UPLOAD_ADDR>`**
    *   **Type:** `String` (optional)
    *   **Description:** Address for uploading distribution artifacts.
*   **`--llvm-download-ci-llvm`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to download CI LLVM.
*   **`--llvm-ninja`**
    *   **Type:** `bool` (flag)
    *   **Description:** Whether to use Ninja for LLVM builds.
*   **`--change-id <CHANGE_ID>`**
    *   **Type:** `String` (optional)
    *   **Description:** Change ID for tracking.
*   **`--build-rustc-version <BUILD_RUSTC_VERSION>`**
    *   **Type:** `String` (optional)
    *   **Description:** The Rustc version to build. This enables a special lattice generation mode.
*   **`--solana-rustc-path <SOLANA_RUSTC_PATH>`**
    *   **Type:** `PathBuf` (optional)
    *   **Description:** Path to the Solana Rustc. Required when `build-rustc-version` is used.
