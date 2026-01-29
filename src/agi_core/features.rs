// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT FEATURES)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/features.rs
//   Module:      Intent Features (Deterministic Semantic Extraction)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Deterministic, ASCII-safe semantic feature extraction for the Intent Semantics Engine.
//       Converts raw intent text into a structured feature representation used by the classifier,
//       planner, and embedding modules.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (understanding structure of requests).
//       • Axiom Four  — Perception / Cognition / Action routing (verb/noun cues).
//
//   Notes:
//       - No ML, no randomness — fully explainable and stable.
//       - Safe to call from any lobe (no I/O, no global state).
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Symbolic feature representation of an intent.
///
/// These features are deterministic and explainable. They are used for:
///   • Domain classification
///   • Plan selection
///   • Similarity and clustering
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntentFeatures {
    /// Extracted verb-like tokens.
    pub verbs: Vec<String>,
    /// Extracted noun-like tokens.
    pub nouns: Vec<String>,
    /// Whether the text appears to contain a URL.
    pub has_url: bool,
    /// Whether the text appears to be a question.
    pub is_question: bool,
    /// Rough sentiment estimate (-1.0 to +1.0).
    pub sentiment: f32,
    /// Rough complexity estimate (0.0–1.0).
    pub complexity: f32,
}

/// Extract deterministic, symbolic features from raw text.
///
/// This function is intentionally simple but structured. It can be
/// extended without breaking existing callers.
pub fn extract_features(text: &str) -> IntentFeatures {
    let lower = text.to_lowercase();
    let tokens: Vec<&str> = lower.split_whitespace().collect();

    let mut verbs = Vec::new();
    let mut nouns = Vec::new();
    let mut has_url = false;
    let is_question = lower.trim_end().ends_with('?');

    for t in &tokens {
        if t.starts_with("http://") || t.starts_with("https://") {
            has_url = true;
        }

        // Verb-ish commands relevant to Syntra’s ecosystem.
        if [
            "open",
            "browse",
            "search",
            "plan",
            "build",
            "create",
            "diagnose",
            "fix",
            "run",
            "scan",
            "inspect",
            "evolve",
            "analyze",
        ]
        .contains(t)
        {
            verbs.push((*t).to_string());
            continue;
        }

        // Noun-ish entities relevant to Syntra’s lobes.
        if [
            "syntra",
            "kernel",
            "renderer",
            "cortex",
            "runtime",
            "task",
            "module",
            "file",
            "ecosystem",
            "safety",
            "sandbox",
            "thoughts",
            "terminal",
            "browser",
        ]
        .contains(t)
        {
            nouns.push((*t).to_string());
        }
    }

    let sentiment = estimate_sentiment(&tokens);
    let complexity = estimate_complexity(text, tokens.len());

    IntentFeatures {
        verbs,
        nouns,
        has_url,
        is_question,
        sentiment,
        complexity,
    }
}

/// Very simple sentiment heuristic.
///
/// Positive/negative words nudge the score; result is clamped to [-1.0, 1.0].
fn estimate_sentiment(tokens: &[&str]) -> f32 {
    let mut score = 0.0;

    for t in tokens {
        match *t {
            "love" | "great" | "good" | "awesome" | "nice" | "excellent" => score += 0.2,
            "hate" | "bad" | "terrible" | "awful" | "broken" | "annoying" => score -= 0.2,
            _ => {}
        }
    }

    score.clamp(-1.0, 1.0)
}

/// Rough complexity heuristic based on length and punctuation.
///
/// This is not a measure of difficulty, just structural richness.
fn estimate_complexity(text: &str, token_count: usize) -> f32 {
    let token_count = token_count as f32;
    let punctuation_count = text.matches(&['.', ',', ';', ':', '?', '!'][..]).count() as f32;

    let raw = (token_count / 20.0) + (punctuation_count / 10.0);
    raw.clamp(0.0, 1.0)
}
