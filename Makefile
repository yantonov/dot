.PHONY: build release test fmt clippy check clean outdated install install-from-source tag-release

CARGO := cargo
EXECUTABLE := $(notdir $(CURDIR))

build: ## compile (debug)
	$(CARGO) build
	@echo "binary: target/debug/$(EXECUTABLE)"

release: ## compile (release)
	$(CARGO) build --release
	@echo "binary: target/release/$(EXECUTABLE)"

test: ## run tests
	$(CARGO) test

fmt: ## check formatting
	$(CARGO) fmt --check

clippy: ## lint with clippy, warnings are errors
	$(CARGO) clippy --all-targets -- -D warnings

check: fmt clippy test build ## all CI checks (fmt, clippy, test, build)

clean: ## remove build artifacts
	$(CARGO) clean

outdated: ## list outdated dependencies (requires cargo-outdated)
	$(CARGO) outdated -R

install: release ## build in release mode and copy to ~/.local/bin
	sh bin/install/install-from-source.sh

install-from-source: install ## alias for install

tag-release: ## bump version, commit, and tag (usage: make tag-release VERSION=0.5.0)
ifndef VERSION
	$(error VERSION is required, e.g. make tag-release VERSION=0.5.0)
endif
	sh bin/dev/tag-release.sh $(VERSION)

help: ## print this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}'
