{
  description = "Test flake for rustc 1.84.1";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
      rustcPath = "/nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/bin/rustc";
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
        ];
        RUSTC = rustcPath;
        # Optionally, you can add other tools or environment variables here
        # For example, to ensure cargo uses this rustc:
        # CARGO_HOME = "${pkgs.stdenv.mkDerivation { name = "cargo-home"; src = null; buildInputs = [ pkgs.cargo ]; installPhase = "mkdir -p $out"; }}/.cargo";
        # PATH = "${pkgs.cargo}/bin:${pkgs.rustc}/bin:${pkgs.stdenv.mkDerivation { name = "rustc-bin"; src = null; buildInputs = [ pkgs.rustc ]; installPhase = "mkdir -p $out/bin; ln -s ${rustcPath} $out/bin/rustc"; }}/bin:${pkgs.lib.makeBinPath pkgs.buildInputs}";
      };
    };
}
