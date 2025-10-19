{
  description = "Flake to process JSON output from rust-bootstrap-nix's standalonex flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    # Reference the xpy-json-output-flake directly
    xpyJsonOutputFlake = {
      url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=flakes/xpy-json-output-flake";
    };
    # Reference the main Rust source code
    rustSrc = {
      url = "github:meta-introspector/rust?ref=feature/CRQ-016-nixify";
    };
    # Reference the evaluate-rust flake
    evaluateRustFlake = {
      url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=flakes/evaluate-rust"; # Reference the evaluate-rust flake
    };
  };

  outputs = { self, nixpkgs, xpyJsonOutputFlake, rustSrc, evaluateRustFlake }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };

      # Get the output path from xpyJsonOutputFlake
      jsonOutputContent = xpyJsonOutputFlake.packages.aarch64-linux.default;

      # List all JSON files in the jsonOutput
      jsonFiles = builtins.filter (name: builtins.match ".*\\.json" name != null) (builtins.attrNames (builtins.readDir jsonOutputContent));

      # Function to read and parse a single JSON file
      readAndParseJson = filename:
        let
          jsonContent = builtins.readFile "${jsonOutputContent}/${filename}";
        in
        builtins.fromJSON jsonContent;

      # Parse all JSON files
      parsedJsons = builtins.map readAndParseJson jsonFiles;

      # Parse all JSON files and evaluate commands
      evaluatedPackages = builtins.concatLists (
        builtins.map
          (json: evaluateRustFlake.lib.evaluateCommand {
            commandInfo = json;
            rustSrc = rustSrc;
            currentDepth = 0;
            maxDepth = 8;
          })
          parsedJsons
      );

    in
    let
      generatedPackages = builtins.listToAttrs (
        builtins.map
          (drv: {
            name = drv.name; # Assuming the derivation has a 'name' attribute
            value = drv;
          })
          evaluatedPackages
      );
    in
    {
      packages.aarch64-linux = generatedPackages // {
        default = pkgs.symlinkJoin {
          name = "all-processed-jsons";
          paths = builtins.attrValues generatedPackages;
        };
      };
    };
}
