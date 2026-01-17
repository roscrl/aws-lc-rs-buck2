use aws_lc_rs::{aead, digest, hkdf, hmac, rand, signature};
use aws_lc_rs::aead::BoundKey;
use aws_lc_rs::signature::KeyPair;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         aws-lc-rs Buck2 Build - Comprehensive Tests          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut passed = 0;
    let mut failed = 0;
    let rng = rand::SystemRandom::new();

    // === HASHING ===
    println!("── Hashing ─────────────────────────────────────────────────────");
    
    print!(" 1. SHA-256........................");
    if test_sha256() { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!(" 2. SHA-384........................");
    if test_digest(&digest::SHA384, 48) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!(" 3. SHA-512........................");
    if test_digest(&digest::SHA512, 64) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    // === MAC ===
    println!("\n── Message Authentication ──────────────────────────────────────");
    
    print!(" 4. HMAC-SHA256....................");
    if test_hmac(hmac::HMAC_SHA256) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!(" 5. HMAC-SHA384....................");
    if test_hmac(hmac::HMAC_SHA384) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!(" 6. HMAC-SHA512....................");
    if test_hmac(hmac::HMAC_SHA512) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    // === RANDOM ===
    println!("\n── Random Number Generation ────────────────────────────────────");
    
    print!(" 7. CSPRNG (32 bytes)..............");
    if test_random(32) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!(" 8. CSPRNG (64 bytes)..............");
    if test_random(64) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    // === SIGNATURES ===
    println!("\n── Digital Signatures ──────────────────────────────────────────");
    
    print!(" 9. Ed25519........................");
    if test_ed25519(&rng) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("10. ECDSA P-256....................");
    if test_ecdsa_p256(&rng) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("11. ECDSA P-384....................");
    if test_ecdsa_p384(&rng) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("12. ECDSA P-521....................");
    if test_ecdsa_p521(&rng) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    // === AEAD ===
    println!("\n── Authenticated Encryption ────────────────────────────────────");
    
    print!("13. AES-128-GCM....................");
    if test_aead(&aead::AES_128_GCM, 16) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("14. AES-256-GCM....................");
    if test_aead(&aead::AES_256_GCM, 32) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("15. ChaCha20-Poly1305..............");
    if test_aead(&aead::CHACHA20_POLY1305, 32) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    // === KEY DERIVATION ===
    println!("\n── Key Derivation ──────────────────────────────────────────────");
    
    print!("16. HKDF-SHA256....................");
    if test_hkdf(hkdf::HKDF_SHA256) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("17. HKDF-SHA384....................");
    if test_hkdf(hkdf::HKDF_SHA384) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    print!("18. HKDF-SHA512....................");
    if test_hkdf(hkdf::HKDF_SHA512) { println!(" ✓"); passed += 1; } else { println!(" ✗"); failed += 1; }

    // Summary
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    if failed == 0 {
        println!("║  Result: ALL {} TESTS PASSED                                ║", passed);
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("\n🎉 aws-lc-rs successfully built and tested with Buck2!");
    } else {
        println!("║  Result: {} passed, {} failed                                ║", passed, failed);
        println!("╚══════════════════════════════════════════════════════════════╝");
        std::process::exit(1);
    }
}

fn test_sha256() -> bool {
    let hash = digest::digest(&digest::SHA256, b"hello world");
    hash.as_ref() == [
        0xb9, 0x4d, 0x27, 0xb9, 0x93, 0x4d, 0x3e, 0x08,
        0xa5, 0x2e, 0x52, 0xd7, 0xda, 0x7d, 0xab, 0xfa,
        0xc4, 0x84, 0xef, 0xe3, 0x7a, 0x53, 0x80, 0xee,
        0x90, 0x88, 0xf7, 0xac, 0xe2, 0xef, 0xcd, 0xe9,
    ]
}

fn test_digest(alg: &'static digest::Algorithm, expected_len: usize) -> bool {
    let hash = digest::digest(alg, b"test data");
    hash.as_ref().len() == expected_len
}

fn test_hmac(alg: hmac::Algorithm) -> bool {
    let key = hmac::Key::new(alg, b"secret key");
    let tag = hmac::sign(&key, b"message");
    hmac::verify(&key, b"message", tag.as_ref()).is_ok()
}

fn test_random(len: usize) -> bool {
    let mut buf1 = vec![0u8; len];
    let mut buf2 = vec![0u8; len];
    rand::fill(&mut buf1).is_ok() && rand::fill(&mut buf2).is_ok() && buf1 != buf2
}

fn test_ed25519(rng: &rand::SystemRandom) -> bool {
    let pkcs8 = match signature::Ed25519KeyPair::generate_pkcs8(rng) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let kp = match signature::Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let sig = kp.sign(b"message");
    let pk = signature::UnparsedPublicKey::new(&signature::ED25519, kp.public_key().as_ref());
    pk.verify(b"message", sig.as_ref()).is_ok()
}

fn test_ecdsa_p256(rng: &rand::SystemRandom) -> bool {
    let alg = &signature::ECDSA_P256_SHA256_ASN1_SIGNING;
    let pkcs8 = match signature::EcdsaKeyPair::generate_pkcs8(alg, rng) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let kp = match signature::EcdsaKeyPair::from_pkcs8(alg, pkcs8.as_ref()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let sig = match kp.sign(rng, b"message") {
        Ok(s) => s,
        Err(_) => return false,
    };
    let pk = signature::UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_ASN1, kp.public_key().as_ref());
    pk.verify(b"message", sig.as_ref()).is_ok()
}

fn test_ecdsa_p384(rng: &rand::SystemRandom) -> bool {
    let alg = &signature::ECDSA_P384_SHA384_ASN1_SIGNING;
    let pkcs8 = match signature::EcdsaKeyPair::generate_pkcs8(alg, rng) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let kp = match signature::EcdsaKeyPair::from_pkcs8(alg, pkcs8.as_ref()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let sig = match kp.sign(rng, b"message") {
        Ok(s) => s,
        Err(_) => return false,
    };
    let pk = signature::UnparsedPublicKey::new(&signature::ECDSA_P384_SHA384_ASN1, kp.public_key().as_ref());
    pk.verify(b"message", sig.as_ref()).is_ok()
}

fn test_ecdsa_p521(rng: &rand::SystemRandom) -> bool {
    let alg = &signature::ECDSA_P521_SHA512_ASN1_SIGNING;
    let pkcs8 = match signature::EcdsaKeyPair::generate_pkcs8(alg, rng) {
        Ok(p) => p,
        Err(_) => return false,
    };
    let kp = match signature::EcdsaKeyPair::from_pkcs8(alg, pkcs8.as_ref()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let sig = match kp.sign(rng, b"message") {
        Ok(s) => s,
        Err(_) => return false,
    };
    let pk = signature::UnparsedPublicKey::new(&signature::ECDSA_P521_SHA512_ASN1, kp.public_key().as_ref());
    pk.verify(b"message", sig.as_ref()).is_ok()
}

fn test_aead(alg: &'static aead::Algorithm, key_len: usize) -> bool {
    let mut key_bytes = vec![0u8; key_len];
    if rand::fill(&mut key_bytes).is_err() { return false; }
    
    let uk = match aead::UnboundKey::new(alg, &key_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    let ns = match NonceSeq::new() {
        Ok(n) => n,
        Err(_) => return false,
    };
    
    let mut sk = aead::SealingKey::new(uk, ns);
    let mut data = b"plaintext message".to_vec();
    
    sk.seal_in_place_append_tag(aead::Aad::from(b"aad"), &mut data).is_ok()
}

fn test_hkdf(alg: hkdf::Algorithm) -> bool {
    let salt = hkdf::Salt::new(alg, b"salt value");
    let prk = salt.extract(b"input key material");
    let okm = prk.expand(&[b"info"], MyLen(32));
    match okm {
        Ok(o) => {
            let mut out = [0u8; 32];
            o.fill(&mut out).is_ok()
        }
        Err(_) => false,
    }
}

struct NonceSeq(Option<aead::Nonce>);
impl NonceSeq {
    fn new() -> Result<Self, ()> {
        let mut n = [0u8; 12];
        rand::fill(&mut n).map_err(|_| ())?;
        Ok(Self(Some(aead::Nonce::assume_unique_for_key(n))))
    }
}
impl aead::NonceSequence for NonceSeq {
    fn advance(&mut self) -> Result<aead::Nonce, aws_lc_rs::error::Unspecified> {
        self.0.take().ok_or(aws_lc_rs::error::Unspecified)
    }
}

struct MyLen(usize);
impl hkdf::KeyType for MyLen {
    fn len(&self) -> usize { self.0 }
}
