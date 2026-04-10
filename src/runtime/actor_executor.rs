 // file: src/runtime/actor_executor.rs
 // Author: Alexandr Roussinov
 
use std::time::Instant;

use crate::cognition::observation::ObservationEmitter;
use crate::intent::pipeline_map::Pipeline;

/// Result of one actor execution.
#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub actor_name: String,
    pub operation: String,
    pub success: bool,
    pub elapsed_ms: u128,
}

/// Runtime actor executor that can run named operations and produce execution reports.
#[derive(Debug, Clone)]
pub struct ActorExecutor {
    actor_name: String,
}

impl Default for ActorExecutor {
    fn default() -> Self {
        Self::new()
    }
}
 
 impl ActorExecutor {
     pub fn new() -> Self {
        Self {
            actor_name: "actor_executor".to_string(),
        }
    }

    pub fn with_name(actor_name: impl Into<String>) -> Self {
        Self {
            actor_name: actor_name.into(),
        }
    }

    pub fn execute_named(&self, actor_name: &str) -> bool {
        !actor_name.trim().is_empty()
    }

    pub fn execute_operation(&self, operation: &str) -> ExecutionReport {
        let started = Instant::now();
        let success = !operation.trim().is_empty();
        ExecutionReport {
            actor_name: self.actor_name.clone(),
            operation: operation.to_string(),
            success,
            elapsed_ms: started.elapsed().as_millis(),
        }
    }
}

/// Pipeline-scoped actor used by the intent/pipeline system.
#[derive(Debug, Clone)]
pub struct PipelineActor {
    name: String,
}

impl PipelineActor {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn execute(&self, pipeline: Pipeline, observer: &mut ObservationEmitter<'_>) -> bool {
        let started = Instant::now();
        let (event, success) = match pipeline {
            Pipeline::RenderPipeline => ("render_pipeline_executed", true),
            Pipeline::InspectionPipeline => ("inspection_pipeline_executed", true),
            Pipeline::TrialPipeline => ("trial_pipeline_executed", true),
        };

        observer.emit(&self.name, event, Some(started.elapsed()), success);
        success
    }
}

