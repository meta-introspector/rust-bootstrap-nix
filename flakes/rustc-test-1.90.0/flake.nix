{
  description = "Test flake for rustc 1.90.0";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
      rustcPath = "/nix/store/wi7qg1yc7x2hbn4yaylzs1kxhdi90i1l-rust-1.90.0-aarch64-unknown-linux-gnu/bin/rustc";
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
