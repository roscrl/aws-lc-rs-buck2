# Architecture

## Overview

This project builds `aws-lc-rs` using Buck2 without Cargo or CMake. The build system compiles:

1. **AWS-LC** - A C cryptography library (fork of BoringSSL)
2. **aws-lc-sys** - Rust FFI bindings to AWS-LC
3. **aws-lc-rs** - High-level Rust cryptography API

## Build Graph

```
┌─────────────────────────────────────────────────────────────┐
│                    aws-lc-rs (Rust)                         │
│                //aws-lc-rs/aws-lc-rs:aws-lc-rs              │
└─────────────────────────┬───────────────────────────────────┘
                          │ depends on
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                   aws-lc-sys (Rust FFI)                     │
│                //aws-lc-rs/aws-lc-sys:aws-lc-sys            │
│                                                             │
│  Uses pregenerated bindings via --cfg flags:                │
│  - pregenerated_bindings_ARCH_VENDOR_OS                     │
└─────────────────────────┬───────────────────────────────────┘
                          │ links to
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  aws_lc_crypto (C library)                  │
│              //aws-lc-rs/aws-lc-sys:aws_lc_crypto           │
│                                                             │
│  258 C source files + 421 assembly files                   │
│  Platform-specific via select()                             │
└─────────────────────────┬───────────────────────────────────┘
                          │ depends on
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                   jitterentropy (C library)                 │
│               //aws-lc-rs/aws-lc-sys:jitterentropy          │
│                                                             │
│  Compiled with -O0 (required by source)                     │
└─────────────────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Separate Jitterentropy Library

The jitterentropy random number generator requires `-O0` compilation to function correctly. We compile it as a separate `cxx_library` target with custom compiler flags.

```python
cxx_library(
    name = "jitterentropy",
    srcs = JITTERENTROPY_SRCS,
    compiler_flags = ["-O0", "-fPIC"],  # -O0 required!
    ...
)
```

### 2. Platform-Specific Assembly

AWS-LC uses hand-optimized assembly for cryptographic operations. We use Buck2's `select()` to choose the correct assembly files:

```python
ASM_SRCS = select({
    "prelude//os:macos": select({
        "prelude//cpu:arm64": APPLE_AARCH64_ASM,
        "prelude//cpu:x86_64": APPLE_X86_64_ASM,
    }),
    "prelude//os:linux": select({
        "prelude//cpu:arm64": LINUX_AARCH64_ASM,
        "prelude//cpu:x86_64": LINUX_X86_64_ASM,
    }),
    ...
})
```

### 3. Pregenerated Bindings

Instead of running `bindgen` at build time, we use aws-lc-sys's pregenerated bindings. This is controlled via `--cfg` flags:

```python
rustc_flags = select({
    "prelude//os:macos": select({
        "prelude//cpu:arm64": ["--cfg=pregenerated_bindings_aarch64_apple_darwin"],
        "prelude//cpu:x86_64": ["--cfg=pregenerated_bindings_x86_64_apple_darwin"],
    }),
    ...
})
```

### 4. Include Paths

The C library requires specific include paths:

```python
include_directories = [
    "aws-lc/generated-include",  # Generated headers
    "aws-lc/include",            # Public API
    "aws-lc/crypto",             # Internal headers
    "aws-lc/third_party/...",    # Third-party deps
]
```

## File Organization

```
aws-lc-rs/aws-lc-sys/
├── BUCK                 # Main build rules
├── sources.bzl          # Source file lists (750+ lines)
│   ├── CRYPTO_SRCS      # Core C sources (258 files)
│   ├── JITTERENTROPY_SRCS
│   ├── APPLE_AARCH64_ASM
│   ├── APPLE_X86_64_ASM
│   ├── LINUX_AARCH64_ASM
│   ├── LINUX_X86_64_ASM
│   └── ...
├── defines.bzl          # Preprocessor defines
│   ├── COMMON_DEFINES
│   └── PLATFORM_DEFINES
└── aws-lc/              # C source (git submodule)
    ├── crypto/          # Crypto implementations
    ├── include/         # Public headers
    ├── generated-include/
    └── generated-src/   # Platform-specific asm
```

## Platform Support Matrix

| Platform | Arch | Assembly | Bindings | Status |
|----------|------|----------|----------|--------|
| macOS | aarch64 | ✓ | ✓ | Tested |
| macOS | x86_64 | ✓ | ✓ | Configured |
| Linux | aarch64 | ✓ | ✓ | Tested |
| Linux | x86_64 | ✓ | ✓ | Tested |
| Windows | x86_64 | ✓ | ✓ | Configured |
| Windows | aarch64 | ✓ | ✓ | Configured |

## Performance Characteristics

The build produces optimized cryptographic code using:

- Hand-written assembly for critical paths
- SIMD instructions (NEON on ARM, AVX on x86)
- Constant-time implementations for side-channel resistance
- Hardware acceleration (AES-NI, SHA extensions)

### Benchmark Comparison

| Operation | macOS M-series | Linux aarch64 |
|-----------|----------------|---------------|
| SHA-256 | 2,190 MB/s | 1,888 MB/s |
| SHA-512 | 1,243 MB/s | 1,082 MB/s |
| AES-256-GCM | 361 MB/s | 831 MB/s |
| ChaCha20-Poly1305 | 305 MB/s | 625 MB/s |
| Ed25519 sign | 125k ops/s | 92k ops/s |
| ECDSA P-256 sign | 56k ops/s | 56k ops/s |

Note: AES-GCM is faster on Linux due to different runtime detection of hardware features.

## Extending the Build

### Adding a New Platform

1. Add assembly sources to `sources.bzl`:
   ```python
   NEW_PLATFORM_ASM = [
       "aws-lc/generated-src/new-platform/crypto/...",
   ]
   ```

2. Update `select()` statements in `BUCK`

3. Add pregenerated bindings cfg flag

### Adding New Cryptographic Tests

Edit `test/crypto/main.rs` and add test functions following the existing pattern.
