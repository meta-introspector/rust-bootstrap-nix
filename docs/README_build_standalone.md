## Building the Standalone Bootstrap

To build the standalone Rust bootstrap environment, which is particularly useful for "Nix on Droid" (aarch64-linux) environments, use the following Nix command:

```bash
nix build ./standalonex#packages.aarch64-linux.default
```

This command will build the default package defined within the `standalonex/flake.nix` for the `aarch64-linux` architecture.
