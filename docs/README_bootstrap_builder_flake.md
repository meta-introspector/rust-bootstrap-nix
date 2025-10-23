# Bootstrap Builder Flake

This flake is responsible for building the Rust bootstrap compiler from source.

## Plan:
1.  Create a `flake.nix` file in this directory that builds the `bootstrap` compiler from the rust source.
2.  The `rust-src` will be an input to this flake, using a github URL with a specific git hash.
3.  The build will use `pkgs.rustPlatform.buildRustPackage`.
4.  After the `bootstrap` compiler is built, it will be used by the `standalonex` flake to generate the JSON output of the full Rust build process.
5.  The findings will then be documented in the `README.md` of the `standalonex` directory.
