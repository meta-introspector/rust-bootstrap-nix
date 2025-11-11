.PHONY: all build run clean

# Define paths and versions (can be overridden)
EXPANDED_DIR ?= ../expanded
PROJECT_ROOT ?= .
RUSTC_VERSION ?= "1.89.0" # Default from Makefile.rust_workflow
RUSTC_HOST ?= "aarch64-unknown-linux-gnu" # Default from Makefile.rust_workflow
VERBOSITY ?= 3
LAYER ?= 0

# Default target
all: build run

# Build split-expanded-lib (library)
build-lib:
	@echo "Building split-expanded-lib library..."
	cargo build --package split-expanded-lib

# Build split-expanded-bin (binary that uses the library)
build-bin: build-lib
	@echo "Building split-expanded-bin binary from workspace root..."
	$(MAKE) -C .. build-split-expanded-bin

# Alias for building both
build: build-bin

# Run split-expanded-bin with example arguments
run: build expand-self
	@echo "Running split-expanded-bin..."
	cargo run --bin split-expanded-bin -- \
		--expanded-manifest $(LOCAL_EXPANDED_DIR)/expanded_manifest.json \
		--project-root $(PROJECT_ROOT) \
		--rustc-version $(RUSTC_VERSION) \
		--rustc-host $(RUSTC_HOST) \
		--verbosity $(VERBOSITY) \
		--layer $(LAYER)

.PHONY: expand-self clean-expanded

# Define paths for expanded-code-collector
LOCAL_METADATA_FILE := target/cargo_metadata.json
LOCAL_EXPANDED_DIR := expanded
FLAKE_LOCK_PATH := ../flake.lock # Assuming flake.lock is in the root

expand-self:
	@echo "Building expanded-code-collector..."
	cargo build --package expanded-code-collector

	@echo "Collecting metadata for split-expanded-lib..."
	mkdir -p $(dir $(LOCAL_METADATA_FILE))
	cargo metadata --format-version 1 --manifest-path Cargo.toml > $(LOCAL_METADATA_FILE)

	@echo "Running expanded-code-collector for split-expanded-lib..."
	cargo run -p expanded-code-collector -- \
		--metadata-path $(CURDIR)/$(LOCAL_METADATA_FILE) \
		--output-dir $(CURDIR)/$(LOCAL_EXPANDED_DIR) \
		--flake-lock-path $(CURDIR)/$(FLAKE_LOCK_PATH) \
		--layer 0 \
		--package split-expanded-lib

clean-expanded:
	@echo "Cleaning local expanded directory..."
	rm -rf $(LOCAL_EXPANDED_DIR)

# Clean generated files and build artifacts
clean: clean-expanded
	@echo "Cleaning build artifacts and generated files..."
	cargo clean --package split-expanded-lib
	cargo clean --package split-expanded-bin
	rm -rf $(PROJECT_ROOT)/rust-bootstrap-core/src/level_*
