# Quick Start Guide

## Prerequisites

- [Nix](https://nixos.org/download.html) (recommended) or manual setup:
  - Buck2 (2025-12-01 release)
  - Rust 1.70+
  - Clang 14+
  - Python 3

## Setup

```bash
# Clone with submodules
git clone --recursive <repo>
cd aws-lc-rs-buck2

# Enter development environment
nix develop
```

## Build & Test

```bash
# Build the library
make build

# Run all 18 crypto tests
make test

# Run examples
make examples

# Run benchmarks
make benchmark

# Show all commands
make help
```

## Using in Your Project

### 1. Add Dependency

In your `BUCK` file:

```python
rust_binary(
    name = "my_app",
    srcs = ["main.rs"],
    deps = ["//aws-lc-rs/aws-lc-rs:aws-lc-rs"],
    edition = "2021",
)
```

### 2. Write Code

```rust
use aws_lc_rs::{digest, hmac, rand, signature, aead, hkdf, agreement};
use aws_lc_rs::signature::KeyPair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // === Hashing ===
    let hash = digest::digest(&digest::SHA256, b"hello world");
    println!("SHA-256: {:02x?}", hash.as_ref());

    // === HMAC ===
    let key = hmac::Key::new(hmac::HMAC_SHA256, b"secret key");
    let tag = hmac::sign(&key, b"message");
    println!("HMAC: {:02x?}", tag.as_ref());

    // === Random Numbers ===
    let mut random_bytes = [0u8; 32];
    rand::fill(&mut random_bytes)?;
    println!("Random: {:02x?}", &random_bytes[..8]);

    // === Ed25519 Signatures ===
    let rng = rand::SystemRandom::new();
    let pkcs8 = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())?;
    let sig = key_pair.sign(b"message to sign");
    println!("Ed25519 sig: {:02x?}...", &sig.as_ref()[..16]);

    // === ECDSA P-256 ===
    let pkcs8 = signature::EcdsaKeyPair::generate_pkcs8(
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        &rng,
    )?;
    let ecdsa_key = signature::EcdsaKeyPair::from_pkcs8(
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        pkcs8.as_ref(),
    )?;
    let ecdsa_sig = ecdsa_key.sign(&rng, b"message")?;
    println!("ECDSA sig: {:02x?}...", &ecdsa_sig.as_ref()[..16]);

    // === X25519 Key Exchange ===
    let my_private = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    let my_public = my_private.compute_public_key()?;
    println!("X25519 public: {:02x?}...", &my_public.as_ref()[..16]);

    // === AES-256-GCM Encryption ===
    use aead::BoundKey;
    
    let mut key_bytes = [0u8; 32];
    rand::fill(&mut key_bytes)?;
    
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)?;
    let mut nonce_bytes = [0u8; 12];
    rand::fill(&mut nonce_bytes)?;
    
    struct SingleNonce(Option<aead::Nonce>);
    impl aead::NonceSequence for SingleNonce {
        fn advance(&mut self) -> Result<aead::Nonce, aws_lc_rs::error::Unspecified> {
            self.0.take().ok_or(aws_lc_rs::error::Unspecified)
        }
    }
    
    let nonce = SingleNonce(Some(aead::Nonce::assume_unique_for_key(nonce_bytes)));
    let mut sealing_key = aead::SealingKey::new(unbound_key, nonce);
    
    let mut plaintext = b"secret message".to_vec();
    sealing_key.seal_in_place_append_tag(aead::Aad::empty(), &mut plaintext)?;
    println!("Ciphertext: {:02x?}...", &plaintext[..16]);

    // === HKDF Key Derivation ===
    let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, b"salt");
    let prk = salt.extract(b"input key material");
    let mut derived = [0u8; 32];
    prk.expand(&[b"info"], hkdf::HKDF_SHA256)?
        .fill(&mut derived)?;
    println!("Derived key: {:02x?}...", &derived[..8]);

    Ok(())
}
```

### 3. Build & Run

```bash
buck2 run //:my_app --target-platforms root//platforms:default
```

## Available Algorithms

| Category | Algorithms |
|----------|-----------|
| **Hash** | SHA-256, SHA-384, SHA-512 |
| **MAC** | HMAC-SHA256, HMAC-SHA384, HMAC-SHA512 |
| **Signature** | Ed25519, ECDSA (P-256, P-384, P-521), RSA |
| **Key Exchange** | X25519, ECDH (P-256, P-384, P-521) |
| **AEAD** | AES-128-GCM, AES-256-GCM, ChaCha20-Poly1305 |
| **KDF** | HKDF-SHA256, HKDF-SHA384, HKDF-SHA512 |
| **Random** | SystemRandom (CSPRNG) |

## Docker Testing

```bash
# Test on Linux ARM64
make test-docker

# Test on Linux x86_64
make test-docker-x86
```

## Troubleshooting

### Build fails with missing headers

Ensure submodules are initialized:
```bash
cd aws-lc-rs && git submodule update --init --recursive
```

### "pregenerated bindings not found"

The platform may not have pregenerated bindings. Check supported platforms in `aws-lc-rs/aws-lc-sys/src/`.

### Slow first build

The first build compiles ~700 source files. Subsequent builds use Buck2's cache and are fast (<1 second for Rust-only changes).
