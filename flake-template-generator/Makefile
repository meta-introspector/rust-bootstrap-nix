DRY_RUN ?= # Set to --dry-run for a dry run, leave empty for actual execution

.PHONY: all generate-flake

all: generate-flake

generate-flake:
	@echo "Generating flake directory using flake-template-generator..."
	$(eval COMPONENT := solana-rust-1.84.1)
	$(eval ARCH := aarch64)
	$(eval PHASE := phase0)
	$(eval STEP := step1)
	$(eval OUTPUT_DIR := flakes/$(COMPONENT)/$(ARCH)/$(PHASE)/$(STEP))
	cargo run --bin flake-template-generator -- \
		--config-path ../bootstrap-config-builder/generated_config.toml \
		--output-dir $(OUTPUT_DIR) \
		--component $(COMPONENT) \
		--arch $(ARCH) \
		--phase $(PHASE) \
		--step $(STEP) \
		$(DRY_RUN) \
		$(ARGS)
