/* ================================================================================================
   SYNTRA KERNEL — AGI CORE (SAFETY & GOVERNANCE LAYER)
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/agi_core/safety.rs
   Module:      AGI Core — Safety & Governance Layer
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Defines safety policies and governance rules for Syntra. Evaluates self-modification
       proposals and enforces boundaries: non-lethal, non-destructive, human-approved.
       This layer never mutates code; it only governs what is allowed to be proposed or
       auto-applied.

   Architectural Role:
       • Axiom Five  — Safety & Governance.
       • Axiom Six   — Evolution (constrained self-modification).
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
///
/// Rules are evaluated in order of appearance; the first matching
/// non-modifiable rule can immediately forbid a proposal.
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
    /// Ordered list of rules.
    pub rules: Vec<SafetyRule>,
    /// Whether Syntra is allowed to auto-apply changes without human approval.
    pub allow_auto_apply: bool,
}

/// Verdict for a single proposal.
#[derive(Debug, Clone)]
pub struct SafetyVerdict {
    /// The original proposal being evaluated.
    pub proposal: ChangeProposal,
    /// Safety level assigned by the policy.
    pub level: SafetyLevel,
    /// Human-readable explanation for the verdict.
    pub reason: String,
}

/// Safety gate: evaluates proposals against the policy.
///
/// This is the main entry point for the evolution engine and self-mod
/// subsystems when deciding whether a proposed change is permissible.
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
    /// Construct a new SafetyGate from a policy.
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

        // Some kinds are always advisory only; they can be auto-applied
        // if and only if the policy explicitly allows it.
        match proposal.kind {
            ChangeKind::NewLobe
            | ChangeKind::Upgrade
            | ChangeKind::Refactor
            | ChangeKind::DeadCodeCleanup
            | ChangeKind::DependencyFix
            | ChangeKind::Evolution => {
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
    ///
    /// `allowed` includes both `Safe` and `ReviewRequired` proposals.
    /// `blocked` includes all `Forbidden` proposals.
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
