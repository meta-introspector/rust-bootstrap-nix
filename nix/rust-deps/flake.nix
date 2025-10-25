{
  description = "Nix Rust Dependencies";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; };
      lib = nixpkgs.lib;
    in
    {
      # Export common-rust-deps.nix as an attribute
      common-rust-deps = import ./common-rust-deps.nix { inherit pkgs lib; };

      packages.aarch64-linux.default = pkgs.runCommand "rust-deps-usage" { }
        "echo 'This flake provides common Rust dependencies. Use nix develop or access common-rust-deps attribute.' > $out";
    };
}
