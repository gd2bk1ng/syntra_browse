// ================================================================================================
//   SYNTRA KERNEL — SAFETY SUBSYSTEM (MODULE ROOT)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety/mod.rs
//   Module:      AGI Core — Safety & Governance (Subsystem Root)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Public entry point for the safety & governance layer. Re-exports safety policy,
//       safety gate, verdicts, and the runtime policy guard.
// ================================================================================================

pub mod safety_policy;
pub mod safety_gate;
pub mod safety_verdict;
pub mod policy_guard;

pub use safety_policy::{SafetyLevel, SafetyPolicy, SafetyRule};
pub use safety_gate::SafetyGate;
pub use safety_verdict::SafetyVerdict;
pub use policy_guard::PolicyGuard;
