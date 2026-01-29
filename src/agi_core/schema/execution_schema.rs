// ================================================================================================
//   SYNTRA KERNEL — AXIOM TWO (EXECUTION SCHEMA)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/schema/execution_schema.rs
//   Module:      Execution Schema
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Defines execution steps and execution plans for routed intents.
// ================================================================================================

#![allow(dead_code)]

/// A single execution step.
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub lobe: String,
    pub action: String,
    pub notes: Option<String>,
}

/// A full execution plan.
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub steps: Vec<ExecutionStep>,
}

impl ExecutionPlan {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(&mut self, step: ExecutionStep) {
        self.steps.push(step);
    }
}
