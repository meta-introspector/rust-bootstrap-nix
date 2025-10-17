# Scripts Documentation

## 1. `debug_build.sh`

**File Path:** `/debug_build.sh`

**Description:** This script is designed to set up a debug build environment and then execute the `x.py build` command. It prints out environment information (`PATH`, `which curl`), creates a `config.toml` with specific settings (`patch-binaries-for-nix = true`, `vendor = true`, and paths to `rustc` and `cargo` obtained via `which`), and then runs `python x.py --config ./config.toml build`.

**Purpose:** To facilitate debugging of the `x.py` build process by explicitly setting up a `config.toml` and showing relevant environment variables.

## 2. `develop.sh`

**File Path:** `/develop.sh`

**Description:** This is a simple wrapper script that executes `nix develop`. It specifically overrides the `nixpkgs` input to point to `github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify`, ensuring that the development environment is built using the specified `meta-introspector` version of `nixpkgs`. It also passes any additional arguments (`"$@"`) to `nix develop`.

**Purpose:** To provide a convenient way to enter the Nix development shell defined in the `flake.nix` of the current directory, while enforcing the use of a specific `nixpkgs` input.

## 3. `diagnose.sh`

**File Path:** `/diagnose.sh`

**Description:** This script is designed to provide diagnostic information about the build environment. It outputs key environment variables (`HOME`, `CARGO_HOME`, `PATH`), attempts to locate `curl`, `rustc`, and `cargo` executables within the `PATH`, displays the content of `config.toml`, and finally runs `python x.py build -vv` to execute the build with very verbose output.

**Purpose:** To help identify and troubleshoot issues related to the build environment, tool locations, configuration, and the `x.py` build process itself by providing detailed diagnostic information.

## 4. `eval_json.sh`

**File Path:** `/eval_json.sh`

**Description:** This script is designed to read a hardcoded JSON file from the Nix store (`/nix/store/hdv212g3rgir248dprwg6bhkz50kkxhb-xpy-build-output-0.1.0/xpy_json_output.json`), parse its content, and then use `nix eval` to extract a specific field (`command`) from the parsed JSON. It includes error handling for an empty JSON content.

**Purpose:** To demonstrate how to extract specific data from a JSON file that is part of a Nix derivation, likely for further processing or analysis within a Nix context. This script directly interacts with the output of the `xpy-build-output` package (from `standalonex/flake.nix`).

## 5. `get_nix_paths.sh`

**File Path:** `/get_nix_paths.sh`

**Description:** This script uses `nix eval --impure --raw` to retrieve the Nix store paths for `sccache`, `curl`, `rustc`, and `cargo`. It specifically evaluates paths from `/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust/eval-rust-env`.

**Purpose:** To collect and display the absolute Nix store paths of essential build tools and compilers. This is useful for verifying that the correct versions of these tools are being used and for debugging purposes. The hardcoded path suggests it's part of a larger system where `eval-rust-env` is a known flake or package.

## 6. `test.sh`

**File Path:** `/test.sh`

**Description:** This script attempts to replicate a Nix build environment for testing purposes. It hardcodes Nix store paths for various tools (`sccache`, `curl`, `rustc`, `cargo`, `grep`), sets up temporary directories for `HOME`, `CARGO_HOME`, and `CARGO_TARGET_DIR`, and then constructs a `config.toml` file with these hardcoded paths. It then executes the `x.py build` command with specific arguments and features, mimicking a build process. Finally, it cleans up the temporary directory.

**Purpose:** To provide a reproducible testing environment outside of a full Nix build, allowing for isolated testing of the `x.py` build system and its interaction with various tools. It essentially simulates the environment that the root `flake.nix` would create for a build.
