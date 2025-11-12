# .PHONY: regenerate-generated-projects
# regenerate-generated-projects:
# 	@echo "Building rust-system-composer..."
# 	cargo build --package rust-system-composer

# 	@echo "Regenerating projects with rust-system-composer..."
# 	cargo run --package rust-system-composer -- \
# 		--input-dir /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/generated-declarations-lib/src/ \
# 		--output-root-dir /data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/generated_projects/ \
# 		--rustc-version "$(shell rustc --version | cut -d ' ' -f 2)" \
# 		--rustc-host "$(shell rustc --version --verbose | grep host | cut -d ' ' -f 2 | head -n 1)"

# 	@echo "Regenerating generated_projects/Cargo.toml..."
# 	/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/scripts/generate_generated_projects_workspace.sh

# 	@echo "Generated projects and workspace updated."
