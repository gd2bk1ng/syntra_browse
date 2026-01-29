// ================================================================================================
//   SYNTRA KERNEL — AXIOM FIVE (POLICY GUARD)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety/policy_guard.rs
//   Module:      Runtime Policy Guard
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Runtime enforcement layer that wraps the self-mod engine and safety gate. Ensures that
//       only proposals passing safety evaluation are surfaced for human review.
// ================================================================================================

#![allow(dead_code)]

use std::sync::Arc;

use crate::agi_core::self_mod::{EvolutionPlan, SelfModEngine};
use crate::agi_core::ecosystem::EcosystemModel;

use super::safety_gate::SafetyGate;
use super::safety_verdict::SafetyVerdict;

/// Orchestrates self-mod + safety gate.
#[derive(Debug, Clone)]
pub struct PolicyGuard {
    self_mod_engine: Arc<SelfModEngine>,
    safety_gate: Arc<SafetyGate>,
}

impl PolicyGuard {
    pub fn new(self_mod_engine: Arc<SelfModEngine>, safety_gate: Arc<SafetyGate>) -> Self {
        Self {
            self_mod_engine,
            safety_gate,
        }
    }

    pub fn generate_and_evaluate_plan(
        &self,
        root: impl AsRef<std::path::Path>,
        ecosystem: &EcosystemModel,
    ) -> (EvolutionPlan, Vec<SafetyVerdict>, Vec<SafetyVerdict>) {
        let plan = self.self_mod_engine.analyze_ecosystem(root, ecosystem);
        let (allowed, blocked) = self.safety_gate.evaluate_evolution_plan(plan.clone());
        (plan, allowed, blocked)
    }
}
