.PHONY: all build

all: build

build:
	@echo "Building generated config.toml..."
	nix build ..#generatedConfigToml
	@echo "Generated config.toml is at $(shell nix build ..#generatedConfigToml --no-link --print-out-paths)"
	@echo "Now building the main project using the generated config.toml..."
	# Placeholder for main project build using the generated config.toml
	# This will be a new Nix derivation that depends on generatedConfigToml
