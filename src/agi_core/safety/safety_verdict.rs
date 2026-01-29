// ================================================================================================
//   SYNTRA KERNEL — AXIOM FIVE (SAFETY VERDICT)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety/safety_verdict.rs
//   Module:      Safety Verdicts
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Defines the verdict structure returned by the safety gate when evaluating proposals.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::self_mod::ChangeProposal;
use super::safety_policy::SafetyLevel;

/// Verdict for a single proposal.
#[derive(Debug, Clone)]
pub struct SafetyVerdict {
    pub proposal: ChangeProposal,
    pub level: SafetyLevel,
    pub reason: String,
}

impl SafetyVerdict {
    pub fn is_safe(&self) -> bool {
        matches!(self.level, SafetyLevel::Safe)
    }

    pub fn requires_review(&self) -> bool {
        matches!(self.level, SafetyLevel::ReviewRequired)
    }

    pub fn is_forbidden(&self) -> bool {
        matches!(self.level, SafetyLevel::Forbidden)
    }
}
