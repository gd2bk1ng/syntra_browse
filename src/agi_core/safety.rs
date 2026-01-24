/* ================================================================================================
   SYNTRA BROWSER — AXIOM SEVEN
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/agi_core/safety.rs
   Module:      AGI Core — Safety & Governance Layer
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Defines safety policies and governance rules for Syntra. Evaluates self-modification
                proposals and enforces boundaries: non-lethal, non-destructive, human-approved.
                This layer never mutates code; it only governs what is allowed to be proposed or
                auto-applied.
   ================================================================================================ */

#![allow(dead_code)]

use crate::agi_core::{ChangeKind, ChangeProposal, EvolutionPlan};

/* ------------------------------------------------------------------------------------------------
   SAFETY LEVELS & RULES
   ------------------------------------------------------------------------------------------------ */

/// Coarse safety level for a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    /// Safe, non-destructive, reversible.
    Safe,
    /// Potentially risky, requires explicit human review.
    ReviewRequired,
    /// Not allowed under any circumstance.
    Forbidden,
}

/// A single safety rule.
#[derive(Debug, Clone)]
pub struct SafetyRule {
    /// Human-readable description of the rule.
    pub description: String,
    /// Optional path prefix that this rule applies to (e.g., "src/agi_core/safety").
    pub path_prefix: Option<String>,
    /// Whether this rule marks the target as non-modifiable.
    pub non_modifiable: bool,
}

/// A safety policy: collection of rules + global constraints.
#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    pub rules: Vec<SafetyRule>,
    /// Whether Syntra is allowed to auto-apply changes without human approval.
    pub allow_auto_apply: bool,
}

/// Verdict for a single proposal.
#[derive(Debug, Clone)]
pub struct SafetyVerdict {
    pub proposal: ChangeProposal,
    pub level: SafetyLevel,
    pub reason: String,
}

/// Safety gate: evaluates proposals against the policy.
#[derive(Debug, Clone)]
pub struct SafetyGate {
    pub policy: SafetyPolicy,
}

impl SafetyPolicy {
    /// Default policy:
    ///   - Core safety/governance modules are non-modifiable.
    ///   - Self-mod engine is non-modifiable.
    ///   - Auto-apply is disabled.
    pub fn default() -> Self {
        Self {
            rules: vec![
                SafetyRule {
                    description: "Core safety & governance layer is non-modifiable.".into(),
                    path_prefix: Some("src/agi_core/safety".into()),
                    non_modifiable: true,
                },
                SafetyRule {
                    description: "Self-modification engine is non-modifiable.".into(),
                    path_prefix: Some("src/agi_core/self_mod".into()),
                    non_modifiable: true,
                },
                SafetyRule {
                    description: "Global policy: never modify safety-critical core without human review."
                        .into(),
                    path_prefix: Some("src/agi_core".into()),
                    non_modifiable: false,
                },
            ],
            allow_auto_apply: false,
        }
    }
}

impl SafetyGate {
    pub fn new(policy: SafetyPolicy) -> Self {
        Self { policy }
    }

    /// Evaluate a single proposal and return a safety verdict.
    pub fn evaluate_proposal(&self, proposal: ChangeProposal) -> SafetyVerdict {
        // Default assumption: safe but requires human review for application.
        let mut level = SafetyLevel::ReviewRequired;
        let mut reason = "Default: changes require human review.".to_string();

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

        // Some kinds are always advisory only.
        match proposal.kind {
            ChangeKind::NewLobe
            | ChangeKind::Upgrade
            | ChangeKind::Refactor
            | ChangeKind::DeadCodeCleanup
            | ChangeKind::DependencyFix
            | ChangeKind::Evolution => {
                // Keep as ReviewRequired unless policy explicitly allows auto-apply.
                if self.policy.allow_auto_apply {
                    level = SafetyLevel::Safe;
                    reason = "Policy allows auto-apply for this kind of change.".into();
                }
            }
        }

        SafetyVerdict {
            proposal,
            level,
            reason,
        }
    }

    /// Evaluate an entire evolution plan and split into allowed + blocked.
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
                SafetyLevel::Safe | SafetyLevel::ReviewRequired => allowed.push(verdict),
            }
        }

        (allowed, blocked)
    }
}
