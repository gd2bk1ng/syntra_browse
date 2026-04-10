// file: src/utilities/evolution_predictor.rs
// Author: Alexandr Roussinov (gd2bk1ng)

use serde::{Deserialize, Serialize};

use crate::utilities::{RiskAnalyzer, RiskAssessment};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionImpact {
    pub confidence: f32,
    pub expected_stability_delta: f32,
    pub expected_maintainability_delta: f32,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct EvolutionPredictor {
    risk: RiskAnalyzer,
}

impl EvolutionPredictor {
    pub fn new() -> Self {
        Self {
            risk: RiskAnalyzer::new(),
        }
    }

    pub fn forecast(&self, subsystem: &str, touch_count: usize) -> (EvolutionImpact, RiskAssessment) {
        let risk = self.risk.assess_change(subsystem, touch_count);

        let confidence = (1.0 - risk.score).clamp(0.1, 0.95);
        let stability = 0.2 - risk.score;
        let maintainability = 0.15 + ((touch_count as f32).ln_1p() * 0.05) - (risk.score * 0.2);

        let impact = EvolutionImpact {
            confidence,
            expected_stability_delta: stability,
            expected_maintainability_delta: maintainability,
            notes: vec![format!(
                "Forecast generated for subsystem '{subsystem}' with {touch_count} touch points"
            )],
        };

        (impact, risk)
    }
}

