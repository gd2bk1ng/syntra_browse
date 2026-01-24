/* ================================================================================================
   SYNTRA BROWSER — BEHAVIORAL MONITOR MODULE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/browser/behavioral_monitor.rs
   Module:      Behavioral Authentication
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Continuously monitors behavioral patterns such as typing rhythm to detect
                unauthorized users and trigger session locking as a failsafe mechanism.
   ================================================================================================ */

use crate::agi_core::behavioral_profile::{CreatorProfile, TypingPattern};

/// Represents a single typing event (key press or release) with timestamp.
#[derive(Debug, Clone)]
pub struct TypingEvent {
    pub key_code: u32,        // Key identifier (e.g., ASCII or scancode)
    pub event_type: KeyEventType,
    pub timestamp_ms: u128,   // Milliseconds since epoch or monotonic clock
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyEventType {
    KeyDown,
    KeyUp,
}

/// Maximum number of recent events stored for analysis
const MAX_BUFFER_SIZE: usize = 200;

pub struct BehavioralMonitor {
    profile: CreatorProfile,
    recent_typing_data: Vec<TypingEvent>,
}

impl BehavioralMonitor {
    /// Creates a new BehavioralMonitor with the given creator profile.
    pub fn new(profile: CreatorProfile) -> Self {
        Self {
            profile,
            recent_typing_data: Vec::with_capacity(MAX_BUFFER_SIZE),
        }
    }

    /// Records a new typing event.
    pub fn record_event(&mut self, event: TypingEvent) {
        self.recent_typing_data.push(event);
        if self.recent_typing_data.len() > MAX_BUFFER_SIZE {
            self.recent_typing_data.remove(0);
        }
    }

    /// Extracts a TypingPattern from recent events.
    fn extract_typing_pattern(&self) -> TypingPattern {
        // Example: compute average key hold times and inter-key latencies.
        // Implement your feature extraction logic here.
        TypingPattern::from_events(&self.recent_typing_data)
    }

    /// Evaluates if current behavior matches the creator profile.
    ///
    /// Returns `true` if behavior matches (authenticated),
    /// or `false` if anomaly detected (possible intruder).
    pub fn evaluate(&self) -> bool {
        let live_pattern = self.extract_typing_pattern();
        self.profile.matches(&live_pattern)
    }
}
