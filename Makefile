.DEFAULT_GOAL := help

# Show possible commands
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build (b)         	- Build the project"
	@echo "  test (t)          	- Run tests with all features"
	@echo "  clippy (lint)     	- Run Clippy on the workspace"
	@echo "  fmt              	- Format the project using nightly"
	@echo "  fmt-check         	- Checks if the codebase is formatted correctly"
	@echo "  doc (d)     	  	- Build the docs"

# Development group
.PHONY: build b
build b:
	cargo build --all-targets

.PHONY: test t
test t:
	cargo test --all-features --all

.PHONY: clippy lint
clippy lint:
	cargo clippy --all-features --all-targets

.PHONY: fmt
fmt:
	cargo +nightly fmt

.PHONY: fmt-check
fmt-check:
	cargo fmt --check

.PHONY: doc d
doc d:
	cargo doc --no-deps --open
