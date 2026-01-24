/* ================================================================================================
   SYNTRA BROWSER — LOCK SCREEN UI
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/ui/lock_screen.rs
   Module:      UI Lock Screen Component
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a lock screen UI displayed when behavioral monitoring detects
                unauthorized access. Prompts the user to re-authenticate using cryptographic
                challenge-response for secure unlocking.
   ================================================================================================ */

use std::io::{self, Write};
use ring::signature::UnparsedPublicKey;
use ring::signature::ED25519;
use data_encoding::BASE64;

const CREATOR_PUBLIC_KEY_BASE64: &str = "YOUR_BASE64_ENCODED_PUBLIC_KEY_HERE";

/// Displays a simple terminal lock screen and prompts for cryptographic signature re-authentication.
///
/// Replace this with your actual UI framework code.
pub fn show_lock_screen(challenge: &[u8]) {
    println!("\n🔒 Session Locked due to inactivity or unauthorized access.");
    println!("Please sign the following challenge to continue:\n");
    println!("{}", BASE64.encode(challenge));
    println!("\nEnter base64-encoded signature: ");

    io::stdout().flush().unwrap();

    let mut signature_input = String::new();
    if let Err(_) = io::stdin().read_line(&mut signature_input) {
        println!("Failed to read input. Try again.");
        return;
    }

    let signature_b64 = signature_input.trim();

    if verify_signature(challenge, signature_b64) {
        println!("✅ Authentication successful. Unlocking session...");
        // Call session unlock logic here
    } else {
        println!("❌ Invalid signature. Session remains locked.");
        // Optionally retry or exit
    }
}

/// Verifies the signed challenge (nonce) from user input.
fn verify_signature(challenge: &[u8], signature_b64: &str) -> bool {
    let public_key_bytes = match BASE64.decode(CREATOR_PUBLIC_KEY_BASE64.as_bytes()) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let public_key = UnparsedPublicKey::new(&ED25519, &public_key_bytes);

    let signature_bytes = match BASE64.decode(signature_b64.as_bytes()) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    public_key.verify(challenge, &signature_bytes).is_ok()
}
