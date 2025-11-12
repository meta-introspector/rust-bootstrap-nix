{
  description = "A flake that builds the bootstrap compiler from JSON data";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rust-bootstrap-nix = {
      url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, rust-bootstrap-nix }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };

      bootstrapBuildPlan = {
        command = "${pkgs.rust-bin.stable.latest.default}/bin/cargo";
        args = [
          "build"
          "--manifest-path"
          "${rust-bootstrap-nix}/standalonex/src/bootstrap/Cargo.toml"
        ];
      };

    in
    {
      packages.aarch64-linux.default = pkgs.stdenv.mkDerivation {
        pname = "bootstrap-from-json";
        version = "0.1.0";

        src = rust-bootstrap-nix;

        nativeBuildInputs = [ pkgs.rust-bin.stable.latest.default ];

        buildPhase = ''
          export HOME=$TMPDIR
          export CARGO_HOME=$TMPDIR/.cargo
          mkdir -p $CARGO_HOME
          ${bootstrapBuildPlan.command} ${builtins.concatStringsSep " " bootstrapBuildPlan.args}
        '';

        installPhase = ''
          mkdir -p $out/bin
          cp target/debug/bootstrap $out/bin/
        '';
      };
    };
}
