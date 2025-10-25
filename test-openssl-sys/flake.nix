{
  description = "A standalone Rust test crate using openssl-sys";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    commonDepsFlake.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/1.84.1-aarch64-config-generation-reference-config&dir=nix/rust-deps";
  };

  outputs = { self, nixpkgs, rust-overlay, commonDepsFlake, ... }@inputs:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      lib = nixpkgs.lib;
      commonRustDeps = commonDepsFlake.common-rust-deps { inherit pkgs lib; };
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "test-openssl-sys";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        buildInputs = commonRustDeps.commonBuildInputs;
      };

      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          (pkgs.rust-bin.stable.latest.default)
        ] ++ commonRustDeps.commonBuildInputs;
        shellHook = ''
          export PKG_CONFIG_PATH="${commonRustDeps.pkgConfigPath}:$PKG_CONFIG_PATH"
        '';
      };
    };
}
