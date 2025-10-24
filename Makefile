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
	@RUST_LOG=debug ./target/debug/bootstrap-config-builder 0 aarch64-unknown-linux-gnu \
		--project-root $(CURDIR) \
		--system aarch64-linux \
		--output config.toml \
		--rust-bootstrap-nix-flake-ref "github:meta-introspector/rust-bootstrap-nix?ref=feature/CRQ-016-nixify" \
		--rust-src-flake-ref "github:meta-introspector/rust?ref=feature/CRQ-016-nixify" \
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

# --- Targets for generating rustc test flakes and configs ---

SOLANA_RUSTC_PATH = /nix/store/b29wwnvfjfzkf23l2d077nmw5cncaz5s-rustc-1.84.1-aarch64-unknown-linux-gnu/bin/rustc

# Build rustc versions (selected from available versions)
RUSTC_BUILD_VERSIONS = \
	1.89.0:/nix/store/icixkhs20b5r5zbj4m6a4vwdvv7pncig-rustc-1.89.0/bin/rustc \
	1.90.0:/nix/store/wi7qg1yc7x2hbn4yaylzs1kxhdi90i1l-rust-1.90.0-aarch64-unknown-linux-gnu/bin/rustc \
	1.92.0-nightly:/nix/store/8zs48kgz8i529l2x8xgv0fhik4sr2b0j-rust-1.92.0-nightly-2025-10-16-aarch64-unknown-linux-gnu/bin/rustc

CARGO_PATH = /nix/store/ahyjafkgyn6zji9qlvv92z8gxmcmaky4-cargo-1.89.0/bin/cargo
PROJECT_ROOT = $(CURDIR)
RUST_SRC_FLAKE_PATH = /data/data/com.termux.nix/files/home/nix/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src

.PHONY: generate-rustc-test-flakes generate-rustc-test-configs clean-rustc-test-flakes

generate-rustc-test-flakes:
	@echo "Generating rustc test flakes..."
	@for entry in $(RUSTC_BUILD_VERSIONS); do \
		version=$$(echo $$entry | cut -d':' -f1); \
		rustc_path=$$(echo $$entry | cut -d':' -f2); \
		dir="flakes/$$version/aarch64-linux/stage0/step1-configure"; \
		mkdir -p $$dir; \
		echo "{\n  description = \"Test flake for rustc $$version\";\n\n  inputs = {\n    nixpkgs.url = \"github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify\";\n  };\n\n  outputs = { self, nixpkgs }:\n    let\n      pkgs = import nixpkgs {\n        system = \"aarch64-linux\";\n      };\n      # This rustcPath is the *source* rustc used to build the next stage\n      rustcPath = \"$(SOLANA_RUSTC_PATH)\";\n    in\n    {\n      devShells.aarch64-linux.default = pkgs.mkShell {\n        buildInputs = [\n          pkgs.cargo\n        ];\n        RUSTC = rustcPath;\n      };\n    };\n}" > $$dir/flake.nix; \
		echo "Generated $$dir/flake.nix for rustc $$version"; \
	done

generate-rustc-test-configs: build-config-builder
	@echo "Generating rustc test configs..."
	@for entry in $(RUSTC_BUILD_VERSIONS); do \
		version=$$(echo $$entry | cut -d':' -f1); \
		rustc_path=$$(echo $$entry | cut -d':' -f2); \
		dir="flakes/$$version/aarch64-linux/stage0/step1-configure"; \
		output_file="$$dir/generated_config_$$version.toml"; \
		echo "Generating config for rustc $$version to $$output_file"; \
		nix develop --command bash -c "cargo run --bin bootstrap-config-generator -- \\\n			--rustc-path \"$$rustc_path\" \\\n			--cargo-path \"$(CARGO_PATH)\" \\\n			--project-root \"$(PROJECT_ROOT)\" \\\n			--rust-src-flake-path \"$(RUST_SRC_FLAKE_PATH)\" \\\n			--output \"$$output_file\""; \
		done

clean-rustc-test-flakes:
	@echo "Cleaning generated rustc test flakes and configs..."
	@for entry in $(RUSTC_BUILD_VERSIONS); do \
		version=$$(echo $$entry | cut -d':' -f1); \
		rm -rf "flakes/$$version"; \
		echo "Cleaned flakes/$$version"; \
		done
