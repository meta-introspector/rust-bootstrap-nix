{
  description = "Test flake for local rust-bootstrap-nix mirror";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rustOverlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrc.url = "github:meta-introspector/rust?ref=d772ccdfd1905e93362ba045f66dad7e2ccd469b";
    naersk.url = "github:meta-introspector/naersk?ref=feature/CRQ-016-nixify";

    # Local mirror references
    rustBootstrapNix = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rustOverlay";
      inputs.rustSrcFlake.follows = "rustSrc";
    };

    rustBootstrapNixConfig = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=flakes/config";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rustBootstrapNix.follows = "rustBootstrapNix";
    };

    rustBootstrapNixXpyJsonOutputFlake = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=flakes/xpy-json-output-flake";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rustSrc.follows = "rustSrc";
    };

    rustBootstrapNixJsonProcessor = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=flakes/json-processor";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rustSrc.follows = "rustSrc";
      inputs.xpyJsonOutputFlake.follows = "rustBootstrapNixXpyJsonOutputFlake";
      inputs.evaluateRustFlake.follows = "rustBootstrapNixEvaluateRustFlake";
    };

    rustBootstrapNixEvaluateRustFlake = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=flakes/evaluate-rust";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.naersk.follows = "naersk";
    };

    rustBootstrapNixStandalonex = {
      url = "git+file:///data/data/com.termux.nix/files/home/git/meta-introspector/rust-bootstrap-nix?ref=feature/bootstrap-001&dir=standalonex";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rustSrcFlake.follows = "rustSrc";
      inputs.rustOverlay.follows = "rustOverlay";
    };
  };

  outputs =
    { self
    , nixpkgs
    , rustOverlay
    , rustSrc
    , naersk
    , rustBootstrapNix
    , rustBootstrapNixConfig
    , rustBootstrapNixXpyJsonOutputFlake
    , rustBootstrapNixJsonProcessor
    , rustBootstrapNixEvaluateRustFlake
    , rustBootstrapNixStandalonex
    }:
    let
      system = "aarch64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "local-bootstrap-test-shell";
        packages = [
          # Example: bring in the default package from the main rust-bootstrap-nix flake
          rustBootstrapNix.packages.${system}.default
          rustBootstrapNixConfig.packages.${system}.default
          rustBootstrapNixXpyJsonOutputFlake.packages.${system}.default
          rustBootstrapNixJsonProcessor.packages.${system}.default
          rustBootstrapNixStandalonex.packages.${system}.default
        ];
        shellHook = ''
          echo "Welcome to the local-bootstrap-test-shell!"
          echo "You can now access packages from the local rust-bootstrap-nix mirror."
        '';
      };
    };
}
