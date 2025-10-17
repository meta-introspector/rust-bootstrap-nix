{
  description = "Flake exposing x.py JSON output directory";

  inputs = {
    # Reference the nix_json_output directory
    nixJsonOutputDir = {
      url = "path:../../../../nix_json_output"; # Relative path from this flake to nix_json_output directory
      flake = false; # Treat it as a plain path
    };
  };

  outputs = { self, nixJsonOutputDir }:
    {
      packages.aarch64-linux.default = nixJsonOutputDir; # Expose the directory itself as a package
    };
}