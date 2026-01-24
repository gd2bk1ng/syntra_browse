/* ================================================================================================
   SYNTRA BROWSER — AXIOM FIVE + SIX + SEVEN
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/agi_core/mod.rs
   Module:      AGI Core — Unified Cognition
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Unified AGI Core for Syntra. Merges:
                  • Axiom Three  — simple reasoning interface
                  • Axiom Five   — semantic intent engine
                  • Axiom Six    — self‑modification proposal engine
                  • Axiom Seven  — safety & governance layer
                Provides a dual-interface Reasoner trait and a Cortex-level ThoughtStream.

   Overview:
     • intent.rs        — Intent, IntentPlan, classifier, planner, JSON escaping, IntentLog.
     • reasoner.rs      — Reasoner trait + NullReasoner / ProbReasoner implementations.
     • ecosystem.rs     — EcosystemLobe + EcosystemModel (structural health model + scan).
     • self_mod.rs      — SelfModEngine + change proposals, refactors, patch hints.
     • safety.rs        — SafetyPolicy, SafetyRule, SafetyGate, SafetyVerdict.
     • ThoughtStream    — Cortex-level thought stream built on IntentLog.
   ================================================================================================ */

#![allow(dead_code)]

pub mod intent;
pub mod reasoner;
pub mod ecosystem;
pub mod self_mod;
pub mod safety;

/* ------------------------------------------------------------------------------------------------
   PUBLIC EXPORTS
   ------------------------------------------------------------------------------------------------ */

pub use intent::{
    classify_domain, escape_json, multi_step_plan, plan_for_domain, Intent, IntentLog, IntentPlan,
    debug_plan,
};

pub use reasoner::{NullReasoner, ProbReasoner, Reasoner};

pub use ecosystem::{EcosystemLobe, EcosystemModel};

pub use self_mod::{
    ChangeKind, ChangeProposal, CircularDependency, DeadCodeReport, EvolutionPlan,
    RefactorSuggestion, SelfModEngine,
};

pub use safety::{
    SafetyLevel, SafetyPolicy, SafetyRule, SafetyVerdict, SafetyGate,
};

/* ------------------------------------------------------------------------------------------------
   THOUGHT STREAM (Cortex-Level)
   ------------------------------------------------------------------------------------------------ */

use intent::{IntentLog as CoreIntentLog, IntentPlan as CoreIntentPlan};

/// Central thought stream for Syntra.
/// Collects IntentPlan entries from the AGI Core and exposes them for introspection.
///
/// Design:
///   - Read-only from the outside (no mutation without explicit push).
///   - Bounded capacity to avoid unbounded memory growth.
///   - Intended as a mirror of Syntra's recent "thoughts".
#[derive(Debug)]
pub struct ThoughtStream {
    log: CoreIntentLog,
}

impl ThoughtStream {
    /// Create a new thought stream with a fixed maximum number of entries.
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
