/// Simple key pair generator for modem-license.
/// Run with: cargo run --package modem-license --example gen-keys

use ring::signature::{Ed25519KeyPair, KeyPair};
use std::fs;

fn main() {
    // Generate Ed25519 key pair
    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
        .expect("Failed to generate key pair");

    // Extract public key from PKCS#8
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
        .expect("Failed to parse generated key");
    let public_key = key_pair.public_key().as_ref();

    // Write private key (PKCS#8 format)
    fs::write("keys/modem-cat.sk", pkcs8_bytes.as_ref())
        .expect("Failed to write private key");

    // Print public key in Rust array format
    println!("Private key written to keys/modem-cat.sk");
    println!("\nPublic key (copy to modem-license/src/lib.rs PUBLIC_KEY_BYTES):");
    print!("[");
    for (i, &byte) in public_key.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        if i % 8 == 0 && i > 0 {
            println!();
            print!("    ");
        }
        print!("0x{:02X}", byte);
    }
    println!(",\n];");
}
