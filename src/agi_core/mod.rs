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
//   Notes:
//     - This subsystem is intentionally modular and future‑proof.
//     - All AGI Core modules are declared here for clarity and stability.
// ================================================================================================

#![allow(dead_code)]

// ================================================================================================
// Submodules — every file in src/agi_core/
// ================================================================================================

pub mod behavioral_profile;
pub mod classifier;
pub mod commands;
pub mod ecosystem;
pub mod ecosystem_model;
pub mod embedding;
pub mod events;
pub mod features;
pub mod feedback;
pub mod intent;
pub mod intent_log;
pub mod node_rs;
pub mod planner;
pub mod providers;
pub mod reasoner;
pub mod routing;
pub mod safety;
pub mod self_mod;
pub mod telemetry;
pub mod theme;

// ================================================================================================
// Public Exports — clean, correct, no missing symbols
// ================================================================================================

// Intent system
pub use intent::{debug_plan, Intent, IntentPlan, Context, PlanNode};
pub use intent_log::IntentLog;

// pub use intent::{debug_plan, Intent, IntentLog, IntentPlan, Context, PlanNode}; ( optional: needs further testing..)


// Reasoner trait must be exported from its defining module (intent.rs)
// pub use crate::agi_core::intent::Reasoner; (optional: needs further testing... )
pub use intent::Reasoner;

// Concrete Reasoner implementations
pub use reasoner::{NullReasoner, ProbReasoner};

// Ecosystem
pub use ecosystem::{EcosystemLobe, EcosystemModel};

// Behavioral profiles
pub use behavioral_profile::{CreatorProfile, TypingPattern};

// Self‑modification engine
pub use self_mod::{
    ChangeKind, ChangeProposal, CircularDependency, DeadCodeReport, EvolutionPlan,
    RefactorSuggestion, SelfModEngine,
};

// Safety & governance
pub use safety::{SafetyGate, SafetyLevel, SafetyPolicy, SafetyRule, SafetyVerdict};

// Telemetry
pub use telemetry::{TelemetryBus, TelemetryEvent};

// Providers (model backends, RPC, etc.)
pub use providers::{ProviderKind, ProviderConfig};

// Events (intent events, AGI events)
pub use events::{AgiEvent, IntentEvent};

// Feedback (reinforcement, corrections)
pub use feedback::{FeedbackSignal, FeedbackKind};

// Classifier (semantic domain classification)
pub use classifier::{DomainClassifier, DomainClass};

// Planner (narrative + multi‑step planning)
pub use planner::{PlanGraph, PlannerEngine};

// Routing (intent → subsystem routing)
pub use routing::{RouteDecision, Router};

// Theme (UI/semantic theme metadata)
pub use theme::{AgiTheme, ThemeColor};

// Commands (AGI control commands)
pub use commands::{AgiCommand, CommandResult};

// ================================================================================================
// ThoughtStream (Cortex-Level Introspection)
// ================================================================================================

use intent::{IntentLog as CoreIntentLog, IntentPlan as CoreIntentPlan};

#[derive(Debug)]
pub struct ThoughtStream {
    log: CoreIntentLog,
}

impl ThoughtStream {
    pub fn new(capacity: usize) -> Self {
        Self {
            log: CoreIntentLog::new(capacity),
        }
    }

    pub fn push(&mut self, plan: CoreIntentPlan) {
        self.log.push(plan);
    }

    pub fn latest(&self) -> Option<&CoreIntentPlan> {
        self.log.latest()
    }

    pub fn all(&self) -> &[CoreIntentPlan] {
        self.log.entries()
    }
}
