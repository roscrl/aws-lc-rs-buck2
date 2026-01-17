//! Simple benchmark for aws-lc-rs operations

use aws_lc_rs::{aead, digest, hmac, rand, signature};
use aws_lc_rs::aead::BoundKey;
use aws_lc_rs::signature::KeyPair;
use std::time::{Duration, Instant};

const ITERATIONS: u32 = 10_000;
const DATA_SIZE: usize = 1024; // 1 KB

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║           aws-lc-rs Performance Benchmark                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let data = vec![0x42u8; DATA_SIZE];
    
    // SHA-256
    let duration = bench("SHA-256 (1KB)", ITERATIONS, || {
        let _ = digest::digest(&digest::SHA256, &data);
    });
    print_result("SHA-256", DATA_SIZE, ITERATIONS, duration);

    // SHA-512
    let duration = bench("SHA-512 (1KB)", ITERATIONS, || {
        let _ = digest::digest(&digest::SHA512, &data);
    });
    print_result("SHA-512", DATA_SIZE, ITERATIONS, duration);

    // HMAC-SHA256
    let key = hmac::Key::new(hmac::HMAC_SHA256, b"benchmark key 32 bytes long!!!!!");
    let duration = bench("HMAC-SHA256 (1KB)", ITERATIONS, || {
        let _ = hmac::sign(&key, &data);
    });
    print_result("HMAC-SHA256", DATA_SIZE, ITERATIONS, duration);

    // AES-256-GCM encryption
    let mut key_bytes = [0u8; 32];
    rand::fill(&mut key_bytes).unwrap();
    let duration = bench("AES-256-GCM encrypt (1KB)", ITERATIONS, || {
        let uk = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes).unwrap();
        let mut nonce_bytes = [0u8; 12];
        rand::fill(&mut nonce_bytes).unwrap();
        let ns = SingleNonce(Some(aead::Nonce::assume_unique_for_key(nonce_bytes)));
        let mut sk = aead::SealingKey::new(uk, ns);
        let mut buf = data.clone();
        let _ = sk.seal_in_place_append_tag(aead::Aad::empty(), &mut buf);
    });
    print_result("AES-256-GCM", DATA_SIZE, ITERATIONS, duration);

    // ChaCha20-Poly1305 encryption
    let duration = bench("ChaCha20-Poly1305 (1KB)", ITERATIONS, || {
        let uk = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key_bytes).unwrap();
        let mut nonce_bytes = [0u8; 12];
        rand::fill(&mut nonce_bytes).unwrap();
        let ns = SingleNonce(Some(aead::Nonce::assume_unique_for_key(nonce_bytes)));
        let mut sk = aead::SealingKey::new(uk, ns);
        let mut buf = data.clone();
        let _ = sk.seal_in_place_append_tag(aead::Aad::empty(), &mut buf);
    });
    print_result("ChaCha20-Poly1305", DATA_SIZE, ITERATIONS, duration);

    // Ed25519 signing
    let rng = rand::SystemRandom::new();
    let pkcs8 = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
    let sign_iterations = 1_000;
    let duration = bench("Ed25519 sign", sign_iterations, || {
        let _ = key_pair.sign(&data);
    });
    print_ops("Ed25519 sign", sign_iterations, duration);

    // Ed25519 verify
    let sig = key_pair.sign(&data);
    let public_key = signature::UnparsedPublicKey::new(
        &signature::ED25519,
        key_pair.public_key().as_ref(),
    );
    let duration = bench("Ed25519 verify", sign_iterations, || {
        let _ = public_key.verify(&data, sig.as_ref());
    });
    print_ops("Ed25519 verify", sign_iterations, duration);

    // ECDSA P-256 signing
    let pkcs8 = signature::EcdsaKeyPair::generate_pkcs8(
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        &rng,
    ).unwrap();
    let ecdsa_kp = signature::EcdsaKeyPair::from_pkcs8(
        &signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        pkcs8.as_ref(),
    ).unwrap();
    let duration = bench("ECDSA P-256 sign", sign_iterations, || {
        let _ = ecdsa_kp.sign(&rng, &data);
    });
    print_ops("ECDSA P-256 sign", sign_iterations, duration);

    println!("\n✅ Benchmark complete!");
}

fn bench<F: FnMut()>(name: &str, iterations: u32, mut f: F) -> Duration {
    print!("Running {}...", name);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
    // Warmup
    for _ in 0..100 {
        f();
    }
    
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let duration = start.elapsed();
    println!(" done");
    duration
}

fn print_result(name: &str, data_size: usize, iterations: u32, duration: Duration) {
    let total_bytes = data_size as u64 * iterations as u64;
    let throughput_mb = (total_bytes as f64 / 1_000_000.0) / duration.as_secs_f64();
    println!("  {}: {:.2} MB/s", name, throughput_mb);
}

fn print_ops(name: &str, iterations: u32, duration: Duration) {
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();
    println!("  {}: {:.0} ops/sec", name, ops_per_sec);
}

struct SingleNonce(Option<aead::Nonce>);
impl aead::NonceSequence for SingleNonce {
    fn advance(&mut self) -> Result<aead::Nonce, aws_lc_rs::error::Unspecified> {
        self.0.take().ok_or(aws_lc_rs::error::Unspecified)
    }
}
