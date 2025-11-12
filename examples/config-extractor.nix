# config-extractor.nix
{ lib, pkgs, ... }:

{
  # A function to extract and process configuration from a TOML file.
  # configFilePath: Path to the config.toml file.
  # extraConfig: Optional attribute set for overriding config values.
  extractConfig = { configFilePath, extraConfig ? { } }:
    let
      # Read the TOML file content
      tomlContent = builtins.readFile configFilePath;

      # Parse the TOML content into a Nix attribute set
      # This assumes the TOML structure is simple enough for builtins.fromTOML
      # For more complex TOML, a custom parser or a Rust tool might be needed.
      parsedToml = builtins.fromTOML tomlContent;

      # Merge parsed TOML with extraConfig, with extraConfig taking precedence
      finalConfig = lib.recursiveUpdate parsedToml extraConfig;

      # Basic validation (example: check for a 'build' attribute)
      validatedConfig =
        if builtins.hasAttr "build" finalConfig
        then finalConfig
        else throw "Configuration missing 'build' section";

    in
    validatedConfig;
}
