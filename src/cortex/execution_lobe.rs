/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/execution_lobe.rs
   Module:      Cortex - Execution Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Executes multi-step tasks, coordinating perception, action, and knowledge.
                This is Syntra's "motor cortex" for structured workflows.

   Notes:
     - Axiom Three keeps tasks simple and linear.
     - Future axioms may introduce branching, retries, and richer workflows.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{info, warn, trace_enter, trace_exit};
use crate::cortex::{PerceptionLobe, KnowledgeLobe};
use crate::cortex::action_lobe::ActionLobe;

/// A simple task step.
#[derive(Debug, Clone)]
pub enum TaskStep {
    FetchAndPerceive { url: String },
    RememberLast,
    NoOp,
}

/// A simple linear task.
#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub steps: Vec<TaskStep>,
}

pub struct ExecutionLobe;

impl ExecutionLobe {
    pub fn run(task: &Task, knowledge: &mut KnowledgeLobe) -> String {
        trace_enter("ExecutionLobe::run");
        info(&format!("Executing task: {}", task.name));

        let mut last_perception = None;
        let mut log = String::new();

        for (idx, step) in task.steps.iter().enumerate() {
            match step {
                TaskStep::FetchAndPerceive { url } => {
                    info(&format!("Step {}: FetchAndPerceive {}", idx + 1, url));
                    let result = ActionLobe::fetch_url(url);
                    if result.success {
                        let perception = PerceptionLobe::perceive(&result.output);
                        knowledge.store(url, perception.clone());
                        last_perception = Some(perception);
                        log.push_str(&format!("Fetched and perceived: {}\n", url));
                    } else {
                        warn(&format!("Failed to fetch URL: {}", url));
                        log.push_str(&format!("Failed to fetch: {}\n", url));
                    }
                }
                TaskStep::RememberLast => {
                    log.push_str("RememberLast: no-op in this simple implementation.\n");
                }
                TaskStep::NoOp => {
                    log.push_str("NoOp step.\n");
                }
            }
        }

        trace_exit("ExecutionLobe::run");
        log
    }
}
