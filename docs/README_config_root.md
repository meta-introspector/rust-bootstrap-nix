## 1. Root `config.toml`

**File Path:** `/config.toml`

**Description:** This is the primary configuration file for the overall `rust-bootstrap-nix` environment. It explicitly defines how the Rust toolchain is sourced and how the build environment is isolated.

**Key Settings:**

*   `vendor = true`:
    *   **Purpose:** Enables vendoring for the Rust build process. This means that dependencies are expected to be present locally (e.g., in a `vendor/` directory) rather than being downloaded from the internet during the build. This is crucial for reproducible builds in a Nix environment.
*   `rustc = "/nix/store/.../bin/rustc"`:
    *   **Purpose:** Specifies the absolute path to the `rustc` (Rust compiler) executable within the Nix store. This ensures that the build uses a precisely defined and versioned compiler provided by Nix.
*   `cargo = "/nix/store/.../bin/cargo"`:
    *   **Purpose:** Specifies the absolute path to the `cargo` (Rust package manager) executable within the Nix store. Similar to `rustc`, this guarantees the use of a specific, Nix-managed `cargo` instance.
*   `HOME = "/data/data/com.termux.nix/files/usr/tmp/..."`:
    *   **Purpose:** Sets the `HOME` environment variable to a temporary, isolated directory. This prevents the build process from interacting with or polluting the user's actual home directory.
*   `CARGO_HOME = "/data/data/com.termux.nix/files/usr/tmp/.../.cargo"`:
    *   **Purpose:** Sets the `CARGO_HOME` environment variable to a temporary `.cargo` directory. This ensures that Cargo's caches, registries, and other state are kept isolated within the build environment.

**Overall Purpose:** The root `config.toml` is fundamental for establishing a hermetic and reproducible Rust build environment. It explicitly directs the build system to use Nix-provided tools and to operate within a clean, temporary workspace.
