.PHONY: all build fast-build run-config-builder-dry-run build-config-builder generate-seed-config generate-flake-dir shear-all clean-shear expand-all clean-expand generate-use-statements-test-file check-rust-decl-splitter run-decl-splitter clean-decl-splitter build-decl-splitter quick-decl-splitter-check run-prelude-generator clean-prelude-generator build-prelude-generator

all: build build-config-builder
	$(MAKE) -C nix-build-scripts/

fast-build:
	$(MAKE) -C nix-build-scripts/ fast-build

build-config-builder:
	@echo "Building bootstrap-config-builder..."
	nix develop --command bash -c "cargo build --package bootstrap-config-builder"

generate-config: build-config-builder
	@echo "Generating config.toml using bootstrap-config-builder..."
	@RUST_LOG=debug ./target/debug/bootstrap-config-generator \
		--config-file bootstrap-config-builder/config.toml \
		--project-root $(CURDIR) \
		--output config.toml


generate-seed-config: build-config-builder
	@echo "Generating seed config.toml using bootstrap-config-generator..."
	cargo run --bin bootstrap-config-generator -- \
		--output bootstrap-config-builder/generated_config.toml \
		--project-root $(CURDIR) \
		--rust-src-flake-path /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src

generate-flake-dir:
	$(MAKE) -C flake-template-generator generate-flake

generate-seed-config: build-config-builder
	@echo "Generating seed config.toml using bootstrap-config-generator..."
	cargo run --bin bootstrap-config-generator -- \
		--output bootstrap-config-builder/generated_config.toml \
		--project-root $(CURDIR) \
		--rust-src-flake-path /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src

run-config-builder-dry-run:
	@echo "Running bootstrap-config-builder in dry-run mode..."
	@RUST_LOG=debug ./target/debug/bootstrap-config-generator \
		--config-file bootstrap-config-builder/config.toml \
		--project-root $(CURDIR) \
		--output generated_config.toml \
		--dry-run

# --- Targets for generating rustc test flakes and configs ---

SOLANA_RUSTC_PATH = /nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/bin/rustc

# Build rustc versions (selected from available versions)
RUSTC_BUILD_VERSIONS = \
	1.89.0 \
	1.90.0 \
	1.92.0-nightly

CARGO_PATH = /nix/store/ahyjafkgyn6zji9qlvv92z8gxmcmaky4-cargo-1.89.0/bin/cargo
PROJECT_ROOT = $(CURDIR)
RUST_SRC_FLAKE_PATH = /data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src

.PHONY: generate-rustc-test-lattice clean-rustc-test-flakes

generate-rustc-test-lattice: build-config-builder
	@echo "Generating rustc test lattice (flakes and configs)..."
	@for version in $(RUSTC_BUILD_VERSIONS); do \
		echo "Generating artifacts for rustc $$version"; \
		nix develop --command bash -c "cargo run --bin bootstrap-config-generator -- \
			--config-file bootstrap-config-builder/config.toml \
			--build-rustc-version \"$$version\" \
			--solana-rustc-path \"$(SOLANA_RUSTC_PATH)\" \
			--cargo-path \"$(CARGO_PATH)\" \
			--project-root \"$(PROJECT_ROOT)\" \
			--rust-src-flake-path \"$(RUST_SRC_FLAKE_PATH)\"" \
	; \
	done

clean-rustc-test-flakes:
	@echo "Cleaning generated rustc test flakes and configs..."
	@for version in $(RUSTC_BUILD_VERSIONS); do \
		rm -rf "flakes/$$version"; \
		echo "Cleaned flakes/$$version"; \
	done

shear-all:
	@echo "Running cargo shear on all packages via Makefile.shear..."
	$(MAKE) -f Makefile.shear shear-all

clean-shear:
	@echo "Cleaning shear processed stamps via Makefile.shear..."
	$(MAKE) -f Makefile.shear clean-shear

expand-all:
	@echo "Running cargo expand on all packages via Makefile.expand..."
	$(MAKE) -f Makefile.expand expand-all

clean-expand:
	@echo "Cleaning expand processed stamps and output files via Makefile.expand..."
	$(MAKE) -f Makefile.expand clean-expand

generate-use-statements-test-file: generated/use_statement_tests/.all-use-statements-generated-stamp

generated/use_statement_tests/.all-use-statements-generated-stamp:
	@echo "Generating all_use_statements.rs..."
	mkdir -p generated/use_statement_tests
	nix develop --command bash -c "cargo run --package prelude-generator -- --generate-aggregated-test-file"
	@touch $@

check-rust-decl-splitter:
	@echo "Running cargo check for rust-decl-splitter..."
	nix develop --command bash -c "cargo check -p rust-decl-splitter"

include Makefile.prelude