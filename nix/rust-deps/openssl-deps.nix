{ pkgs, lib, ... }:

let
  # Build inputs specifically for OpenSSL
  opensslBuildInputs = [
    pkgs.openssl
  ]
  ++ (lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.apple_sdk.frameworks.Security ]);

  # PKG_CONFIG_PATH for OpenSSL development files
  opensslPkgConfigPath = "${pkgs.openssl.dev}/lib/pkgconfig";

in
{
  inherit opensslBuildInputs opensslPkgConfigPath;
}
