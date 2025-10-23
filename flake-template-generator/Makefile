.PHONY: all generate-flake

all: generate-flake

generate-flake:
	@echo "Generating flake directory using flake-template-generator..."
	cargo run --bin flake-template-generator -- \
		--config-path ../bootstrap-config-builder/generated_config.toml \
		--output-dir flakes/generated-config-flakes \
		--component solana-rust-1.83 \
		--arch aarch64 \
		--phase phase0 \
		--step step1 \
		$(ARGS)
