# aws-lc-rs Buck2 Build
# Convenience targets for common operations

PLATFORM := root//platforms:default

.PHONY: all build test examples benchmark clean info help test-docker test-docker-x86 release

all: build test

build: ## Build aws-lc-rs
	buck2 build //aws-lc-rs/aws-lc-rs:aws-lc-rs --target-platforms $(PLATFORM)

test: ## Run all crypto tests
	buck2 run //test/crypto:test_crypto --target-platforms $(PLATFORM)

examples: ## Run all examples
	@echo "=== Hello Crypto ==="
	buck2 run //examples/hello-crypto:hello-crypto --target-platforms $(PLATFORM)
	@echo ""
	@echo "=== Key Exchange ==="
	buck2 run //examples/key-exchange:key-exchange --target-platforms $(PLATFORM)
	@echo ""
	@echo "=== Encryption ==="
	buck2 run //examples/encrypt:encrypt --target-platforms $(PLATFORM)
	@echo ""
	@echo "=== Benchmark ==="
	buck2 run //examples/benchmark:benchmark --target-platforms $(PLATFORM)

benchmark: ## Run benchmark only
	buck2 run //examples/benchmark:benchmark --target-platforms $(PLATFORM)

targets: ## List all targets
	buck2 targets //...

clean: ## Clean build artifacts
	buck2 clean

verify: clean build test examples ## Full clean build and test
	@echo ""
	@echo "✅ All verifications passed!"

test-docker: ## Test in Docker (linux/arm64)
	./scripts/test-docker.sh linux/arm64

test-docker-x86: ## Test in Docker (linux/amd64)
	./scripts/test-docker.sh linux/amd64

info: ## Show project info
	@./scripts/info.sh

release: ## Create release tarball
	./scripts/release.sh

help: ## Show this help
	@echo "aws-lc-rs Buck2 Build"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z0-9_-]+:.*## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*## "}; {printf "  %-15s %s\n", $$1, $$2}'
