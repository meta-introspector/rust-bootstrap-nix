#!/bin/sh
exec nix develop --override-input nixpkgs github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify "$@"
