{
  description = "Dynamically generated config flake";

  outputs = { self }:
    let
      configTomlContent = builtins.readFile ./config.toml;
    in
    {
      packages.aarch64-linux.default = configTomlContent; # Output the string directly
    };
}
