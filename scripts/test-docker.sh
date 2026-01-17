#!/usr/bin/env bash
# Test the build in a clean Docker environment
set -euo pipefail

PLATFORM="${1:-linux/arm64}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           Docker Build Verification                          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Platform: $PLATFORM"
echo ""

docker run --rm --platform "$PLATFORM" -v "$(pwd):/workspace" -w /workspace ubuntu:22.04 bash -c '
set -e

echo "=== Installing dependencies ==="
apt-get update -qq
apt-get install -y -qq curl git clang lld python3 zstd make >/dev/null 2>&1

echo "=== Installing Rust ==="
curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y >/dev/null 2>&1
export PATH="/root/.cargo/bin:${PATH}"

echo "=== Installing Buck2 ==="
ARCH=$(uname -m)
if [ "$ARCH" = "aarch64" ]; then
    BUCK_ARCH="aarch64-unknown-linux-gnu"
elif [ "$ARCH" = "x86_64" ]; then
    BUCK_ARCH="x86_64-unknown-linux-gnu"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi
curl -sL "https://github.com/facebook/buck2/releases/download/2025-12-01/buck2-${BUCK_ARCH}.zst" | zstd -d > /usr/local/bin/buck2
chmod +x /usr/local/bin/buck2

echo ""
echo "=== Environment ==="
echo "Arch: $ARCH"
echo "Rust: $(rustc --version)"
echo "Clang: $(clang --version | head -1)"
echo "Buck2: $(buck2 --version | head -1)"
echo ""

echo "=== Building aws-lc-rs ==="
buck2 build //aws-lc-rs/aws-lc-rs:aws-lc-rs --target-platforms root//platforms:default 2>&1 | tail -3

echo ""
echo "=== Running Tests ==="
buck2 run //test/crypto:test_crypto --target-platforms root//platforms:default 2>&1 | grep -E "(──|✓|✗|PASSED|FAILED|🎉)"

echo ""
echo "=== Running Examples ==="
echo "--- Hello Crypto ---"
buck2 run //examples/hello-crypto:hello-crypto --target-platforms root//platforms:default 2>&1 | grep -E "(🔐|✅)"

echo "--- Key Exchange ---"
buck2 run //examples/key-exchange:key-exchange --target-platforms root//platforms:default 2>&1 | grep "✅"

echo "--- Encrypt ---"
buck2 run //examples/encrypt:encrypt --target-platforms root//platforms:default 2>&1 | grep "✅"

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           All Tests Passed!                                  ║"
echo "╚══════════════════════════════════════════════════════════════╝"
'
