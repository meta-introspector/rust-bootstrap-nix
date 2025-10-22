.PHONY: all build generate-configuration-nix-lock fast-build

all: build

build: generate-configuration-nix-lock
	@echo "Updating Nix flake lock file..."
	nix flake update
	@echo "Building generated config.toml..."
	nix build ..#generatedConfigToml
	@echo "Generated config.toml is at $(shell nix build ..#generatedConfigToml --no-link --print-out-paths)"
	@echo "Now building the main project using the generated config.toml..."
	# Placeholder for main project build using the generated config.toml
	# This will be a new Nix derivation that depends on generatedConfigToml

fast-build: generate-configuration-nix-lock
	@echo "Building generated config.toml (without flake update)..."
	nix build ..#generatedConfigToml
	@echo "Generated config.toml is at $(shell nix build ..#generatedConfigToml --no-link --print-out-paths)"
	@echo "Now building the main project using the generated config.toml..."
	# Placeholder for main project build using the generated config.toml
	# This will be a new Nix derivation that depends on generatedConfigToml
