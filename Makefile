.PHONY: help help-all build release test fmt clippy check clean outdated install install-from-source tag-release

CARGO := cargo
EXECUTABLE := $(notdir $(CURDIR))

.DEFAULT_GOAL := help

build:
	$(CARGO) build
	@echo "binary: target/debug/$(EXECUTABLE)"

release:
	$(CARGO) build --release
	@echo "binary: target/release/$(EXECUTABLE)"

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt --check

clippy:
	$(CARGO) clippy --all-targets -- -D warnings

check: fmt clippy test build

clean:
	$(CARGO) clean

outdated:
	$(CARGO) outdated -R

install: release
	sh bin/install/install-from-source.sh

install-from-source: install

tag-release:
ifndef VERSION
	$(error VERSION is required, e.g. make tag-release VERSION=0.5.0)
endif
	sh bin/dev/tag-release.sh $(VERSION)

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "  check    - All CI checks (fmt, clippy, test, build)"
	@echo "  build    - Compile (debug)"
	@echo "  release  - Compile (release)"
	@echo "  test     - Run tests"
	@echo "  install  - Build in release mode and copy to ~/.local/bin"
	@echo ""
	@echo "Run 'make help-all' for the full list, including internal targets."

help-all:
	@echo "Public targets:"
	@echo "  check              - All CI checks (fmt, clippy, test, build)"
	@echo "  build              - Compile (debug)"
	@echo "  release            - Compile (release)"
	@echo "  test               - Run tests"
	@echo "  install            - Build in release mode and copy to ~/.local/bin"
	@echo ""
	@echo "Internal targets (typically called via check or build):"
	@echo "  fmt                - Check formatting"
	@echo "  clippy             - Lint with clippy, warnings are errors"
	@echo "  clean              - Remove build artifacts"
	@echo "  outdated           - List outdated dependencies (requires cargo-outdated)"
	@echo "  install-from-source  - Alias for install"
	@echo "  tag-release        - Bump version, commit, and tag (VERSION=...)"
