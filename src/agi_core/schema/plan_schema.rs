// ================================================================================================
//   SYNTRA KERNEL — AXIOM TWO (ROUTE PLAN SCHEMA)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/schema/plan_schema.rs
//   Module:      Route Plan Schema
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Defines the structure of a routing plan produced by the Router.
// ================================================================================================

#![allow(dead_code)]

use super::intent_schema::Intent;

/// Routing plan for a single intent.
#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub intent: Intent,
    pub target_lobe: String,
    pub confidence: f32,
    pub reason: String,
}
