// ================================================================================================
//   SYNTRA KERNEL — AXIOM TWO (STRUCTURED KNOWLEDGE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/schema/mod.rs
//   Module:      Schema Root
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Root module for structured schemas used across Syntra Kernel.
// ================================================================================================

pub mod intent_schema;
pub mod plan_schema;
pub mod execution_schema;

pub use intent_schema::Intent;
pub use plan_schema::RoutePlan;
pub use execution_schema::{ExecutionStep, ExecutionPlan};
