// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT EMBEDDING)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/embedding.rs
//   Module:      Intent Embedding (Symbolic, Deterministic)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Symbolic, interpretable embeddings for intents. Converts high-level semantic classes
//       and extracted features into a compact vector representation suitable for similarity,
//       clustering, and lightweight routing.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (semantic positioning of intents).
//       • Axiom Four  — Routing between perception / cognition / action lobes.
//
//   Notes:
//       - This is *not* a neural embedding.
//       - All dimensions are documented and stable across versions.
// ================================================================================================

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::agi_core::features::IntentFeatures;

/// Symbolic embedding for similarity and clustering.
///
/// This is a compact, interpretable vector derived from features and
/// domain classification. It is stable and versioned by convention.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IntentEmbedding {
    /// Domain-oriented vector (heuristic dimensions).
    ///
    /// Indices:
    ///   0: browse / web
    ///   1: knowledge / search
    ///   2: task / execution
    ///   3: self / introspection
    ///   4: evolution / sandbox
    ///   5: safety / policy
    ///   6: system / ecosystem
    ///   7: freeform / chat / creative
    pub domain_vector: [f32; 8],

    /// Sentiment component (-1.0 to +1.0).
    pub sentiment: f32,

    /// Complexity component (0.0–1.0).
    pub complexity: f32,
}

/// Construct a symbolic embedding from class + features.
///
/// This is a compact, interpretable representation used for similarity
/// and clustering. It is not a neural embedding.
pub fn build_embedding(class: &str, features: &IntentFeatures) -> IntentEmbedding {
    let mut domain_vector = [0.0_f32; 8];

    // Map high-level classes to domain axes.
    match class {
        "browse" => domain_vector[0] = 1.0,
        "knowledge" => domain_vector[1] = 1.0,
        "task" => domain_vector[2] = 1.0,
        "self" | "introspection" => domain_vector[3] = 1.0,
        "evolve" | "sandbox" => domain_vector[4] = 1.0,
        "safety" => domain_vector[5] = 1.0,
        "ecosystem" => domain_vector[6] = 1.0,
        "freeform" | _ => domain_vector[7] = 1.0,
    }

    // Slight nudge for questions in knowledge/browse space.
    if features.is_question && (class == "knowledge" || class == "browse") {
        domain_vector[1] += 0.1;
    }

    IntentEmbedding {
        domain_vector,
        sentiment: features.sentiment,
        complexity: features.complexity,
    }
}
