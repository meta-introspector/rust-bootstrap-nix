{
  description = "Flake to process JSON output from rust-bootstrap-nix's standalonex flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    # Reference the standalonex flake within the rust-bootstrap-nix submodule
    standalonex = {
      url = "path:../../standalonex"; # Relative path from this flake to standalonex flake
    };
  };

  outputs = { self, nixpkgs, standalonex }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };

      # Get the output path of the standalonex flake
      standalonexOutput = standalonex.packages.aarch64-linux.default;

      # List all JSON files in the standalonex output
      jsonFiles = builtins.filter (name: builtins.match ".*\\.json" name != null) (builtins.attrNames (builtins.readDir standalonexOutput));

      # Function to read and parse a single JSON file
      readAndParseJson = filename:
        let
          jsonContent = builtins.readFile "${standalonexOutput}/${filename}";
        in
        builtins.fromJSON jsonContent;

      # Parse all JSON files
      parsedJsons = builtins.map readAndParseJson jsonFiles;

      # Debugging: Print parsedJsons and type of json.command
      _debug = builtins.trace "Parsed JSONs: ${builtins.toJSON parsedJsons}" (
        builtins.map (json: builtins.trace "Command: ${json.command}, Type: ${builtins.typeOf json.command}" json) parsedJsons
      );

    in
    let
      generatedPackages = builtins.listToAttrs (
        builtins.map (json: {
          name = json.command; # Use the 'command' field as the package name
          value = pkgs.runCommand json.command {} ''
            mkdir -p $out
            echo "--- Package for ${json.command} ---" > $out/output.txt
            echo "${builtins.toJSON json}" >> $out/output.txt
          '';
        }) parsedJsons
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
