// ================================================================================================
//   SYNTRA KERNEL — AXIOM FIVE (SAFETY POLICY)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety/safety_policy.rs
//   Module:      Safety Policy & Rules
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Defines safety levels, rules, and policies that govern self-modification and evolution.
// ================================================================================================

#![allow(dead_code)]

/// Coarse safety level for a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Safe,
    ReviewRequired,
    Forbidden,
}

/// A single safety rule.
#[derive(Debug, Clone)]
pub struct SafetyRule {
    pub description: String,
    pub path_prefix: Option<String>,
    pub non_modifiable: bool,
}

/// A safety policy: collection of rules + global constraints.
#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    pub rules: Vec<SafetyRule>,
    pub allow_auto_apply: bool,
}

impl SafetyPolicy {
    /// Default safety policy for Syntra Kernel.
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
                    description: "Global policy: never modify safety-critical core without review."
                        .into(),
                    path_prefix: Some("src/agi_core".into()),
                    non_modifiable: false,
                },
            ],
            allow_auto_apply: false,
        }
    }
}
