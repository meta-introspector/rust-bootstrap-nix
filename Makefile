.PHONY: all build fast-build run-config-builder-dry-run build-config-builder generate-seed-config generate-flake-dir

all: build build-config-builder

build: generate-config
	$(MAKE) -C nix-build-scripts/

fast-build:
	$(MAKE) -C nix-build-scripts/ fast-build

build-config-builder:
	@echo "Building bootstrap-config-builder..."
	nix develop --command bash -c "cargo build --package bootstrap-config-builder"

generate-config: build-config-builder
	@echo "Generating config.toml using bootstrap-config-builder..."
	$(eval NIXPKGS_PATH := $(shell nix build .#nixpkgsOutPath --no-link --print-out-paths))
	$(eval RUST_OVERLAY_PATH := $(shell nix build .#rustOverlayOutPath --no-link --print-out-paths))
	$(eval RUST_BOOTSTRAP_NIX_PATH := $(shell nix build .#rustBootstrapNixOutPath --no-link --print-out-paths))
	$(eval CONFIGURATION_NIX_PATH := $(shell nix build .#configurationNixOutPath --no-link --print-out-paths))
	$(eval RUST_SRC_FLAKE_PATH := $(shell nix build .#rustSrcFlakeOutPath --no-link --print-out-paths))
	@RUST_LOG=debug ./target/debug/bootstrap-config-generator \
		--config-file bootstrap-config-builder/config.toml \
		--project-root $(CURDIR) \
		--output config.toml \
		--nixpkgs-path $(NIXPKGS_PATH) \
		--rust-overlay-path $(RUST_OVERLAY_PATH) \
		--rust-bootstrap-nix-path $(RUST_BOOTSTRAP_NIX_PATH) \
		--configuration-nix-path $(CONFIGURATION_NIX_PATH) \
		--rust-src-flake-path $(RUST_SRC_FLAKE_PATH)

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
	$(eval NIXPKGS_PATH := $(shell nix build .#nixpkgsOutPath --no-link --print-out-paths))
	$(eval RUST_OVERLAY_PATH := $(shell nix build .#rustOverlayOutPath --no-link --print-out-paths))
	$(eval RUST_BOOTSTRAP_NIX_PATH := $(shell nix build .#rustBootstrapNixOutPath --no-link --print-out-paths))
	$(eval CONFIGURATION_NIX_PATH := $(shell nix build .#configurationNixOutPath --no-link --print-out-paths))
	$(eval RUST_SRC_FLAKE_PATH := $(shell nix build .#rustSrcFlakeOutPath --no-link --print-out-paths))
	@RUST_LOG=debug ./target/debug/bootstrap-config-generator \
		--config-file bootstrap-config-builder/config.toml \
		--project-root $(CURDIR) \
		--output generated_config.toml \
		--nixpkgs-path $(NIXPKGS_PATH) \
		--rust-overlay-path $(RUST_OVERLAY_PATH) \
		--rust-bootstrap-nix-path $(RUST_BOOTSTRAP_NIX_PATH) \
		--configuration-nix-path $(CONFIGURATION_NIX_PATH) \
		--rust-src-flake-path $(RUST_SRC_FLAKE_PATH) \
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
		nix develop --command bash -c "cargo run --bin bootstrap-config-generator -- \\
			--config-file bootstrap-config-builder/config.toml \\
			--build-rustc-version \"$$version\" \\
			--solana-rustc-path \"$(SOLANA_RUSTC_PATH)\" \\
			--cargo-path \"$(CARGO_PATH)\" \\
			--project-root \"$(PROJECT_ROOT)\" \\
			--rust-src-flake-path \"$(RUST_SRC_FLAKE_PATH)\""; \
	done

clean-rustc-test-flakes:
	@echo "Cleaning generated rustc test flakes and configs..."
	@for version in $(RUSTC_BUILD_VERSIONS); do \
		rm -rf "flakes/$$version"; \
		echo "Cleaned flakes/$$version"; \
	done
