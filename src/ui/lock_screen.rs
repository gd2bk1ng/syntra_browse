/* ================================================================================================
   SYNTRA BROWSER — LOCK SCREEN UI
   ------------------------------------------------------------------------------------------------
   File:        src/ui/lock_screen.rs
   Module:      UI Lock Screen Component
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a lock screen UI displayed when behavioral monitoring detects
                unauthorized access. Prompts the user to re-authenticate.
   ================================================================================================ */

use std::io::{self, Write};

/// Displays a simple terminal lock screen and prompts for password re-authentication.
///
/// Replace this with your actual UI framework code.
pub fn show_lock_screen() {
    println!("\n🔒 Session Locked due to inactivity or unauthorized access.");
    println!("Please re-authenticate to continue.\n");

    // Simple password prompt example (replace with secure input and UI)
    print!("Enter password: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        println!("Failed to read input. Try again.");
        return;
    }

    let password = input.trim();

    if verify_password(password) {
        println!("✅ Authentication successful. Unlocking session...");
        // Call session unlock logic here
    } else {
        println!("❌ Incorrect password. Session remains locked.");
        // Optionally retry or exit
    }
}

/// Dummy password verification function.
/// Replace with your secure authentication logic (e.g., cryptographic key check).
fn verify_password(input: &str) -> bool {
    const CORRECT_PASSWORD: &str = "syntra_creator_password"; // Replace securely!
    input == CORRECT_PASSWORD
}
