.PHONY: help build run test

help: ## List available targets
	@grep -E '^[a-z-]+:.*## ' $(MAKEFILE_LIST) | awk -F':.*## ' '{printf "  %-8s %s\n", $$1, $$2}'

build: ## Build the release binary (Vulkan GPU support included)
	cargo build --release

run: build ## Build and start Mundwerk
	./target/release/mundwerk

test: ## Run the unit tests
	cargo test --release
