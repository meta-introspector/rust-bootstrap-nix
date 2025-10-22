.PHONY: all build fast-build

all: build

build:
	$(MAKE) -C nix-build-scripts/

fast-build:
	$(MAKE) -C nix-build-scripts/ fast-build