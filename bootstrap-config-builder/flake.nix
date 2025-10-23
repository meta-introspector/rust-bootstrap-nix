{
  description = "Flake to generate config.toml for rust-bootstrap-nix";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=3487cd3843083db70ee30023f19344568ade9c9f";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rustSrcFlake, rustOverlay }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rustOverlay.overlays.default ];
      };
      rustcPath = "${pkgs.rust-bin.stable."1.89.0".default}/bin/rustc"; # Using a specific rustc version
      cargoPath = "${pkgs.cargo}/bin/cargo"; # Using cargo from nixpkgs
      projectRoot = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix"; # Absolute path to the main project root
      rustSrcFlakePath = "/data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src"; # Absolute path to rust-src
    in
    {
      packages.aarch64-linux.generatedConfigToml = pkgs.runCommand "generated-config.toml"
        {
          nativeBuildInputs = [ pkgs.cargo pkgs.rustc ]; # Ensure cargo and rustc are available
        } ''
        # Build the bootstrap-config-generator binary
        ${pkgs.cargo}/bin/cargo build --release --bin bootstrap-config-generator --target aarch64-unknown-linux-gnu

        # Run the generator to create config.toml
        ./target/release/bootstrap-config-generator \
          --rustc-path ${rustcPath} \
          --cargo-path ${cargoPath} \
          --project-root ${projectRoot} \
          --rust-src-flake-path ${rustSrcFlakePath} \
          --output $out/config.toml
      '';
    };
  }




