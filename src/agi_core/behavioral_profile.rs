// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (BEHAVIORAL PROFILE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/behavioral_profile.rs
//   Module:      Behavioral Profile
//   Description: Models creator-specific behavioral patterns such as typing cadence, latency,
//                and other biometric-style interaction signals. Used by the behavioral monitor
//                and session manager to detect anomalies.
//
//   Notes:
//     - This subsystem is intentionally lightweight and ASCII-safe.
//     - Future versions may integrate statistical models, ML embeddings, or predictive engines.
// ================================================================================================

/// Represents a typing pattern extracted from user input.
/// Future expansions may include:
///   - average key hold time
///   - inter-key latency distribution
///   - error rate patterns
///   - rhythm signatures
#[derive(Debug, Clone, Default)]
pub struct TypingPattern {
    pub avg_hold_ms: f32,
    pub avg_latency_ms: f32,
    pub sample_count: u64,
}

/// Represents the stored behavioral profile of the creator.
/// This is used to detect anomalies or unauthorized usage.
#[derive(Debug, Clone, Default)]
pub struct CreatorProfile {
    pub typing_pattern: TypingPattern,
}

impl CreatorProfile {
    /// Load the creator profile from disk or return a default profile.
    /// In future versions, this may load from:
    ///   - encrypted local storage
    ///   - continuity subsystem
    ///   - distributed identity service
    pub async fn load() -> Self {
        // Placeholder: return default profile
        // Future: load from persistence layer
        Self::default()
    }

    /// Compare a live typing pattern to the stored profile.
    /// Returns true if the deviation is within acceptable bounds.
    pub fn matches(&self, live: &TypingPattern) -> bool {
        // Simple heuristic placeholder:
        // Future: statistical deviation, ML classifier, predictive modeling
        let hold_diff = (self.typing_pattern.avg_hold_ms - live.avg_hold_ms).abs();
        let latency_diff = (self.typing_pattern.avg_latency_ms - live.avg_latency_ms).abs();

        hold_diff < 50.0 && latency_diff < 80.0
    }

    /// Update the stored profile with new data.
    /// Future versions may use:
    ///   - exponential smoothing
    ///   - adaptive learning
    ///   - confidence-weighted updates
    pub fn update(&mut self, new: &TypingPattern) {
        // Simple averaging placeholder
        self.typing_pattern.avg_hold_ms =
            (self.typing_pattern.avg_hold_ms + new.avg_hold_ms) / 2.0;

        self.typing_pattern.avg_latency_ms =
            (self.typing_pattern.avg_latency_ms + new.avg_latency_ms) / 2.0;

        self.typing_pattern.sample_count += new.sample_count;
    }
}
