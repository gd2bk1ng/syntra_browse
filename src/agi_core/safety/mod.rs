// ================================================================================================
//   SYNTRA KERNEL — AXIOM FIVE (SAFETY & GOVERNANCE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/safety/mod.rs
//   Module:      AGI Core — Safety & Governance (Subsystem Root)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Root module for the Syntra Kernel safety subsystem. This folder-based architecture merges
//       the original public façade (`safety.rs`) with the internal safety modules, providing a clean,
//       stable, and extensible interface for the rest of the kernel.
//
//       The safety subsystem governs:
//         • Safety policies (rules, levels, constraints)
//         • Safety gates (runtime enforcement)
//         • Verdicts (allow, deny, escalate)
//         • Policy guards (runtime wrappers for safe execution)
//
//   Architectural Role:
//       • Axiom Five — Safety & Governance
//       • Axiom Six  — Self-modification constraints
//       • Axiom Seven — Kernel integrity & protection
//
//   Notes:
//       - This module is intentionally minimal at the root level.
//       - All heavy logic lives in submodules.
//       - This replaces the old flat `safety.rs` file entirely.
// ================================================================================================

#![allow(dead_code)]

pub mod safety_policy;
pub mod safety_gate;
pub mod safety_verdict;
pub mod policy_guard;

// -------------------------------------------------------------------------------------------------
// Public API (merged from old safety.rs façade)
// -------------------------------------------------------------------------------------------------

pub use safety_policy::{SafetyLevel, SafetyPolicy, SafetyRule};
pub use safety_gate::SafetyGate;
pub use safety_verdict::SafetyVerdict;
pub use policy_guard::PolicyGuard;
