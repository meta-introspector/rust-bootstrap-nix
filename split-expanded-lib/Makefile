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
	@echo "Building split-expanded-bin binary..."
	cargo build --package split-expanded-bin

# Alias for building both
build: build-bin

# Run split-expanded-bin with example arguments
run: build
	@echo "Running split-expanded-bin..."
	cargo run --bin split-expanded-bin -- \
		--expanded-manifest $(EXPANDED_DIR)/expanded_manifest.json \
		--project-root $(PROJECT_ROOT) \
		--rustc-version $(RUSTC_VERSION) \
		--rustc-host $(RUSTC_HOST) \
		--verbosity $(VERBOSITY) \
		--layer $(LAYER)

# Clean generated files and build artifacts
clean:
	@echo "Cleaning build artifacts and generated files..."
	cargo clean --package split-expanded-lib
	cargo clean --package split-expanded-bin
	rm -rf $(PROJECT_ROOT)/rust-bootstrap-core/src/level_*
