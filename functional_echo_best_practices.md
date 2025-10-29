## Best Practices from `echo` (README.md, Cargo.toml, rmg-core/src/lib.rs, math/mod.rs, math/prng.rs, rmg-ffi/src/lib.rs, rmg-wasm/src/lib.rs, docs/*.md)

*   **Vision-Driven Development:** Clearly articulating a bold vision ("Echo is a recursive metagraph (RMG) simulation engine...") and core principles guides all development efforts.
*   **Extreme Determinism:** Implementing BLAKE3 hashing for IDs and snapshots, using deterministic PRNGs (`xoroshiro128+`), and explicitly handling floating-point precision are paramount for reproducibility across runs and environments.
*   **Content-Addressed Storage:** Using hashes (BLAKE3) for addressing nodes, types, snapshots, and rewrite rules ensures data integrity, provenance, and efficient caching.
*   **Transactional and Atomic Operations:** Implementing `begin`, `apply`, `commit` for graph rewrites ensures data consistency and snapshot isolation.
*   **Deterministic Scheduling:** Ordering rewrite rules based on `(scope_hash, rule_id)` guarantees consistent execution order, crucial for reproducibility.
*   **Monorepo Structure for Related Crates:** Organizing `rmg-core`, `rmg-ffi`, `rmg-wasm`, `rmg-cli` within a single workspace simplifies dependency management and promotes code sharing.
*   **Comprehensive Documentation:** The extensive `docs/` directory with architecture outlines, decision logs, execution plans, and detailed specifications is a gold standard for project documentation, crucial for complex systems.
*   **FFI and WASM for Interoperability:** Providing C-compatible bindings (`rmg-ffi`) and `wasm-bindgen` bindings (`rmg-wasm`) enables seamless integration with other languages (Lua, Python, JavaScript) and platforms.
*   **Strong Typing for Clarity and Safety:** Using strongly typed identifiers (`NodeId`, `TypeId`, `EdgeId`) prevents accidental mixing of different types of identifiers.
*   **Explicit Error Handling:** Using `thiserror` for custom error types (`EngineError`) provides clear and structured error reporting.
*   **Versioned PRNG Algorithms:** `PRNG_ALGO_VERSION` in `prng.rs` is a good practice for tracking changes in deterministic algorithms and ensuring backward compatibility or clear migration paths.
*   **Detailed Build Profiles:** Similar to `category_theory`, `echo` also uses detailed build profiles for performance optimization.
*   **Clear Contribution Guidelines:** `CONTRIBUTING.md` and `AGENTS.md` provide clear expectations for contributors, including branching workflows, testing, documentation, and code style.
*   **Security Policy:** `SECURITY.md` outlines how to report vulnerabilities and the project's approach to security, which is essential for any public project.
*   **Git Best Practices (Echo's perspective):** Explicitly forbidding `git --force`, `git rebase`, and `git amend` in `AGENTS.md` to preserve a messy but truthful distributed history, prioritizing truth over tidiness.