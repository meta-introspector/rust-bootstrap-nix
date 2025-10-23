.PHONY: all generate-flake

all: generate-flake

generate-flake:
	@echo "Generating flake directory using flake-template-generator..."
	cargo run --bin flake-template-generator -- \
		--config-path ../bootstrap-config-builder/generated_config.toml \
		--output-dir ../target/generated-flake
