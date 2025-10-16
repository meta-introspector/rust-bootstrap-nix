{
  description = "Minimal Python development environment";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShell = pkgs.mkShell {
        name = "minimal-python-dev-shell";
        packages = with pkgs; [ python3 git ];
      };

      packages.${system}.helloPython = pkgs.writeScriptBin "hello-python" ''
        #!${pkgs.python3}/bin/python
        print("Hello from Nix Python!")
      '';
    };
}
