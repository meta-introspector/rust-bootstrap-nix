NIX_FLAKE_ROOT := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# Hardcoded list of flakes to update
FLAKE_DIRS := \
	$(NIX_FLAKE_ROOT) \
	$(NIX_FLAKE_ROOT)/flakes/bootstrap-builder \
	$(NIX_FLAKE_ROOT)/flakes/bootstrap-builder/cc-flake \
	$(NIX_FLAKE_ROOT)/flakes/bootstrap-compiler-flake \
	$(NIX_FLAKE_ROOT)/flakes/config \
	$(NIX_FLAKE_ROOT)/flakes/evaluate-rust \
	$(NIX_FLAKE_ROOT)/minimal-flake 

.PHONY: update-flakes
update-flakes:
	@echo "Deleting existing flake.lock files..."
	@find $(NIX_FLAKE_ROOT) -type f -name "flake.lock" -delete
	@echo "Updating selected flake.lock files..."
	@for dir in $(FLAKE_DIRS); do \
		echo "Updating flake in $$dir..."; \
		nix flake update "$$dir" || { echo "Error updating flake in $$dir"; exit 1; }; \
	done
	@echo "All selected flake.lock files updated."
