#!/usr/bin/env bash
set -euo pipefail

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           aws-lc-rs Buck2 Build Verification                 ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check prerequisites
echo "Checking prerequisites..."
command -v buck2 >/dev/null 2>&1 || { echo "❌ buck2 not found"; exit 1; }
command -v rustc >/dev/null 2>&1 || { echo "❌ rustc not found"; exit 1; }
command -v clang >/dev/null 2>&1 || { echo "❌ clang not found"; exit 1; }
echo "✓ All prerequisites found"
echo ""

# Check submodules
echo "Checking submodules..."
if [ ! -d "aws-lc-rs/aws-lc-sys/aws-lc/crypto" ]; then
    echo "Initializing submodules..."
    cd aws-lc-rs && git submodule update --init --recursive && cd ..
fi
echo "✓ Submodules ready"
echo ""

# Clean build
echo "Cleaning previous build..."
buck2 clean 2>/dev/null || true
echo "✓ Clean complete"
echo ""

# Build
echo "Building aws-lc-rs..."
buck2 build //aws-lc-rs/aws-lc-rs:aws-lc-rs --target-platforms root//platforms:default
echo "✓ Build complete"
echo ""

# Test
echo "Running tests..."
buck2 run //test/crypto:test_crypto --target-platforms root//platforms:default
echo ""

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           Verification Complete - All Passed!                ║"
echo "╚══════════════════════════════════════════════════════════════╝"
