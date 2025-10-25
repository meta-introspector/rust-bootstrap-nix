{
  description = "Test flake for rustc 1.89.0";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustBootstrapNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/1.84.1-aarch64-config-generation-reference-config&dir=nix/rust-deps"; # Update ref to current branch
  };

  outputs = { self, nixpkgs, rustBootstrapNix }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
      lib = nixpkgs.lib; # Define lib here
      commonRustDeps = rustBootstrapNix.common-rust-deps; # Update import path
      # This rustcPath is the *source* rustc used to build the next stage
      rustcPath = "${pkgs.rust-bin.stable."1.89.0".default}/bin/rustc";
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc # Added pkgs.rustc
        ] ++ commonRustDeps.commonBuildInputs; # Added commonRustDeps
        RUSTC = rustcPath;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath; # Added PKG_CONFIG_PATH
      };

      packages.aarch64-linux.default = pkgs.runCommand "stage0-step1-configure-usage" { }
        "echo 'This flake provides a devShell for configuring stage 0 step 1. Use nix develop.' > $out";
    };
}
