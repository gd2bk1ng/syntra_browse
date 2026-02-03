// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT CLASSIFIER)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/classifier/mod.rs
//   Module:      Intent Classifier (Hierarchical, Feature-Aware)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Deterministic, explainable semantic classification for intents. Maps extracted features
//       and optional context into a high-level semantic domain. Also provides lightweight
//       probability estimation, alternative domain suggestions, and a structured domain taxonomy.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (semantic interpretation).
//       • Axiom Four  — Routing between perception / cognition / action lobes.
//       • Axiom Six   — Evolution (meta-intents for self-modification).
//
//   Notes:
//       - No ML, no randomness — fully deterministic.
//       - Domain taxonomy aligns with Cortex, Terminal, and ThoughtStream.
//       - This module is now folder-based and extensible.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::features::IntentFeatures;
use crate::agi_core::intent::{Context, Intent};

// ================================================================================================
// Domain Taxonomy (Structured)
// ================================================================================================

/// High-level semantic domain for an intent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainClass {
    Browse,
    Knowledge,
    Task,
    Evolve,
    Sandbox,
    Introspection,
    Ecosystem,
    Safety,
    Freeform,
    Unknown(String),
}

impl DomainClass {
    pub fn as_str(&self) -> &str {
        match self {
            DomainClass::Browse => "browse",
            DomainClass::Knowledge => "knowledge",
            DomainClass::Task => "task",
            DomainClass::Evolve => "evolve",
            DomainClass::Sandbox => "sandbox",
            DomainClass::Introspection => "introspection",
            DomainClass::Ecosystem => "ecosystem",
            DomainClass::Safety => "safety",
            DomainClass::Freeform => "freeform",
            DomainClass::Unknown(s) => s.as_str(),
        }
    }
}

// ================================================================================================
// Classification Result (Upgraded)
// ================================================================================================

/// Result of a classification pass.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ClassificationResult {
    /// Primary semantic class.
    pub class: DomainClass,
    /// Probability estimate (0.0–1.0).
    pub probability: f32,
    /// Alternative candidate classes with scores.
    pub alternatives: Vec<(DomainClass, f32)>,
}

// ================================================================================================
// Primary Domain Classifier (Your Logic + Structured API)
// ================================================================================================

/// Hierarchical, feature-aware domain classification.
///
/// Deterministic and explainable. Uses:
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
        return result(
            DomainClass::Browse,
            0.95,
            vec![(DomainClass::Knowledge, 0.4)],
        );
    }
    if lower.starts_with("knowledge ") || lower.starts_with("search ") {
        return result(
            DomainClass::Knowledge,
            0.9,
            vec![(DomainClass::Browse, 0.5)],
        );
    }
    if lower.starts_with("task ") || lower.starts_with("run ") || lower.contains("execute") {
        return result(
            DomainClass::Task,
            0.9,
            vec![(DomainClass::Freeform, 0.3)],
        );
    }
    if lower.starts_with("evolve ") || lower.contains("self-mod") || lower.contains("self mod") {
        return result(
            DomainClass::Evolve,
            0.9,
            vec![(DomainClass::Sandbox, 0.6)],
        );
    }
    if lower.starts_with("sandbox ") {
        return result(
            DomainClass::Sandbox,
            0.9,
            vec![(DomainClass::Evolve, 0.6)],
        );
    }
    if lower.starts_with("thoughts") {
        return result(
            DomainClass::Introspection,
            0.9,
            vec![(DomainClass::Freeform, 0.7)],
        );
    }
    if lower.starts_with("ecosystem") {
        return result(
            DomainClass::Ecosystem,
            0.9,
            vec![(DomainClass::Freeform, 0.5)],
        );
    }
    if lower.starts_with("safety") {
        return result(
            DomainClass::Safety,
            0.9,
            vec![(DomainClass::Freeform, 0.4)],
        );
    }

    // --- Question bias → knowledge -------------------------------------------------------------
    if features.is_question {
        return result(
            DomainClass::Knowledge,
            0.75,
            vec![(DomainClass::Freeform, 0.4)],
        );
    }

    // --- Contextual nudges ---------------------------------------------------------------------
    if let Some(ctx) = ctx {
        if ctx.active_task.is_some() {
            return result(
                DomainClass::Task,
                0.7,
                vec![(DomainClass::Freeform, 0.4)],
            );
        }
    }

    // --- Default: freeform conversational / creative -------------------------------------------
    result(
        DomainClass::Freeform,
        0.6,
        vec![(DomainClass::Knowledge, 0.3)],
    )
}

// ================================================================================================
// Probability Refinement (Your Logic, Upgraded Types)
// ================================================================================================

/// Helper to construct a classification result.
fn result(
    class: DomainClass,
    prob: f32,
    alts: Vec<(DomainClass, f32)>,
) -> ClassificationResult {
    ClassificationResult {
        class,
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
    alternatives: &[(DomainClass, f32)],
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

// ================================================================================================
// High-Level Classifier Wrapper (New)
// ================================================================================================

/// Configurable domain classifier.
#[derive(Debug, Clone)]
pub struct DomainClassifier {
    pub name: String,
    pub version: String,
    pub threshold: f32,
}

impl Default for DomainClassifier {
    fn default() -> Self {
        Self {
            name: "SyntraDomainClassifier".into(),
            version: "0.1.0".into(),
            threshold: 0.5,
        }
    }
}

impl DomainClassifier {
    pub fn new(name: impl Into<String>, version: impl Into<String>, threshold: f32) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            threshold,
        }
    }

    /// Full classification pipeline.
    pub fn classify(&self, intent: &Intent, features: &IntentFeatures) -> ClassificationResult {
        let mut result = classify_domain(&intent.text, features, intent.context.as_ref());

        result.probability = refine_probability(
            result.probability,
            features,
            intent.context.as_ref(),
            &result.alternatives,
        );

        result
    }

    /// Confidence gate.
    pub fn is_confident(&self, result: &ClassificationResult) -> bool {
        result.probability >= self.threshold
    }
}
