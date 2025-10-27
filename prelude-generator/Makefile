
.PHONY: all build run clean

all: build

build:
	cargo build

run: build
	cargo run -- --dry-run --report

run-generate: build
	cargo run -- --dry-run

run-cache-report: build
	cargo run -- --cache-report

clean:
	cargo clean
	rm -f prelude_generator_summary.md prelude_processing_results.json
