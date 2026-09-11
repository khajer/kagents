.PHONY: build release run-server run-cli check check-server check-cli fmt clippy test test-verbose ci clean

build: ## Build all binaries (debug)
	cargo build

release: ## Build all binaries (release)
	cargo build --release

run-server: ## Start kserve HTTP server (port 6411)
	cargo run --bin kserve

run-cli: ## Run kcli client
	cargo run --bin kcli

check: ## Check the whole workspace
	cargo check

check-server: ## Check kserve only
	cargo check --bin kserve

check-cli: ## Check kcli only
	cargo check --bin kcli

fmt: ## Format code (required before commits)
	cargo fmt

clippy: ## Lint (required before commits)
	cargo clippy -- -D warnings

clippy-all: ## Lint all targets including tests
	cargo clippy --all-targets -- -D warnings

test: ## Run all tests
	cargo test

test-verbose: ## Run all tests with stdout shown
	cargo test -- --nocapture

ci: fmt clippy test ## Run the checks required before committing

clean: ## Remove build artifacts
	cargo clean
