{
  description = "Flake for evaluating Rust build commands and generating Nix packages recursively.";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    naersk.url = "github:meta-introspector/naersk?ref=feature/CRQ-016-nixify"; # For rust2nix functionality
  };

  outputs = { self, nixpkgs, naersk }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };

      # The core recursive function
      evaluateCommand = { commandInfo, rustSrc, currentDepth, maxDepth }:
        if currentDepth >= maxDepth then
          # Base case: recursion limit reached
          [ (pkgs.runCommand "recursion-limit-reached" {} ''
              echo "Recursion limit reached for command: ${commandInfo.command}" > $out/output.txt
            '') ]
        else if commandInfo.command == "cargo" && builtins.elem "build" commandInfo.args then
          # Case: cargo build command - integrate naersk
          [ (naersk.lib.${pkgs.system}.buildPackage {
              pname = "cargo-build-${commandInfo.command}-${builtins.substring 0 8 (builtins.hashString "sha256" (builtins.toJSON commandInfo))}";
              version = "0.1.0"; # Generic version
              src = rustSrc;
              # Pass cargo arguments directly to naersk
              cargoBuildFlags = commandInfo.args;
              # Pass environment variables directly to the build
              env = commandInfo.env;
            }) ]
        else
          # Case: other commands (e.g., rustc directly)
          [ (pkgs.runCommand "simple-command-${commandInfo.command}" {
              src = rustSrc;
              # Pass the environment variables directly
              env = commandInfo.env;
            } ''
              mkdir -p $out
              # Execute the command
              ${commandInfo.command} ${builtins.concatStringsSep " " commandInfo.args} > $out/output.txt 2>&1
            '') ]
        ;
    in
    {
      lib = {
        inherit evaluateCommand;
      };
    };
}