{
  description = "Flake to read and process JSON output from rust-bootstrap-nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustBootstrapNix.url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001";
  };

  outputs = { self, nixpkgs, rustBootstrapNix }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };
      # Access the xpy_json_output.json from the rustBootstrapNix default package
      jsonFile = "${rustBootstrapNix.packages.aarch64-linux.default}/xpy_json_output.json";
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
