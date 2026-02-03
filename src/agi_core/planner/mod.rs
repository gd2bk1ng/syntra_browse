// ================================================================================================
//   SYNTRA KERNEL — AGI CORE / PLANNER
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/planner/mod.rs
//   Module:      Planning Engine
//   Description: Multi-step and narrative planning engine. Converts intents into structured,
//                executable plans used by the Cortex lobes and Runtime.
//
//   Notes:
//     - Backed by Axiom Three (Reasoning).
//     - Produces DAG-style plan graphs.
//     - Fully extensible: add new planning strategies or integrate LLM-based planners.
// ================================================================================================

use crate::agi_core::intent::{Intent, IntentPlan};

/// A single step in a plan.
#[derive(Debug, Clone)]
pub struct PlanNode {
    pub id: usize,
    pub description: String,
    pub depends_on: Vec<usize>,
}

impl PlanNode {
    pub fn new(id: usize, description: impl Into<String>) -> Self {
        Self {
            id,
            description: description.into(),
            depends_on: Vec::new(),
        }
    }

    pub fn with_dependency(mut self, dep: usize) -> Self {
        self.depends_on.push(dep);
        self
    }
}

/// A graph of plan nodes (DAG).
#[derive(Debug, Clone, Default)]
pub struct PlanGraph {
    pub nodes: Vec<PlanNode>,
}

impl PlanGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: PlanNode) {
        self.nodes.push(node);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// High-level planning engine.
#[derive(Debug, Default)]
pub struct PlannerEngine;

impl PlannerEngine {
    pub fn new() -> Self {
        Self
    }

    /// Multi-step planning wrapper.
    pub fn plan_multi_step(&self, intent: &Intent) -> IntentPlan {
        IntentPlan::from_steps(vec![
            format!("Analyze intent: {}", intent.text),
            "Determine required actions".into(),
            "Generate execution sequence".into(),
        ])
    }

    /// Narrative-style planning wrapper.
    pub fn plan_narrative(&self, intent: &Intent) -> IntentPlan {
        IntentPlan::from_steps(vec![
            format!("Understanding the user's goal: {}", intent.text),
            "Building a coherent narrative".into(),
            "Producing a structured plan".into(),
        ])
    }
}

/// Pretty-print a plan.
pub fn debug_plan(plan: &IntentPlan) {
    println!("[PLAN DEBUG] {:#?}", plan);
}
