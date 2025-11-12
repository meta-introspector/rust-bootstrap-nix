### Example `config.toml` for Relocatable Nix Builds

```toml
# config.toml
[install]
prefix = "/nix/store/some-hash-my-rust-package"
# bindir will automatically be set to "/nix/store/some-hash-my-rust-package/bin"
# libdir = "lib" # would resolve to /nix/store/some-hash-my-rust-package/lib

[build]
patch-binaries-for-nix = true
```

This configuration ensures that your Rust project builds and installs in a manner compatible with Nix's strict path requirements, promoting reproducibility and relocatability.
