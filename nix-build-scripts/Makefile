SHELL := /nix/store/hxmi7d6vbdgbzklm4icfk2y83ncw8la9-bash-5.3p3/bin/bash

.PHONY: build generate-configuration-nix-lock fast-build

build: generate-configuration-nix-lock
	@echo "Updating Nix flake lock file..."
	nix flake update
	@echo "Now building the main project (standalonex) using the generated config.toml..."
	nix build ../standalonex

fast-build: generate-configuration-nix-lock
	@echo "Building generated config.toml (without flake update)..."
	nix build ..#generatedConfigToml
	@echo "Generated config.toml is at $(shell nix build ..#generatedConfigToml --no-link --print-out-paths)"
	@echo "Now building the main project using the generated config.toml..."
	@echo "Now building the main project (standalonex) using the generated config.toml..."
	nix build ../standalonex
