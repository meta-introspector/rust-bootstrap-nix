.PHONY: all build

all: build

build:
	@echo "Entering Nix development shell and running cargo build..."
	nix develop --command bash -c "cargo build"
	@echo "Adding Cargo.lock to Git..."
	git add Cargo.lock
	@echo "Running nix build..."
	nix build