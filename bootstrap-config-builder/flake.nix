{
  description = "Flake to generate config.toml for rust-bootstrap-nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=3487cd3843083db70ee30023f19344568ade9c9f";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    cargo2nix.url = "github:meta-introspector/cargo2nix?ref=feature/CRQ-016-nixify"; # Add cargo2nix input
  };

  outputs = { self, nixpkgs, rustSrcFlake, rustOverlay, cargo2nix }@inputs:
    let
      system = "aarch64-linux"; # Define system here
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rustOverlay.overlays.default cargo2nix.overlays.default ];
      };
      lib = nixpkgs.lib;
      commonRustDeps = import ../nix/rust-deps/common-rust-deps.nix { inherit pkgs lib; };
      rustcPath = "${pkgs.rust-bin.stable.1.89.0.rustc}/bin/rustc";
      cargoPath = "${pkgs.cargo}/bin/cargo";
      projectRoot = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix";
      rustSrcFlakePath = "/data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src";
    in
    {
      devShells.aarch64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.rust-analyzer
        ] ++ commonRustDeps.commonBuildInputs;
        PKG_CONFIG_PATH = commonRustDeps.pkgConfigPath;
        RUST_SRC_PATH = "${rustSrcFlake}/library";
      };

      packages.aarch64-linux.generatedConfigToml = pkgs.runCommand "generated-config.toml"
        {
          src = ./.;
          nativeBuildInputs = [ pkgs.cargo pkgs.rustc pkgs.cacert ];
        } ''
        cd $src
        export CARGO_HOME=$(mktemp -d)
        ${pkgs.cargo}/bin/cargo build --release --bin bootstrap-config-generator --target aarch64-unknown-linux-gnu

        ./target/release/bootstrap-config-generator \
          --rustc-path "${rustcPath}" \
          --cargo-path "${cargoPath}" \
          --project-root "${projectRoot}" \
          --rust-src-flake-path "${rustSrcFlakePath}" \
          --output "$out/config.toml"
      '';
    };
}
