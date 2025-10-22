.PHONY: all build fast-build run-config-builder-dry-run

all: build

build:
	$(MAKE) -C nix-build-scripts/

fast-build:
	$(MAKE) -C nix-build-scripts/ fast-build

run-config-builder-dry-run:
	@echo "Running bootstrap-config-builder in dry-run mode..."
	@RUST_LOG=debug ./target/debug/bootstrap-config-builder 0 aarch64-unknown-linux-gnu \
		--project-root $(CURDIR) \
		--system aarch64-linux \
		--output generated_config.toml \
		--rust-bootstrap-nix-flake-ref "github:meta-introspector/rust-bootstrap-nix?rev=e1215ab7f9aa7674c57155c59bfc6ed2c1d10e14" \
		--rust-src-flake-ref "github:meta-introspector/rust?rev=e6c1b92d0abaa3f64032d6662cbcde980c826ff2" \
		--dry-run
