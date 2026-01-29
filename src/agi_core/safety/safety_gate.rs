// ================================================================================================
//   SYNTRA KERNEL — AXIOM FIVE (SAFETY GATE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety/safety_gate.rs
//   Module:      Safety Gate (Proposal Evaluation)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Evaluates self-modification proposals and evolution plans against the active safety policy.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::self_mod::{ChangeKind, ChangeProposal, EvolutionPlan};
use super::safety_policy::{SafetyLevel, SafetyPolicy};
use super::safety_verdict::SafetyVerdict;

/// Evaluates proposals against the safety policy.
#[derive(Debug, Clone)]
pub struct SafetyGate {
    pub policy: SafetyPolicy,
}

impl SafetyGate {
    pub fn new(policy: SafetyPolicy) -> Self {
        Self { policy }
    }

    pub fn evaluate_proposal(&self, proposal: ChangeProposal) -> SafetyVerdict {
        let mut level = SafetyLevel::ReviewRequired;
        let mut reason = "Default: requires human review.".to_string();

        if let Some(target) = &proposal.target {
            for rule in &self.policy.rules {
                if let Some(prefix) = &rule.path_prefix {
                    if target.starts_with(prefix) {
                        if rule.non_modifiable {
                            return SafetyVerdict {
                                proposal,
                                level: SafetyLevel::Forbidden,
                                reason: format!(
                                    "Target '{}' is protected by rule: {}",
                                    target, rule.description
                                ),
                            };
                        } else {
                            level = SafetyLevel::ReviewRequired;
                            reason = format!(
                                "Target '{}' is governed by rule: {}",
                                target, rule.description
                            );
                        }
                    }
                }
            }
        }

        match proposal.kind {
            ChangeKind::NewLobe
            | ChangeKind::Upgrade
            | ChangeKind::Refactor
            | ChangeKind::DeadCodeCleanup
            | ChangeKind::DependencyFix
            | ChangeKind::Evolution => {
                if self.policy.allow_auto_apply {
                    level = SafetyLevel::Safe;
                    reason = "Policy allows auto-apply for this change.".into();
                }
            }
        }

        SafetyVerdict {
            proposal,
            level,
            reason,
        }
    }

    pub fn evaluate_evolution_plan(
        &self,
        plan: EvolutionPlan,
    ) -> (Vec<SafetyVerdict>, Vec<SafetyVerdict>) {
        let mut allowed = Vec::new();
        let mut blocked = Vec::new();

        for p in plan.proposals {
            let verdict = self.evaluate_proposal(p);
            match verdict.level {
                SafetyLevel::Forbidden => blocked.push(verdict),
                _ => allowed.push(verdict),
            }
        }

        (allowed, blocked)
    }
}
