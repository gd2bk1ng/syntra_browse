pub struct SessionManager;

impl SessionManager {
    pub fn lock() {
        // Lock session, disable input, etc.
        println!("Session locked due to behavioral mismatch.");
    }

    pub fn unlock() {
        // Unlock after successful re-authentication
    }
}
