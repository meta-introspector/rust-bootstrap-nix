{
  description = "Standalone x.py environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    # Reference the original rust-src directory
    rustSrc.url = "path:../../.."; # Relative path from test-rust2/standalonex to vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src
  };

  outputs = { self, nixpkgs, rustSrc }:
    let
      pkgs = import nixpkgs { system = "aarch64-linux"; }; # Assuming aarch64-linux for now
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        name = "standalonex-dev-shell";

        packages = [
          pkgs.python3
        ];

        shellHook = ''
          # Change to the rust-src directory before running x.py
          cd ${rustSrc}
          echo "Now in: $(pwd)"
          echo "x.py is available and will be run from its original location."
        '';
      };
    };
}