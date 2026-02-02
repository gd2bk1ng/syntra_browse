// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT SEMANTICS ENGINE, CORE TYPES)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/intent.rs
//   Module:      Intent Semantics Engine — Core Types
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Core data structures and traits for the Syntra Kernel Intent Semantics Engine.
//       This module defines the primary intent representations, planning graph, and
//       reasoning interface. Feature extraction, embeddings, classification, planning,
//       and logging live in sibling modules:
//
//         • features.rs     — Deterministic semantic feature extraction.
//         • embedding.rs    — Symbolic intent embeddings.
//         • classifier.rs   — Hierarchical, feature-aware domain classification.
//         • planner.rs      — Narrative + multi-step planning and plan graphs.
//         • reasoner.rs     — Concrete Reasoner implementations.
//         • intent_log.rs   — Ring buffer for recent IntentPlans.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (intent understanding).
//       • Axiom Four  — Perception / Cognition / Action routing.
//       • Axiom Six   — Evolution (intents that drive self-modification).
//
//   Notes:
//       - This file is intentionally focused on *types* and *interfaces*.
//       - Logic lives in the sibling modules to keep the AGI Core maintainable.
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::agi_core::embedding::IntentEmbedding;
use crate::agi_core::features::IntentFeatures;

// ================================================================================================
// Core Intent Structures
// ================================================================================================

pub struct IntentLog; // TODO: flesh out later

/// Raw user intent with confidence metadata.
///
/// This is the minimal unit passed into the reasoning engine. The `label`
/// is typically the raw text from the user; `confidence` can be used by
/// upstream systems to indicate how certain they are that this text is
/// indeed the primary intent (e.g., after pre-parsing).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Raw intent text (usually user input).
    pub label: String,
    /// Upstream confidence in this intent (0.0–1.0).
    pub confidence: f32,
}

impl Intent {
    /// Construct a new intent from a label and confidence.
    pub fn new(label: impl Into<String>, confidence: f32) -> Self {
        Self {
            label: label.into(),
            confidence,
        }
    }
}

/// Lightweight contextual state for reasoning.
///
/// This is intentionally minimal and ASCII-safe. It allows the Reasoner
/// to incorporate recent history and system state without depending on
/// heavy-weight global singletons.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    /// Recent intent labels (most recent last).
    pub recent_intents: Vec<String>,
    /// Optional active task name or identifier.
    pub active_task: Option<String>,
    /// Normalized system load (0.0–1.0).
    pub system_load: f32,
}

/// A node in a graph-based plan.
///
/// Plans can be represented as a DAG of nodes, where each node describes
/// a step and edges indicate possible transitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanNode {
    /// Node identifier (index into the plan graph).
    pub id: usize,
    /// Human-readable description of this step.
    pub description: String,
    /// Indices of next nodes (edges).
    pub next: Vec<usize>,
}

/// Classified intent with semantic domain + high-level plan.
///
/// This is the primary output of the Reasoner. It is designed to be:
///   • Human-readable
///   • Machine-consumable
///   • Stable across versions (backwards compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentPlan {
    /// Original intent text.
    pub intent: String,
    /// High-level semantic class (e.g., "browse", "knowledge", "task").
    pub class: String,
    /// Narrative description of the plan.
    pub plan: String,
    /// Linearized multi-step plan (for compatibility with simple callers).
    pub steps: Vec<String>,
    /// Probability estimate (0.0–1.0) for classification confidence.
    pub probability: f32,
    /// Extracted features used during reasoning.
    pub features: IntentFeatures,
    /// Symbolic embedding for similarity and clustering.
    pub embedding: IntentEmbedding,
    /// Optional graph-based plan representation.
    pub graph: Vec<PlanNode>,
    /// Optional alternative classifications for disambiguation.
    pub alternatives: Vec<(String, f32)>,
}

// ================================================================================================
// Reasoner Trait
// ================================================================================================

/// Pluggable reasoning engine interface.
///
/// Implementations may be:
///   • Deterministic (NullReasoner)
///   • Probabilistic (ProbReasoner)
///   • Remote (RPC-backed models)
pub trait Reasoner {
    /// Full semantic reasoning with optional context.
    fn process(&self, intent: Intent, ctx: Option<&Context>) -> IntentPlan;

    /// Simple text reasoning (Axiom Three compatibility).
    fn reason_text(&self, intent: &Intent) -> String {
        self.process(intent.clone(), None).plan
    }
}

// ================================================================================================
// Debug / Introspection Helpers
// ================================================================================================

/// Produce a one-line debug summary of an IntentPlan.
///
/// Useful for overlays, logs, and terminal diagnostics.
pub fn debug_plan(plan: &IntentPlan) -> String {
    let alt_summary = if plan.alternatives.is_empty() {
        "alts=[]".to_string()
    } else {
        let parts: Vec<String> = plan
            .alternatives
            .iter()
            .map(|(c, p)| format!("{}:{:.2}", c, p))
            .collect();
        format!("alts=[{}]", parts.join(","))
    };

    format!(
        "[IntentPlan] class='{}' prob={:.2} steps={} {} intent='{}'",
        plan.class,
        plan.probability,
        plan.steps.len(),
        alt_summary,
        plan.intent.replace('\n', " ")
    )
}
