# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-01-17

### Added

- Complete Buck2 build system for aws-lc-rs
- Support for macOS (aarch64, x86_64)
- Support for Linux (aarch64, x86_64)
- Windows configuration (x86_64, aarch64)
- 18 comprehensive cryptographic tests
- 4 practical examples:
  - `hello-crypto` - Minimal usage
  - `key-exchange` - X25519 ECDH
  - `encrypt` - AES-256-GCM with HKDF
  - `benchmark` - Performance testing
- Makefile with convenience commands
- GitHub Actions CI configuration
- Nix development environment

### Cryptographic Operations

- **Hashing**: SHA-256, SHA-384, SHA-512
- **MAC**: HMAC-SHA256, HMAC-SHA384, HMAC-SHA512
- **Random**: CSPRNG
- **Signatures**: Ed25519, ECDSA P-256/P-384/P-521
- **AEAD**: AES-128-GCM, AES-256-GCM, ChaCha20-Poly1305
- **KDF**: HKDF-SHA256, HKDF-SHA384, HKDF-SHA512
- **Key Exchange**: X25519

### Technical Details

- 258 C source files compiled with `cxx_library`
- 421 platform-specific assembly files
- Jitterentropy compiled separately with `-O0`
- Uses pregenerated Rust FFI bindings
- No Cargo, no CMake, no build.rs execution

### Performance (Apple Silicon)

- SHA-256: 2.2 GB/s
- SHA-512: 1.2 GB/s
- HMAC-SHA256: 1.4 GB/s
- AES-256-GCM: 360 MB/s
- Ed25519 sign: 125,000 ops/sec
- Ed25519 verify: 35,000 ops/sec
- ECDSA P-256 sign: 56,000 ops/sec
