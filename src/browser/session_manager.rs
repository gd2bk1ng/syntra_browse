/* ================================================================================================
   SYNTRA BROWSER — SESSION MANAGER
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/session_manager.rs
   Module:      Session Lifecycle Management
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Manages session lock/unlock states and coordinates with UI and behavioral monitoring.
   ================================================================================================ */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct SessionManager {
    locked: Arc<AtomicBool>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            locked: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn lock(&self) {
        self.locked.store(true, Ordering::SeqCst);
        println!("Session locked.");
        // Additional logic: disable inputs, pause tasks, notify UI
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::SeqCst);
        println!("Session unlocked.");
        // Additional logic: re-enable inputs, resume tasks, notify UI
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}
