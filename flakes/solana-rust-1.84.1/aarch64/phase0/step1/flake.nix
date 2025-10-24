{
  description = "Dynamically generated config flake";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };
      configTomlContent = builtins.readFile ./config.toml;
    in
    {
      packages.aarch64-linux.default = pkgs.writeText "config.toml" configTomlContent;
    };
}
