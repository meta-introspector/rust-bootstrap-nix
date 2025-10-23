## JSON Output Generation

The flake provides a package that builds the Rust compiler in a "dry run" mode.
In this mode, the build commands are not actually executed, but are captured in JSON files.
This is useful for analyzing the build process and for creating alternative build systems.

To build the package and generate the JSON files, run the following command from this directory:

```bash
nix build
```

The generated JSON files will be in the `result` directory.

### Sample JSON Output

Here is a sample of one of the generated JSON files:

```json
{
  "command": "/nix/store/lrr9mf5sg6qbas19z1ixjna024zkqws4-rust-default-1.90.0/bin/cargo",
  "args": [
    "build",
    "--manifest-path",
    "/nix/store/qsclyr4nsd25i5p9al261blrki1l9w31-source/standalonex/src/bootstrap/Cargo.toml"
  ],
  "env": {
    "SHELL": "/nix/store/hxmi7d6vbdgbzklm4icfk2y83ncw8la9-bash-5.3p3/bin/bash",
    "RUST_BOOTSTRAP_JSON_OUTPUT_DIR": "/nix/store/sc437kd47w1bajlcrdmmgdg0ng57f1l5-xpy-build-output-0.1.0",
    "..."
  },
  "cwd": "/nix/store/qsclyr4nsd25i5p9al261blrki1l9w31-source/standalonex",
  "type": "rust_compiler_invocation"
}
```

### Field Explanations

-   `command`: The command to be executed.
-   `args`: A list of arguments for the command.
-   `env`: A dictionary of environment variables for the command.
-   `cwd`: The working directory in which the command should be executed.
-   `type`: The type of the invocation. In this case, it's a rust compiler invocation.
