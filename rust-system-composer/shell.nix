{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    (import
      (builtins.fetchGit {
        url = "https://github.com/oxalica/rust-overlay";
        ref = "master";
      })
      { inherit pkgs; }).rust-bin.nightly.latest.default
  ];
}
