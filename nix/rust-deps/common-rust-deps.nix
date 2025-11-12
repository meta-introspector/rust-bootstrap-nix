{ pkgs, lib, ... }:

let
  # Inlined content from openssl-deps.nix
  # Build inputs specifically for OpenSSL
  opensslBuildInputs = [
    pkgs.openssl
  ]
  ++ (lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.apple_sdk.frameworks.Security ]);

  # PKG_CONFIG_PATH for OpenSSL development files
  opensslPkgConfigPath = "${pkgs.openssl.dev}/lib/pkgconfig";

  commonBuildInputs = opensslBuildInputs ++ [
    pkgs.pkg-config
  ];

  pkgConfigPath = opensslPkgConfigPath;

in
{
  inherit commonBuildInputs pkgConfigPath;
}
