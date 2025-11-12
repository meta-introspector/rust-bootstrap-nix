{
  description = "Test flake for rustc 1.89.0";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
      rustcPath = "/nix/store/icixkhs20b5r5zbj4m6a4vwdvv7pncig-rustc-1.89.0/bin/rustc";
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
        ];
        RUSTC = rustcPath;
      };
    };
}
