# Standalone x.py Environment

This directory contains the `standalonex` project, which provides a Nix-managed environment for building and testing the Rust bootstrap process. It includes a `flake.nix` that defines a development shell and packages for building the `bootstrap` and `nix_bootstrap` executables.

## Development Environment

To enter the development environment, navigate to this directory and run:

```bash
nix develop
```

This shell provides `python3`, `cargo`, and the nightly Rust toolchain for working with the Rust bootstrap.

## Building `bootstrap` and `nix_bootstrap`

The `flake.nix` defines Nix packages for building the `bootstrap` and `nix_bootstrap` executables. You can build them using:

```bash
nix build .#packages.aarch64-linux.default.bootstrap-main
nix build .#packages.aarch64-linux.default.nix-bootstrap
```
