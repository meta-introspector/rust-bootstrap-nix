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
      jsonFiles = builtins.filter (name: builtins.match ".*\\.json" name != null) (builtins.readDir standalonexOutput);

      # Function to read and parse a single JSON file
      readAndParseJson = filename:
        let
          jsonContent = builtins.readFile "${standalonexOutput}/${filename}";
        in
        builtins.fromJSON jsonContent;

      # Parse all JSON files
      parsedJsons = builtins.map readAndParseJson jsonFiles;

    in
    {
      packages.aarch64-linux.default = pkgs.runCommand "processed-json-output" {} ''
        echo "--- Parsed JSON Output ---" > $out/output.txt
        ${builtins.concatStringsSep "\n" (builtins.map (json: "echo \"${builtins.toJSON json}\"" ) parsedJsons)} >> $out/output.txt
      '';
    };
}
