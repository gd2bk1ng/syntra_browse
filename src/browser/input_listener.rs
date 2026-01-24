/* ================================================================================================
   SYNTRA BROWSER — INPUT LISTENER
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/input_listener.rs
   Module:      Real Input Event Capture and Forwarding
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Captures real keyboard input events using winit and forwards them
                asynchronously to the behavioral monitoring system.
   ================================================================================================ */

use tokio::sync::mpsc::Sender;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoopWindowTarget,
};

use crate::browser::behavioral_monitor::{TypingEvent, KeyEventType};

/// Converts winit keyboard input to TypingEvent and sends it over the channel.
pub fn handle_window_event<T>(
    event: &WindowEvent,
    input_tx: &Sender<TypingEvent>,
    _event_loop: &EventLoopWindowTarget<T>,
) {
    if let WindowEvent::KeyboardInput { input, .. } = event {
        if let Some(virtual_keycode) = input.virtual_keycode {
            // Map ElementState to KeyEventType
            let event_type = match input.state {
                ElementState::Pressed => KeyEventType::KeyDown,
                ElementState::Released => KeyEventType::KeyUp,
            };

            // Compose TypingEvent (timestamp_ms can be generated via std::time or a monotonic clock)
            let timestamp_ms = current_timestamp_millis();

            let typing_event = TypingEvent {
                key_code: virtual_keycode as u32,
                event_type,
                timestamp_ms,
            };

            // Send asynchronously; ignore error if receiver dropped
            let _ = input_tx.try_send(typing_event);
        }
    }
}

/// Helper: get current timestamp in milliseconds (monotonic or system time)
fn current_timestamp_millis() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
