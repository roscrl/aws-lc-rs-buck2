//! Minimal example using aws-lc-rs built with Buck2

use aws_lc_rs::{digest, hmac, rand, signature};
use aws_lc_rs::signature::KeyPair;

fn main() {
    println!("🔐 aws-lc-rs Hello Crypto Example\n");

    // 1. Hash some data
    let message = b"Hello, cryptography!";
    let hash = digest::digest(&digest::SHA256, message);
    println!("SHA-256 of {:?}:", String::from_utf8_lossy(message));
    println!("  {:02x?}\n", hash.as_ref());

    // 2. Generate a random key and compute HMAC
    let mut key_bytes = [0u8; 32];
    rand::fill(&mut key_bytes).expect("Failed to generate random key");
    let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, &key_bytes);
    let tag = hmac::sign(&hmac_key, message);
    println!("HMAC-SHA256:");
    println!("  {:02x?}\n", tag.as_ref());

    // 3. Generate an Ed25519 key pair and sign
    let rng = rand::SystemRandom::new();
    let pkcs8 = signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .expect("Failed to generate Ed25519 key");
    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())
        .expect("Failed to parse Ed25519 key");
    
    let sig = key_pair.sign(message);
    println!("Ed25519 Signature:");
    println!("  {:02x?}...\n", &sig.as_ref()[..32]);

    // 4. Verify the signature
    let public_key = signature::UnparsedPublicKey::new(
        &signature::ED25519,
        key_pair.public_key().as_ref(),
    );
    public_key.verify(message, sig.as_ref())
        .expect("Signature verification failed");
    println!("✅ Signature verified successfully!");
}
