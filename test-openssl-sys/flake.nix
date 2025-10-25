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
      commonRustDeps = commonDepsFlake.common-rust-deps;
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "test-openssl-sys";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        buildInputs = [ pkgs.pkg-config pkgs.openssl ] ++ commonRustDeps.commonBuildInputs;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath;
        OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        preBuild = ''
          export PATH=${pkgs.pkg-config}/bin:$PATH
          set -x
          echo "PATH: $PATH"
          pkg-config --version || echo "pkg-config not found in PATH"
          ls -laR ${commonRustDeps.pkgConfigPath} || true
          ls -laR ${pkgs.openssl.dev}/lib/pkgconfig || true
          ls -laR ${pkgs.openssl}/lib || true
          ls -laR ${pkgs.openssl.dev}/include || true
        '';
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
