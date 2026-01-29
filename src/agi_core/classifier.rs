// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT CLASSIFIER)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/classifier.rs
//   Module:      Intent Classifier (Hierarchical, Feature-Aware)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Deterministic, explainable semantic classification for intents. Maps extracted features
//       and optional context into a high-level semantic domain. Also provides lightweight
//       probability estimation and alternative domain suggestions.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (semantic interpretation).
//       • Axiom Four  — Routing between perception / cognition / action lobes.
//       • Axiom Six   — Evolution (meta-intents for self-modification).
//
//   Notes:
//       - No ML, no randomness — fully deterministic.
//       - Domain taxonomy is aligned with Cortex + Terminal + ThoughtStream.
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::agi_core::features::IntentFeatures;
use crate::agi_core::intent::Context;

/// Result of a classification pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// Primary semantic class.
    pub class: String,
    /// Probability estimate (0.0–1.0).
    pub probability: f32,
    /// Alternative candidate classes with scores.
    pub alternatives: Vec<(String, f32)>,
}

// ================================================================================================
// Primary Domain Classifier
// ================================================================================================

/// Hierarchical, feature-aware domain classification.
///
/// This function is deterministic and explainable. It uses:
///   • Keyword patterns
///   • URL presence
///   • Question detection
///   • Context (when available)
pub fn classify_domain(
    text: &str,
    features: &IntentFeatures,
    ctx: Option<&Context>,
) -> ClassificationResult {
    let lower = text.to_lowercase();

    // --- Strong lexical prefixes ---------------------------------------------------------------
    if lower.starts_with("browse ") || features.has_url {
        return result("browse", 0.95, vec![("knowledge".into(), 0.4)]);
    }
    if lower.starts_with("knowledge ") || lower.starts_with("search ") {
        return result("knowledge", 0.9, vec![("browse".into(), 0.5)]);
    }
    if lower.starts_with("task ") || lower.starts_with("run ") || lower.contains("execute") {
        return result("task", 0.9, vec![("self".into(), 0.3)]);
    }
    if lower.starts_with("evolve ") || lower.contains("self-mod") || lower.contains("self mod") {
        return result("evolve", 0.9, vec![("sandbox".into(), 0.6)]);
    }
    if lower.starts_with("sandbox ") {
        return result("sandbox", 0.9, vec![("evolve".into(), 0.6)]);
    }
    if lower.starts_with("thoughts") {
        return result("introspection", 0.9, vec![("self".into(), 0.7)]);
    }
    if lower.starts_with("ecosystem") {
        return result("ecosystem", 0.9, vec![("self".into(), 0.5)]);
    }
    if lower.starts_with("safety") {
        return result("safety", 0.9, vec![("self".into(), 0.4)]);
    }

    // --- Question bias → knowledge -------------------------------------------------------------
    if features.is_question {
        return result("knowledge", 0.75, vec![("freeform".into(), 0.4)]);
    }

    // --- Contextual nudges ---------------------------------------------------------------------
    if let Some(ctx) = ctx {
        if ctx.active_task.is_some() {
            return result("task", 0.7, vec![("freeform".into(), 0.4)]);
        }
    }

    // --- Default: freeform conversational / creative -------------------------------------------
    result("freeform", 0.6, vec![("knowledge".into(), 0.3)])
}

// ================================================================================================
// Probability & Alternatives
// ================================================================================================

/// Helper to construct a classification result.
fn result(class: &str, prob: f32, alts: Vec<(String, f32)>) -> ClassificationResult {
    ClassificationResult {
        class: class.to_string(),
        probability: prob,
        alternatives: alts,
    }
}

/// Lightweight probability refinement.
///
/// Adjusts the base probability using:
///   • sentiment
///   • context
///   • presence of alternatives
pub fn refine_probability(
    base: f32,
    features: &IntentFeatures,
    ctx: Option<&Context>,
    alternatives: &[(String, f32)],
) -> f32 {
    let mut p = base;

    // Sentiment nudge.
    if features.sentiment > 0.3 {
        p += 0.05;
    } else if features.sentiment < -0.3 {
        p -= 0.05;
    }

    // Contextual nudge.
    if let Some(ctx) = ctx {
        if ctx.active_task.is_some() {
            p += 0.03;
        }
    }

    // Alternative penalty.
    if !alternatives.is_empty() {
        p -= 0.1;
    }

    p.clamp(0.0, 1.0)
}
