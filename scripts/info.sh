#!/usr/bin/env bash
# Display project info

set -euo pipefail

VERSION=$(cat VERSION 2>/dev/null || echo "unknown")

cat <<EOF
╔══════════════════════════════════════════════════════════════╗
║           aws-lc-rs Buck2 Build v${VERSION}                        ║
╚══════════════════════════════════════════════════════════════╝

Platform:     $(uname -s) $(uname -m)
Buck2:        $(buck2 --version 2>/dev/null | head -1 || echo "not found")
Rust:         $(rustc --version 2>/dev/null || echo "not found")
Clang:        $(clang --version 2>/dev/null | head -1 || echo "not found")

Targets:
  //aws-lc-rs/aws-lc-rs:aws-lc-rs     Main crypto library
  //test/crypto:test_crypto            18 cryptographic tests
  //examples/hello-crypto              Minimal usage example
  //examples/key-exchange              X25519 ECDH example
  //examples/encrypt                   AES-256-GCM example
  //examples/benchmark                 Performance benchmark

Quick Start:
  make build       Build the library
  make test        Run all tests
  make examples    Run all examples
  make benchmark   Run performance test
  make test-docker Test in Docker container
  make help        Show all commands

EOF
