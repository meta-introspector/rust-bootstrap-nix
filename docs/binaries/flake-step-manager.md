## `flake-step-manager`

**Purpose:** Manages steps or phases within a Nix flake-based workflow, taking specific parameters to define the context of the operation.

**Parameters:**

*   **`--rust-version <RUST_VERSION>`**
    *   **Type:** `String`
    *   **Description:** The Rust version associated with the step.
*   **`--arch <ARCH>`**
    *   **Type:** `String`
    *   **Description:** The architecture for which the step is being managed.
*   **`--phase <PHASE>`**
    *   **Type:** `String`
    *   **Description:** The phase of the workflow (e.g., `configure`, `build`, `test`).
*   **`--step <STEP>`**
    *   **Type:** `String`
    *   **Description:** The specific step within the phase.
