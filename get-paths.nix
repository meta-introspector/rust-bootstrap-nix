{ system ? builtins.currentSystem }:

let
  nixpkgs = (builtins.getFlake "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust2").inputs.nixpkgs;
  rust-overlay = (builtins.getFlake "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/test-rust2").inputs.rust-overlay;

  pkgs = import nixpkgs {
    inherit system;
    overlays = [ rust-overlay.overlays.default ];
  };

  rustToolchain = pkgs.rustChannels.nightly.rust.override {
    targets = [
      (if system == "aarch64-linux" then "aarch64-unknown-linux-gnu" else "x86_64-unknown-linux-gnu")
    ];
  };

in {
  sccache = pkgs.sccache.outPath;
  curl = pkgs.curl.outPath;
  rustc = "${rustToolchain}/bin/rustc";
  cargo = "${rustToolchain}/bin/cargo";
}
