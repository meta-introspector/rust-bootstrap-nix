{
  description = "Flake to process JSON output from rust-bootstrap-nix's standalonex flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    # Reference the rust-bootstrap-nix repository
    
    rustBootstrapNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=8bbbceb41f915448d6ad2a81baa1c61802c6f90f";
#    rustBootstrapNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001";
      #url = "github:meta-introspector/rust-bootstrap-nix?ref=";
    #};
    # Reference the main Rust source code
   rustSrc.url = "github:meta-introspector/rust?ref=e6c1b92d0abaa3f64032d6662cbcde980c826ff2";
#    rustSrc = {
#      url = "path:../../../../"; # Relative path from this flake to the main Rust source code
#      flake = false; # Treat it as a plain path
#    };
    # Reference the evaluate-rust flake
    evaluateRustFlake = {
      url = "path:../evaluate-rust"; # Relative path from this flake to evaluate-rust flake
    };
  };

  outputs = { self, nixpkgs, rustBootstrapNix, rustSrc, evaluateRustFlake }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };

      # Get the output path from xpy-json-output-flake within rustBootstrapNix
      jsonOutputContent = rustBootstrapNix.flakes.xpy-json-output-flake.packages.aarch64-linux.default;

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
        builtins.map (json: evaluateRustFlake.lib.evaluateCommand {
          commandInfo = json;
          rustSrc = rustSrc;
          currentDepth = 0;
          maxDepth = 8;
        }) parsedJsons
      );

    in
    let
      generatedPackages = builtins.listToAttrs (
        builtins.map (drv: {
          name = drv.name; # Assuming the derivation has a 'name' attribute
          value = drv;
        }) evaluatedPackages
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
