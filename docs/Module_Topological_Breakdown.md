# Module Topological Breakdown

This document outlines the topological layering of modules within the project, based on their internal dependencies as defined in their `Cargo.toml` files. This breakdown helps in understanding the project's structure and the impact of changes across different components.

---

### Layer 1: Foundational Modules
These modules have no internal dependencies on other workspace members. They primarily depend on external crates or are standalone.

*   `/bootstrap-config-builder`
*   `/configuration-nix`
*   `/flake-template-generator`
*   `/test_definitions_crates/test_definitions_lib`
*   `/test-openssl-sys`
*   `/standalonex/src/config_core`
*   `/standalonex/src/stage0_parser_crate`
*   `/test_definitions_crates/test_definitions_macro`
*   `/standalonex/src/build_helper`
*   `/standalonex/src/bootstrap/src/core/build_steps/test_utils` (bootstrap-test-utils)

### Layer 2: Intermediate Modules (Level 1)
These modules depend only on modules from Layer 1.

*   `/standalonex/src/config_macros` (Depends on `standalonex/src/config_core`)
*   `/standalonex/src/bootstrap/src/core/config_utils` (bootstrap-config-utils) (Depends on `standalonex/src/stage0_parser_crate`)
*   `/test_definitions_crates/test_definitions_test` (Depends on `test_definitions_crates/test_definitions_macro`)

### Layer 3: Intermediate Modules (Level 2)
These modules depend on modules from Layer 1 and/or Layer 2.

*   `/standalonex/src/bootstrap-config-types` (Depends on `standalonex/src/config_macros` (L2) and `standalonex/src/build_helper` (L1))
*   `/standalonex/src/bootstrap/src/core/config_processor` (bootstrap-config-processor) (Depends on `standalonex/src/bootstrap/src/core/config_utils` (L2))

### Layer 4: Top-Level Application Module
This module depends on modules from Layer 1, 2, and/or 3.

*   `/standalonex/src/bootstrap` (Depends on `bootstrap-config-types` (L3), `config_core` (L1), `config_macros` (L2), `bootstrap-config-utils` (L2), `bootstrap-test-utils` (L1), `build_helper` (L1), `stage0_parser_crate` (L1), `test_definitions_macro` (L1))
