## Overall Plan: Refactor `configuration-nix` and Integrate with `config-generator`

This plan breaks down the task into logical, smaller steps. I will execute these steps sequentially, using `read_file` before each modification to ensure accuracy, and then `write_file` to apply the changes.

### Part 1: Refactor `configuration-nix/src/main.rs` to use `config_generator` module

1.  **Create `configuration-nix/src/config_generator.rs`:** This file already exists from a previous step. It contains the DWIM logic for discovering flake paths, querying Nix inputs, and constructing the `config.toml` content.
2.  **Modify `configuration-nix/src/main.rs`:**
    *   Remove all the existing logic (flake path discovery, Nix input querying, `config.toml` construction).
    *   Add `mod config_generator;` to declare the new module.
    *   Call `config_generator::generate_config_toml(stage_num, target_triple);` with the parsed command-line arguments.
3.  **Modify `configuration-nix/Cargo.toml`:** Add `config_generator` as a module to the `[lib]` section (or `[bin]` if it's a binary, but it's a module for `main.rs`).

### Part 2: Update `configuration-nix/flake.nix` for new inputs

1.  **Modify `configuration-nix/flake.nix`:**
    *   Add `rustSrcFlake` as an input. This is necessary because `config_generator.rs` now queries for `rustSrcFlake_path`.
    *   Ensure `configurationNix` input points to the current flake itself (this is already the case, but good to verify).

### Part 3: Integrate `configuration-nix` changes into `flakes/config-generator/flake.nix`

1.  **Modify `flakes/config-generator/flake.nix`:**
    *   **Add `rustSrcFlake` input:** Ensure `rustSrcFlake` is an input to `flakes/config-generator/flake.nix`.
    *   **Update `generateConfigTomlForStage`:** Simplify the `pkgs.runCommand` to just call `configurationNix.packages.${system}.default` with `stageNum` and `targetTriple` as arguments. Remove the environment variables `RUSTC_PATH`, `CARGO_PATH`, etc., as the Rust program will now discover these itself.
    *   **Update `configGeneratorScript`:** Simplify the script to just call `configurationNix.packages.${system}.default` with `stageNum` and `targetTriple` as arguments.
    *   **Update `packages` output:** Ensure the `packages` output correctly calls `generateConfigTomlForStage` with the required arguments.

### Current Status: `bootstrap-config-builder` Refactoring

The `bootstrap-config-builder` crate has been successfully refactored to use utility functions in `utils.rs` and now correctly generates `config.toml` by querying Nix flakes. This was achieved by directly overwriting files using `write_file` after modifications were confirmed.