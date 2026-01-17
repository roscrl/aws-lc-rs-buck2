# aws-lc-rs with Buck2

Build [aws-lc-rs](https://github.com/aws/aws-lc-rs) using [Buck2](https://buck2.build/) - no Cargo, no CMake.

## Features

- ✅ **Pure Buck2 build** - No Cargo or CMake required
- ✅ **18 cryptographic tests** - Comprehensive validation
- ✅ **Multi-platform** - macOS, Linux, Windows (x86_64 & aarch64)
- ✅ **Fast rebuilds** - Content-addressable caching
- ✅ **Hermetic** - Fully reproducible builds

## Quick Start

```bash
# Clone with submodules
git clone --recursive <repo>
cd ppppp

# Enter development environment
nix develop

# Build
buck2 build //aws-lc-rs/aws-lc-rs:aws-lc-rs --target-platforms root//platforms:default

# Test
buck2 run //test/crypto:test_crypto --target-platforms root//platforms:default
```

## Test Results

```
╔══════════════════════════════════════════════════════════════╗
║         aws-lc-rs Buck2 Build - Comprehensive Tests          ║
╚══════════════════════════════════════════════════════════════╝

── Hashing ─────────────────────────────────────────────────────
 1. SHA-256........................ ✓
 2. SHA-384........................ ✓
 3. SHA-512........................ ✓

── Message Authentication ──────────────────────────────────────
 4. HMAC-SHA256.................... ✓
 5. HMAC-SHA384.................... ✓
 6. HMAC-SHA512.................... ✓

── Random Number Generation ────────────────────────────────────
 7. CSPRNG (32 bytes).............. ✓
 8. CSPRNG (64 bytes).............. ✓

── Digital Signatures ──────────────────────────────────────────
 9. Ed25519........................ ✓
10. ECDSA P-256.................... ✓
11. ECDSA P-384.................... ✓
12. ECDSA P-521.................... ✓

── Authenticated Encryption ────────────────────────────────────
13. AES-128-GCM.................... ✓
14. AES-256-GCM.................... ✓
15. ChaCha20-Poly1305.............. ✓

── Key Derivation ──────────────────────────────────────────────
16. HKDF-SHA256.................... ✓
17. HKDF-SHA384.................... ✓
18. HKDF-SHA512.................... ✓

╔══════════════════════════════════════════════════════════════╗
║  Result: ALL 18 TESTS PASSED                                ║
╚══════════════════════════════════════════════════════════════╝
```

## Supported Platforms

| Platform | Architecture | Status |
|----------|--------------|--------|
| macOS | aarch64 | ✅ Tested |
| macOS | x86_64 | ✅ Configured |
| Linux | aarch64 | ✅ Tested |
| Linux | x86_64 | ✅ Configured |
| Windows | x86_64 | ✅ Configured |
| Windows | aarch64 | ✅ Configured |

## Project Structure

```
.
├── aws-lc-rs/
│   ├── aws-lc-sys/           # C library + Rust FFI
│   │   ├── BUCK              # Build rules
│   │   ├── sources.bzl       # ~750 lines of source paths
│   │   ├── defines.bzl       # Compiler defines
│   │   └── aws-lc/           # C source (submodule)
│   └── aws-lc-rs/            # Main Rust library
│       └── BUCK
├── third-party/rust/         # Dependencies (zeroize, untrusted)
├── test/crypto/              # Comprehensive test suite
├── platforms/                # Platform definitions
└── toolchains/               # Toolchain configuration
```

## Usage

Add as a dependency in your BUCK file:

```python
rust_binary(
    name = "my_app",
    srcs = ["main.rs"],
    deps = ["//aws-lc-rs/aws-lc-rs:aws-lc-rs"],
)
```

Use in Rust:

```rust
use aws_lc_rs::{digest, hmac, rand, signature, aead, hkdf};
use aws_lc_rs::signature::KeyPair;

fn main() {
    // Hashing
    let hash = digest::digest(&digest::SHA256, b"hello");
    
    // HMAC
    let key = hmac::Key::new(hmac::HMAC_SHA256, b"secret");
    let tag = hmac::sign(&key, b"message");
    
    // Random
    let mut bytes = [0u8; 32];
    rand::fill(&mut bytes).unwrap();
    
    // Ed25519 signing
    let rng = rand::SystemRandom::new();
    let pkcs8 = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
    let sig = key_pair.sign(b"message");
    
    // ECDSA P-256
    let pkcs8 = signature::EcdsaKeyPair::generate_pkcs8(
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING, &rng
    ).unwrap();
    
    // AES-GCM, ChaCha20-Poly1305, HKDF all work too!
}
```

## Build Targets

| Target | Description |
|--------|-------------|
| `//aws-lc-rs/aws-lc-sys:jitterentropy` | Jitterentropy RNG (compiled with -O0) |
| `//aws-lc-rs/aws-lc-sys:aws_lc_crypto` | AWS-LC C library (~300 files) |
| `//aws-lc-rs/aws-lc-sys:aws-lc-sys` | Rust FFI bindings |
| `//aws-lc-rs/aws-lc-rs:aws-lc-rs` | Main Rust crypto library |
| `//test/crypto:test_crypto` | Comprehensive test suite |

## How It Works

1. **C Library**: 258 C files + 421 platform-specific assembly files compiled with `cxx_library`
2. **Jitterentropy**: Compiled separately with `-O0` (required by source)
3. **Platform Selection**: Buck2 `select()` chooses correct assembly per platform
4. **FFI Bindings**: Uses aws-lc-sys's pregenerated bindings via `--cfg` flags
5. **Rust Library**: Standard `rust_library` depending on the C library

## Examples

```bash
# Hello Crypto - minimal example
buck2 run //examples/hello-crypto:hello-crypto --target-platforms root//platforms:default

# Benchmark - performance test
buck2 run //examples/benchmark:benchmark --target-platforms root//platforms:default
```

### Benchmark Results (Apple M-series)

```
SHA-256: 2189 MB/s
SHA-512: 1243 MB/s
HMAC-SHA256: 1431 MB/s
AES-256-GCM: 360 MB/s
ChaCha20-Poly1305: 305 MB/s
Ed25519 sign: 125,213 ops/sec
Ed25519 verify: 34,994 ops/sec
ECDSA P-256 sign: 56,401 ops/sec
```

## Statistics

| Metric | Value |
|--------|-------|
| C source files | 258 |
| Assembly files | 421 |
| Cryptographic tests | 18 |
| Platforms supported | 6 |
| Clean build time | ~17 seconds |
| Cached rebuild | <1 second |

## Documentation

- [Quick Start Guide](docs/QUICKSTART.md) - Get started in 5 minutes
- [Architecture](docs/ARCHITECTURE.md) - How the build system works
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Changelog](CHANGELOG.md) - Version history

## License

- aws-lc-rs: Apache-2.0 OR ISC
- AWS-LC: Apache-2.0 OR ISC OR OpenSSL
- This configuration: Same as aws-lc-rs
