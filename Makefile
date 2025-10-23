.PHONY: all build fast-build run-config-builder-dry-run

all: build

build:
	$(MAKE) -C nix-build-scripts/

fast-build:
	$(MAKE) -C nix-build-scripts/ fast-build

run-config-builder-dry-run:
	@echo "Running bootstrap-config-builder in dry-run mode..."
	$(eval NIXPKGS_PATH := $(shell nix build .#nixpkgsOutPath --no-link --print-out-paths))
	$(eval RUST_OVERLAY_PATH := $(shell nix build .#rustOverlayOutPath --no-link --print-out-paths))
	$(eval RUST_BOOTSTRAP_NIX_PATH := $(shell nix build .#rustBootstrapNixOutPath --no-link --print-out-paths))
	$(eval CONFIGURATION_NIX_PATH := $(shell nix build .#configurationNixOutPath --no-link --print-out-paths))
	$(eval RUST_SRC_FLAKE_PATH := $(shell nix build .#rustSrcFlakeOutPath --no-link --print-out-paths))
	@RUST_LOG=debug ./target/debug/bootstrap-config-builder 0 aarch64-unknown-linux-gnu \
		--project-root $(CURDIR) \
		--system aarch64-linux \
		--output generated_config.toml \
		--rust-bootstrap-nix-flake-ref "github:meta-introspector/rust-bootstrap-nix?rev=e1215ab7f9aa7674c57155c59bfc6ed2c1d10e14" \
		--rust-src-flake-ref "github:meta-introspector/rust?rev=e6c1b92d0abaa3f64032d6662cbcde980c826ff2" \
		--nixpkgs-path $(NIXPKGS_PATH) \
		--rust-overlay-path $(RUST_OVERLAY_PATH) \
		--rust-bootstrap-nix-path $(RUST_BOOTSTRAP_NIX_PATH) \
		--configuration-nix-path $(CONFIGURATION_NIX_PATH) \
		--rust-src-flake-path $(RUST_SRC_FLAKE_PATH) \
		--dry-run
