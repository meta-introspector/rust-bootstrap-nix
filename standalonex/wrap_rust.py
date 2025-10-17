#!/usr/bin/env python3

import os
import sys
import json
import subprocess

# Counter for flake steps
flake_step_counter = 0

def generate_nix_flake(command, args, env):
    global flake_step_counter
    flake_step_counter += 1
    step_dir = f"test-rust2/bootstrap/step{flake_step_counter:03d}"
    os.makedirs(step_dir, exist_ok=True)

    flake_content = f"""
{{
  description = "Nix flake for Rust compiler command step {flake_step_counter}";

  outputs = {{ self, nixpkgs }}:
    let
      pkgs = import nixpkgs {{ system = "aarch64-linux"; }}; # Assuming aarch64-linux
    in
    {{
      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {{
        pname = "rust-compiler-step-{flake_step_counter:03d}";
        version = "0.1.0";

        # Capture command, args, and environment
        buildCommand = '''
          echo "Command: {command}"
          echo "Args: {args}"
          echo "Env: {json.dumps(env)}"
          # TODO: Reconstruct and execute the actual command here
        ''';
      }};
    }};
}}
"""
    with open(os.path.join(step_dir, "flake.nix"), "w") as f:
        f.write(flake_content)

def main():
    original_command = sys.argv[1]
    original_args = sys.argv[2:]
    original_env = os.environ.copy()

    # Generate Nix flake for this command
    generate_nix_flake(original_command, original_args, original_env)

    # Execute the original command
    # This part needs to be careful to avoid infinite recursion if original_command is also wrap_rust.py
    # For now, we'll assume original_command is the actual rustc/cargo
    subprocess.run([original_command] + original_args, env=original_env)

if __name__ == "__main__":
    main()
