//! File Encryption Example
//! Demonstrates AES-256-GCM authenticated encryption with key derivation

use aws_lc_rs::{aead, hkdf, rand};
use aws_lc_rs::aead::BoundKey;

fn main() {
    println!("🔒 File Encryption Example (AES-256-GCM + HKDF)\n");

    // Simulate a password-derived master key (in practice, use a proper KDF like Argon2)
    let password = b"my-secret-password";
    let salt = b"unique-salt-value";
    
    // Derive encryption key using HKDF
    println!("1. Deriving key from password...");
    let prk = hkdf::Salt::new(hkdf::HKDF_SHA256, salt).extract(password);
    let mut key_bytes = [0u8; 32];
    prk.expand(&[b"encryption"], hkdf::HKDF_SHA256)
        .expect("HKDF expand failed")
        .fill(&mut key_bytes)
        .expect("HKDF fill failed");
    println!("   Key derived: {:02x?}...\n", &key_bytes[..8]);

    // Original plaintext
    let plaintext = b"This is a secret message that needs to be encrypted!";
    println!("2. Original message ({} bytes):", plaintext.len());
    println!("   {:?}\n", String::from_utf8_lossy(plaintext));

    // Encrypt
    println!("3. Encrypting with AES-256-GCM...");
    let (ciphertext, nonce) = encrypt(&key_bytes, plaintext);
    println!("   Ciphertext ({} bytes): {:02x?}...", ciphertext.len(), &ciphertext[..16]);
    println!("   Nonce: {:02x?}\n", nonce);

    // Decrypt
    println!("4. Decrypting...");
    let decrypted = decrypt(&key_bytes, &nonce, &ciphertext);
    println!("   Decrypted: {:?}\n", String::from_utf8_lossy(&decrypted));

    // Verify
    if plaintext.as_slice() == decrypted.as_slice() {
        println!("✅ Success! Decrypted message matches original.");
    } else {
        println!("❌ Error: Decryption failed!");
        std::process::exit(1);
    }

    // Demonstrate tamper detection
    println!("\n5. Demonstrating tamper detection...");
    let mut tampered = ciphertext.clone();
    tampered[0] ^= 0xFF; // Flip some bits
    match try_decrypt(&key_bytes, &nonce, &tampered) {
        Ok(_) => println!("   ❌ Tampered data was accepted (this shouldn't happen)"),
        Err(_) => println!("   ✅ Tampered data rejected - authentication failed!"),
    }
}

fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, [u8; 12]) {
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)
        .expect("Failed to create key");
    
    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    rand::fill(&mut nonce_bytes).expect("Failed to generate nonce");
    
    let nonce_seq = SingleNonce(Some(aead::Nonce::assume_unique_for_key(nonce_bytes)));
    let mut sealing_key = aead::SealingKey::new(unbound_key, nonce_seq);
    
    let mut in_out = plaintext.to_vec();
    sealing_key.seal_in_place_append_tag(aead::Aad::empty(), &mut in_out)
        .expect("Encryption failed");
    
    (in_out, nonce_bytes)
}

fn decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Vec<u8> {
    try_decrypt(key, nonce, ciphertext).expect("Decryption failed")
}

fn try_decrypt(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, aws_lc_rs::error::Unspecified> {
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)?;
    
    let nonce_seq = SingleNonce(Some(aead::Nonce::assume_unique_for_key(*nonce)));
    let mut opening_key = aead::OpeningKey::new(unbound_key, nonce_seq);
    
    let mut in_out = ciphertext.to_vec();
    let plaintext = opening_key.open_in_place(aead::Aad::empty(), &mut in_out)?;
    
    Ok(plaintext.to_vec())
}

struct SingleNonce(Option<aead::Nonce>);
impl aead::NonceSequence for SingleNonce {
    fn advance(&mut self) -> Result<aead::Nonce, aws_lc_rs::error::Unspecified> {
        self.0.take().ok_or(aws_lc_rs::error::Unspecified)
    }
}
