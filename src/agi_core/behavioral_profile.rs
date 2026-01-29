// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (BEHAVIORAL PROFILE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/behavioral_profile.rs
//   Module:      Behavioral Profile (Axiom Two / Axiom Three Support Layer)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Models creator-specific behavioral signatures such as typing cadence, latency,
//       and interaction rhythm. These signals are used by the Behavioral Monitor and
//       Session Manager to detect anomalies, unauthorized access, or deviations from
//       expected creator patterns.
//
//       This subsystem is intentionally lightweight and ASCII-safe. It is designed to
//       evolve into a more sophisticated biometric/behavioral identity engine as the
//       AGI Core matures.
//
//   Architectural Role:
//       • Supports Axiom Two (Identity & Continuity).
//       • Supports Axiom Three (Behavioral Awareness).
//       • Provides a stable behavioral baseline for anomaly detection.
//       • Integrates with the Browser UI, Session Manager, and future Continuity Engine.
//
//   Future Extensions:
//       • Statistical deviation modeling
//       • ML-based behavioral embeddings
//       • Predictive typing rhythm models
//       • Encrypted persistence layer
//       • Multi-device behavioral fusion
//
// ================================================================================================

/// Represents a typing pattern extracted from user input.
///
/// This structure is intentionally minimal but designed to expand.
/// Future versions may include:
///   - Key hold time distributions
///   - Inter-key latency histograms
///   - Error rate signatures
///   - Burst typing detection
///   - Circadian rhythm typing variance
#[derive(Debug, Clone, Default)]
pub struct TypingPattern {
    /// Average key hold duration in milliseconds.
    pub avg_hold_ms: f32,

    /// Average inter-key latency in milliseconds.
    pub avg_latency_ms: f32,

    /// Number of samples contributing to this pattern.
    pub sample_count: u64,
}

/// Represents the stored behavioral profile of the creator.
///
/// This profile acts as a behavioral "fingerprint" and is used to:
///   • Detect anomalies
///   • Validate session authenticity
///   • Adaptively learn the creator’s evolving typing style
///
/// The profile is intentionally conservative: it prefers false negatives
/// (allowing access) over false positives (locking out the creator).
#[derive(Debug, Clone, Default)]
pub struct CreatorProfile {
    /// The creator’s baseline typing pattern.
    pub typing_pattern: TypingPattern,
}

impl CreatorProfile {
    // --------------------------------------------------------------------------------------------
    // LOADING & PERSISTENCE
    // --------------------------------------------------------------------------------------------

    /// Load the creator profile from disk or return a default profile.
    ///
    /// Future versions may load from:
    ///   - Encrypted local storage
    ///   - Continuity subsystem (episodic memory)
    ///   - Distributed identity service
    ///   - Secure enclave or TPM-backed storage
    pub async fn load() -> Self {
        // Placeholder: return default profile.
        // Future: load from persistence layer.
        Self::default()
    }

    // --------------------------------------------------------------------------------------------
    // MATCHING & ANOMALY DETECTION
    // --------------------------------------------------------------------------------------------

    /// Compare a live typing pattern to the stored profile.
    ///
    /// Returns `true` if the deviation is within acceptable bounds.
    ///
    /// Behavior:
    ///   - If no baseline exists, treat the session as "learning" rather than anomalous.
    ///   - Uses simple absolute thresholds for now.
    ///   - Future versions will use statistical deviation or ML classifiers.
    pub fn matches(&self, live: &TypingPattern) -> bool {
        // No baseline → accept and begin learning.
        if self.typing_pattern.sample_count == 0 {
            return true;
        }

        let hold_diff = (self.typing_pattern.avg_hold_ms - live.avg_hold_ms).abs();
        let latency_diff = (self.typing_pattern.avg_latency_ms - live.avg_latency_ms).abs();

        // Placeholder thresholds.
        // Future: dynamic thresholds based on variance and confidence.
        hold_diff < 50.0 && latency_diff < 80.0
    }

    // --------------------------------------------------------------------------------------------
    // ADAPTIVE LEARNING
    // --------------------------------------------------------------------------------------------

    /// Update the stored profile with new data.
    ///
    /// Uses weighted averaging based on sample counts.
    /// Future versions may use:
    ///   - Exponential smoothing
    ///   - Confidence-weighted updates
    ///   - Outlier rejection
    ///   - Time-decayed learning
    pub fn update(&mut self, new: &TypingPattern) {
        if new.sample_count == 0 {
            return;
        }

        // If no baseline exists, adopt the new pattern directly.
        if self.typing_pattern.sample_count == 0 {
            self.typing_pattern = new.clone();
            return;
        }

        let total = self.typing_pattern.sample_count + new.sample_count;
        let w_old = self.typing_pattern.sample_count as f32 / total as f32;
        let w_new = new.sample_count as f32 / total as f32;

        self.typing_pattern.avg_hold_ms =
            self.typing_pattern.avg_hold_ms * w_old + new.avg_hold_ms * w_new;

        self.typing_pattern.avg_latency_ms =
            self.typing_pattern.avg_latency_ms * w_old + new.avg_latency_ms * w_new;

        self.typing_pattern.sample_count = total;
    }
}
