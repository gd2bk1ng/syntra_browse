/* ================================================================================================
   SYNTRA BROWSER — INPUT HANDLER
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/input_handler.rs
   Module:      Async Input Event Processing and Behavioral Monitoring Integration
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Handles keyboard input events asynchronously, records behavioral data,
                triggers session lock with UI lock screen on anomaly detection,
                and unlocks session with adaptive learning after successful re-authentication.
   ================================================================================================ */

use crate::browser::behavioral_monitor::{BehavioralMonitor, TypingEvent};
use crate::ui::lock_screen;
use crate::browser::session_manager::SessionManager;

/// Processes a single keyboard event asynchronously.
///
/// # Arguments
/// * `event` - The captured typing event.
/// * `monitor` - Mutable reference to the behavioral monitor.
/// * `session_manager` - Reference to the session manager.
/// * `challenge` - Challenge nonce used for cryptographic re-authentication.
pub async fn on_key_event(
    event: TypingEvent,
    monitor: &mut BehavioralMonitor,
    session_manager: &SessionManager,
    challenge: &[u8],
) {
    if session_manager.is_locked().await {
        // If session is locked, ignore input or optionally redirect to lock screen UI
        return;
    }

    monitor.record_event(event);

    if !monitor.evaluate() {
        println!("⚠️ Behavioral anomaly detected. Locking session...");
        session_manager.lock().await;

        // Trigger the async UI lock screen with the cryptographic challenge,
        // passing mutable reference to behavioral_monitor for adaptive learning
        lock_screen::show_lock_screen(challenge, session_manager, monitor).await;

        // Additional logic to pause input or background tasks can be added here
    }
}
