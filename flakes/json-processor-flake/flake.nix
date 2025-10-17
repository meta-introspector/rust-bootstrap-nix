{
  description = "Flake to process JSON output from rust-bootstrap-nix's standalonex flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    # Reference the standalonex flake within the rust-bootstrap-nix submodule
    standalonex = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=standalonex";
    };
  };

  outputs = { self, nixpkgs, standalonex }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };
      # Access the xpy_json_output.json from the standalonex default package
      jsonFile = "${standalonex.packages.aarch64-linux.default}/xpy_json_output.json";
      jsonContent = builtins.readFile jsonFile;
      parsedJson = builtins.fromJSON jsonContent;
    in
    {
      packages.aarch64-linux.default = pkgs.runCommand "processed-json-output" { } ''
        echo "--- Parsed JSON Output ---" > $out/output.txt
        echo "${builtins.toJSON parsedJson}" >> $out/output.txt
      '';
    };
}
