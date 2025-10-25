{
  description = "Test flake for rustc 1.89.0";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustBootstrapNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=nix/rust-deps"; # Add rust-bootstrap-nix as an input with dir
  };

  outputs = { self, nixpkgs, rustBootstrapNix }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
      };
      lib = nixpkgs.lib; # Define lib here
      commonRustDeps = import rustBootstrapNix.common-rust-deps { inherit pkgs lib; }; # Update import path
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
    };
}
