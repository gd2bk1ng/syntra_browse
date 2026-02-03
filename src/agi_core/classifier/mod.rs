// ================================================================================================
//   SYNTRA KERNEL — AGI CORE / CLASSIFIER
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/classifier/mod.rs
//   Module:      Domain Classification Engine
//   Description: Semantic domain classifier for intents. Converts raw intent text into
//                high‑level cognitive domains used by the Reasoner, Planner, and Cortex lobes.
//
//   Notes:
//     - Backed by Axiom Five (Intent Engine).
//     - Designed for model‑backed classification in future versions.
//     - Fully extensible: add new domains, heuristics, or ML backends.
// ================================================================================================

use crate::agi_core::intent::Intent;

/// High-level semantic domain for an intent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainClass {
    General,
    Coding,
    Research,
    Creative,
    System,
    Unknown(String),
}

impl DomainClass {
    pub fn as_str(&self) -> &str {
        match self {
            DomainClass::General => "general",
            DomainClass::Coding => "coding",
            DomainClass::Research => "research",
            DomainClass::Creative => "creative",
            DomainClass::System => "system",
            DomainClass::Unknown(s) => s.as_str(),
        }
    }
}

/// Result of a domain classification.
#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub domain: DomainClass,
    pub confidence: f32,
}

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
            name: "SyntraDomainClassifier".to_string(),
            version: "0.1.0".to_string(),
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

    /// Classify an intent into a semantic domain.
    pub fn classify_intent(&self, intent: &Intent) -> ClassificationResult {
        // Placeholder for future ML model integration.
        let text = intent.text.to_lowercase();

        let (domain, prob) = if text.contains("code") || text.contains("rust") {
            (DomainClass::Coding, 0.92)
        } else if text.contains("research") || text.contains("explain") {
            (DomainClass::Research, 0.88)
        } else if text.contains("story") || text.contains("write") {
            (DomainClass::Creative, 0.85)
        } else if text.contains("system") || text.contains("kernel") {
            (DomainClass::System, 0.90)
        } else {
            (DomainClass::General, 0.55)
        };

        ClassificationResult {
            domain,
            confidence: prob,
        }
    }

    pub fn is_confident(&self, result: &ClassificationResult) -> bool {
        result.confidence >= self.threshold
    }
}
