//! Key Exchange Example
//! Demonstrates X25519 ECDH key agreement between two parties

use aws_lc_rs::{agreement, rand, error::Unspecified};

fn main() {
    println!("🔑 Key Exchange Example (X25519 ECDH)\n");

    let rng = rand::SystemRandom::new();

    // Alice generates her key pair
    println!("Alice generates her key pair...");
    let alice_private = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
        .expect("Failed to generate Alice's private key");
    let alice_public = alice_private.compute_public_key()
        .expect("Failed to compute Alice's public key");
    println!("  Alice's public key: {:02x?}...\n", &alice_public.as_ref()[..16]);

    // Bob generates his key pair
    println!("Bob generates his key pair...");
    let bob_private = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
        .expect("Failed to generate Bob's private key");
    let bob_public = bob_private.compute_public_key()
        .expect("Failed to compute Bob's public key");
    println!("  Bob's public key: {:02x?}...\n", &bob_public.as_ref()[..16]);

    // Alice computes the shared secret using Bob's public key
    println!("Alice computes shared secret using Bob's public key...");
    let alice_shared = agreement::agree_ephemeral(
        alice_private,
        &agreement::UnparsedPublicKey::new(&agreement::X25519, bob_public.as_ref()),
        Unspecified,
        |shared_secret: &[u8]| Ok(shared_secret.to_vec()),
    ).expect("Alice's key agreement failed");
    println!("  Alice's shared secret: {:02x?}...\n", &alice_shared[..16]);

    // Bob computes the shared secret using Alice's public key
    println!("Bob computes shared secret using Alice's public key...");
    let bob_shared = agreement::agree_ephemeral(
        bob_private,
        &agreement::UnparsedPublicKey::new(&agreement::X25519, alice_public.as_ref()),
        Unspecified,
        |shared_secret: &[u8]| Ok(shared_secret.to_vec()),
    ).expect("Bob's key agreement failed");
    println!("  Bob's shared secret: {:02x?}...\n", &bob_shared[..16]);

    // Verify both parties computed the same shared secret
    if alice_shared == bob_shared {
        println!("✅ Success! Both parties derived the same shared secret.");
        println!("   This secret can now be used to derive encryption keys.");
    } else {
        println!("❌ Error: Shared secrets don't match!");
        std::process::exit(1);
    }
}
