/* ================================================================================================
   SYNTRA BROWSER — LOCK SCREEN UI
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/ui/lock_screen.rs
   Module:      Async UI Lock Screen Component with Session Unlock Flow
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Displays a lock screen, prompts for cryptographic signature re-authentication,
                unlocks the session on success, and triggers adaptive learning update.
   ================================================================================================ */

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use ring::signature::UnparsedPublicKey;
use ring::signature::ED25519;
use data_encoding::BASE64;

use crate::browser::session_manager::SessionManager;
use crate::agi_core::behavioral_profile::{CreatorProfile, TypingPattern};

const CREATOR_PUBLIC_KEY_BASE64: &str = "YOUR_BASE64_ENCODED_PUBLIC_KEY_HERE";

/// Shows the lock screen, prompts for signature asynchronously, verifies, unlocks session,
/// and updates behavioral profile adaptively.
pub async fn show_lock_screen(
    challenge: &[u8],
    session_manager: &SessionManager,
    behavioral_monitor: &mut BehavioralMonitor,
) {
    println!("\n🔒 Session Locked. Please sign the following challenge to unlock:\n");
    println!("{}", BASE64.encode(challenge));
    print!("Enter base64-encoded signature: ");
    io::stdout().flush().await.unwrap();

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut signature_b64 = String::new();
    if let Err(_) = reader.read_line(&mut signature_b64).await {
        println!("Failed to read input. Try again.");
        return;
    }
    let signature_b64 = signature_b64.trim();

    if verify_signature(challenge, signature_b64) {
        println!("✅ Authentication successful. Unlocking session...");
        session_manager.unlock().await;

        // Optional: Update behavioral profile adaptively after unlock
        let live_pattern = behavioral_monitor.extract_typing_pattern();
        behavioral_monitor.profile.adapt(&live_pattern, 0.05);
        behavioral_monitor.profile.save().await;

        println!("Behavioral profile updated. You may resume.");
    } else {
        println!("❌ Invalid signature. Session remains locked.");
        // Optionally retry or exit
    }
}

/// Verifies the signed challenge.
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
