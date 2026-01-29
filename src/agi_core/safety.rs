// ================================================================================================
//   SYNTRA KERNEL — AXIOM FIVE (SAFETY & GOVERNANCE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety.rs
//   Module:      Safety Subsystem (Public API)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Public façade for the modular safety subsystem. Re-exports the internal safety modules
//       under a clean, stable interface for the rest of the kernel.
// ================================================================================================

pub mod safety;

pub use safety::{
    safety_gate::SafetyGate,
    safety_policy::{SafetyLevel, SafetyPolicy, SafetyRule},
    safety_verdict::SafetyVerdict,
    policy_guard::PolicyGuard,
};
