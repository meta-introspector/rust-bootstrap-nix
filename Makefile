NIX_FLAKE_ROOT := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

# Find all directories containing a flake.nix file
FLAKE_DIRS := $(shell find $(NIX_FLAKE_ROOT) -type f -name "flake.nix" -print0 | xargs -0 -n1 dirname | sort -u)

.PHONY: update-flakes
update-flakes:
	@echo "Deleting existing flake.lock files..."
	@find $(NIX_FLAKE_ROOT) -type f -name "flake.lock" -delete
	@echo "Updating all flake.lock files..."
	@for dir in $(FLAKE_DIRS); do \
		echo "Updating flake in $$dir..."; \
		nix flake update "$$dir" || { echo "Error updating flake in $$dir"; exit 1; }; \
	done
	@echo "All flake.lock files updated."