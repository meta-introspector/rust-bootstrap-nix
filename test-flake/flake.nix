{
  description = "Minimal test flake";

  outputs = { self }:
    {
      packages.aarch64-linux.default = "Hello, Nix!";
    };
}
