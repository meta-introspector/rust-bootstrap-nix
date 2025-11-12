{
  description = "Test flake for rustc 1.89.0";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustBootstrapNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/1.84.1-aarch64-config-generation-reference-config&dir=nix/rust-deps"; # Update ref to current branch
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify"; # Add rust-overlay input
  };

  outputs = { self, nixpkgs, rustBootstrapNix, rustOverlay }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rustOverlay.overlays.default ]; # Apply rust-overlay
      };
      lib = nixpkgs.lib; # Define lib here
      commonRustDeps = rustBootstrapNix.common-rust-deps; # Update import path
      # This rustcPath is the *source* rustc used to build the next stage
      rustcPath = "${pkgs.rust-bin.stable."1.89.0".default}/bin/rustc";
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-bin.stable."1.89.0".default # Provide the entire toolchain
        ] ++ commonRustDeps.commonBuildInputs; # Added commonRustDeps
        RUSTC = rustcPath;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath; # Added PKG_CONFIG_PATH
        shellHook = ''
          export PATH=$PATH:${pkgs.rust-bin.stable."1.89.0".default}/bin
        '';
      };

      packages.aarch64-linux.default = pkgs.runCommand "stage0-step1-configure-usage" { }
        "echo 'This flake provides a devShell for configuring stage 0 step 1. Use nix develop.' > $out";
    };
}
