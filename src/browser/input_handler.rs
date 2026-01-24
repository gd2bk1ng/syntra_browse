/* ================================================================================================
   SYNTRA BROWSER — INPUT HANDLER
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/input_handler.rs
   Module:      Input Event Processing and Behavioral Monitoring Integration
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Handles keyboard input events, records behavioral data, and triggers session lock
                on anomaly detection.
   ================================================================================================ */

use crate::browser::behavioral_monitor::{BehavioralMonitor, TypingEvent};
use crate::ui::lock_screen;
use crate::browser::session_manager::SessionManager;

fn on_key_event(event: TypingEvent, monitor: &mut BehavioralMonitor, session_manager: &SessionManager, challenge: &[u8]) {
    if session_manager.is_locked() {
        // If session is locked, ignore input or redirect to lock screen UI
        return;
    }

    monitor.record_event(event);

    if !monitor.evaluate() {
        println!("⚠️ Behavioral anomaly detected. Locking session...");
        session_manager.lock();
        lock_screen::show_lock_screen(challenge);
        // Additional logic to pause input or background tasks can go here
    }
}
