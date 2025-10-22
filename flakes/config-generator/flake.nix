{
  description = "A virtual Nix package to generate rust-bootstrap-nix config.toml files";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustBootstrapNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify";
    configurationNix.url = "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify&dir=configuration-nix";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, flake-utils, rustBootstrapNix, configurationNix, rust-overlay } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        lib = pkgs.lib;

        # Helper function to get rustToolchain for a specific system
        getRustToolchain = system: pkgs:
          pkgs.rustChannels.nightly.rust.override {
            targets = [ (if system == "aarch64-linux" then "aarch64-unknown-linux-gnu" else "x86_64-unknown-linux-gnu") ];
          };

        # Replicate the generateConfigTomlForStage logic from the main flake.nix
        generateConfigTomlForStage = { system, stageNum, targetTriple, extraConfig ? { } }:
          let
            rustToolchain = getRustToolchain system pkgs;
          in
          pkgs.runCommand "config-stage-${toString stageNum}-${targetTriple}.toml"
            {
              nativeBuildInputs = [ configurationNix.packages.${system}.default ]; # Only configurationNix is needed
            }
            ''
              ${configurationNix.packages.${system}.default}/bin/configuration-nix "${toString stageNum}" "${targetTriple}"
              mv config.toml $out
            '';

        configGeneratorScript = pkgs.writeShellScript "config-generator-app" ''
          stageNum="$1"
          targetTriple="$2"
          if [ -z "$stageNum" ] || [ -z "$targetTriple" ]; then
            echo "Usage: $0 <stage_number> <target_triple>"
            exit 1
          fi
          
          # Call the configuration-nix executable directly
          ${configurationNix.packages.${system}.default}/bin/configuration-nix "$stageNum" "$targetTriple"
        '';
