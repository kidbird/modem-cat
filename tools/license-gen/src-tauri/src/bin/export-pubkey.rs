/// CLI tool to display the embedded public key.
/// Since the private key is now embedded in modem-license crate,
/// this tool simply extracts and displays the corresponding public key.

fn main() {
    // Use the embedded private key directly
    let pkcs8_bytes = modem_license::get_embedded_private_key()
        .unwrap_or_else(|e| {
            eprintln!("Failed to get embedded private key: {}", e);
            std::process::exit(1);
        });

    let pub_bytes = modem_license::extract_public_key_from_pkcs8(&pkcs8_bytes)
        .unwrap_or_else(|e| {
            eprintln!("Failed to extract public key: {}", e);
            std::process::exit(1);
        });

    println!("// Embedded public key (matches PUBLIC_KEY_BYTES in modem-license/src/lib.rs):");
    println!("pub const PUBLIC_KEY_BYTES: [u8; 32] = [");
    for (i, chunk) in pub_bytes.chunks(8).enumerate() {
        print!("    ");
        for (j, byte) in chunk.iter().enumerate() {
            print!("0x{:02X}", byte);
            if i < 3 || j < 7 {
                print!(", ");
            }
        }
        println!();
    }
    println!("];");
}
