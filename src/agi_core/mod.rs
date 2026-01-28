// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (UNIFIED COGNITION)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/mod.rs
//   Module:      AGI Core — Unified Cognition
//   Description: Central cognitive subsystem of the Syntra Kernel. Integrates:
//                  • Axiom Three  — foundational reasoning interface
//                  • Axiom Five   — semantic intent engine
//                  • Axiom Six    — self‑modification & evolution engine
//                  • Axiom Seven  — safety & governance layer
//
//                Provides:
//                  • Intent engine (classification, planning, logging)
//                  • Reasoner trait with multiple implementations
//                  • Ecosystem health model & structural scanning
//                  • Self‑modification proposal engine
//                  • Safety policy enforcement
//                  • Cortex-level ThoughtStream for introspection
//
//   Notes:
//     - This subsystem is intentionally modular and future‑proof.
//     - Designed for integration with predictive, continuity, and knowledge subsystems.
// ================================================================================================

#![allow(dead_code)]

pub mod intent;
pub mod reasoner;
pub mod ecosystem;
pub mod self_mod;
pub mod safety;

// ================================================================================================
// Public Exports
// ================================================================================================

pub use intent::{
    classify_domain, debug_plan, escape_json, multi_step_plan, plan_for_domain, Intent, IntentLog,
    IntentPlan,
};

pub use reasoner::{NullReasoner, ProbReasoner, Reasoner};

pub use ecosystem::{EcosystemLobe, EcosystemModel};

pub use self_mod::{
    ChangeKind, ChangeProposal, CircularDependency, DeadCodeReport, EvolutionPlan,
    RefactorSuggestion, SelfModEngine,
};

pub use safety::{SafetyGate, SafetyLevel, SafetyPolicy, SafetyRule, SafetyVerdict};

// ================================================================================================
// ThoughtStream (Cortex-Level Introspection)
// ================================================================================================

use intent::{IntentLog as CoreIntentLog, IntentPlan as CoreIntentPlan};

/// The ThoughtStream is a Cortex-level introspection buffer that mirrors
/// Syntra Kernel’s recent cognitive activity.
///
/// It stores a bounded sequence of `IntentPlan` entries, allowing the UI,
/// diagnostics, or higher-level cognitive lobes to inspect the system’s
/// reasoning history.
///
/// Design Principles:
///   - Read-only from the outside (immutable access).
///   - Bounded capacity to prevent unbounded memory growth.
///   - Represents the “recent thoughts” of the AGI Core.
#[derive(Debug)]
pub struct ThoughtStream {
    log: CoreIntentLog,
}

impl ThoughtStream {
    /// Create a new ThoughtStream with a fixed maximum number of entries.
    pub fn new(capacity: usize) -> Self {
        Self {
            log: CoreIntentLog::new(capacity),
        }
    }

    /// Push a new plan into the stream.
    pub fn push(&mut self, plan: CoreIntentPlan) {
        self.log.push(plan);
    }

    /// Get the most recent thought, if any.
    pub fn latest(&self) -> Option<&CoreIntentPlan> {
        self.log.latest()
    }

    /// Get all stored thoughts (oldest → newest).
    pub fn all(&self) -> &[CoreIntentPlan] {
        self.log.entries()
    }
}
