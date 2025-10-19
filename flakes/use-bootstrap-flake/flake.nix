{
  description = "A flake to use the built bootstrap binary";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    standalonex = {
      url = "path:../../standalonex";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, standalonex }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        name = "use-bootstrap-dev-shell";

        packages = [
          standalonex.packages.aarch64-linux.default # The built bootstrap binary
        ];

        shellHook = ''
          export PATH=${standalonex.packages.aarch64-linux.default}/bin:$PATH
          echo "Bootstrap binary is available in your PATH."
        '';
      };
    };
}
