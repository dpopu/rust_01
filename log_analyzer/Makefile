.PHONY: build test fmt check clean run release help

# Detect OS
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
else
    DETECTED_OS := $(shell uname -s)
endif

# Default target - OS-aware help
help:
ifeq ($(DETECTED_OS),Windows)
	@echo Available targets:
	@echo   make build     - Build all workspace members
	@echo   make test      - Run all tests
	@echo   make fmt       - Format all code
	@echo   make check     - Run cargo check and clippy
	@echo   make clean     - Clean build artifacts
	@echo   make run       - Run the log_parser binary
	@echo   make release   - Build release version
else
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
endif

## Build all workspace members
build:
	cargo build --all

## Run all tests
test:
	cargo test --all

## Format all code
fmt:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all

## Check code and run clippy
check:
	cargo check --all
	cargo clippy --all -- -D warnings

## Clean build artifacts
clean:
	cargo clean

## Run the log_parser binary
run:
	cargo run --manifest-path bins/log_parser/Cargo.toml

## Build release version
release:
	cargo build --all --release
