FLAKES = flakes/config \
         flakes/xpy-json-output-flake \
         flakes/json-processor \
         flakes/json-processor-flake \
         flakes/evaluate-rust

.PHONY: update-flakes

update-flakes:
	@echo "Updating root flake..."
	nix flake update .
	@echo "Updating sub-flakes..."
	@for flake in $(FLAKES); do \
		echo "Updating $$flake..."; \
		nix flake update $$flake; \
	done
