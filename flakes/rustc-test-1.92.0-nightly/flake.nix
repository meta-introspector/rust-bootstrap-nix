{
  description = "Test flake for rustc 1.92.0-nightly-2025-10-16";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
      rustcPath = "/nix/store/8zs48kgz8i529l2x8xgv0fhik4sr2b0j-rust-1.92.0-nightly-2025-10-16-aarch64-unknown-linux-gnu/bin/rustc";
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
