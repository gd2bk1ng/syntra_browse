 // file: src/utilities/risk_analyzer.rs
 // Author: Alexandr Roussinov (gd2bk1ng)
 
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub target: String,
    pub score: f32,
    pub level: RiskLevel,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RiskAnalyzer;

impl RiskAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn assess_change(&self, target: &str, touch_count: usize) -> RiskAssessment {
        let score = (touch_count as f32 / 10.0).clamp(0.0, 1.0);
        let level = if score < 0.25 {
            RiskLevel::Low
        } else if score < 0.5 {
            RiskLevel::Moderate
        } else if score < 0.8 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        let mut reasons = Vec::new();
        if touch_count > 5 {
            reasons.push("Large number of touched modules".to_string());
        }
        if target.contains("core") || target.contains("runtime") {
            reasons.push("Change touches mission-critical subsystem".to_string());
        }

        RiskAssessment {
            target: target.to_string(),
            score,
            level,
            reasons,
        }
    }
}
