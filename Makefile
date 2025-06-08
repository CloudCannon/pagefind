# Makefile for Pagefind

# Variables
# GIT_VERSION defaults to `git describe --tags --always --dirty` unless passed as an env var.
GIT_VERSION ?= $(shell git describe --tags --always --dirty)
SHELL := /bin/bash

.PHONY: all build test test-unit build-web-js build-ui-default build-ui-modular build-playground build-pagefind-web build-pagefind-web-fast build-pagefind-rust

build: build-web-js build-ui-default build-ui-modular build-playground build-pagefind-web-fast build-pagefind-rust
	@echo "✅ All components built successfully"

all: build-web-js build-ui-default build-ui-modular build-playground build-pagefind-web build-pagefind-rust
	@echo "✅ All components built successfully (fast pagefind build)"

build-web-js:
	@echo "Building pagefind_web_js..."
	@cd pagefind_web_js && npm i && npm run build-coupled
	@echo "✅ 1/6 pagefind_web_js built successfully"

build-ui-default:
	@echo "Building pagefind_ui/default..."
	@cd pagefind_ui/default && npm i && npm run build
	@echo "✅ 2/6 pagefind_ui/default built successfully"

build-ui-modular:
	@echo "Building pagefind_ui/modular..."
	@cd pagefind_ui/modular && npm i && npm run build
	@echo "✅ 3/6 pagefind_ui/modular built successfully"

build-playground:
	@echo "Building pagefind_playground..."
	@cd pagefind_playground && npm i && npm run build
	@echo "✅ 4/6 pagefind_playground built successfully"

build-pagefind-web:
	@echo "Building pagefind_web (ALL)..."
	@cd pagefind_web && ./local_build.sh
	@echo "✅ 5/6 pagefind_web (ALL) built successfully"

build-pagefind-web-fast:
	@echo "Building pagefind_web (FAST)..."
	@cd pagefind_web && ./local_fast_build.sh
	@echo "✅ pagefind_web (FAST) built successfully"

build-pagefind-rust:
	@echo "Building pagefind (Rust)..."
	@cd pagefind && cargo build --release --features extended
	@echo "✅ 6/6 pagefind built successfully"

test:
	@echo "Running integration tests..."
	@./scripts/test.sh

test-unit:
	@echo "Running unit tests..."
	@./scripts/test.unit.sh

clean:
	@echo "Cleaning Rust projects..."
	@cargo clean
	@echo "✅ Rust cleaned."
