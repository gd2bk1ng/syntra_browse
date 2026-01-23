/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/pipeline/executor.rs
   Module:      Pipeline Executor
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Non-intelligent pipeline executor and actor-driven pipeline execution.
                Currently, no actors or feedback are implemented in the simple executor.
                The actor-based executor uses PipelineActor and ObservationEmitter
                for enhanced processing and observation.

   Notes:
     - Executes predefined pipelines with simple logging or via actor execution.
     - Returns success status as boolean.
   ================================================================================================ */

use crate::intent::pipeline_map::Pipeline;
use crate::cognition::observation::ObservationEmitter;
use crate::runtime::actor_executor::PipelineActor;

/// Executes the given pipeline and returns success status.
///
/// # Parameters
///
/// - `pipeline`: The pipeline variant to execute.
///
/// # Returns
///
/// - `true` if the pipeline executed successfully.
///
/// # Examples
///
/// ```
/// use crate::intent::pipeline_map::Pipeline;
/// use crate::pipeline::executor::execute_pipeline;
///
/// let success = execute_pipeline(Pipeline::RenderPipeline);
/// assert!(success);
/// ```
pub fn execute_pipeline(pipeline: Pipeline) -> bool {
    match pipeline {
        Pipeline::RenderPipeline => {
            println!("🖼️ Executing Render Pipeline");
            true
        }
        Pipeline::InspectionPipeline => {
            println!("🔍 Executing Inspection Pipeline");
            true
        }
        Pipeline::TrialPipeline => {
            println!("🧪 Executing Trial Pipeline");
            true
        }
    }
}

/// Executes the given pipeline using a PipelineActor and emits observations.
///
/// # Parameters
///
/// - `pipeline`: Reference to the pipeline variant to execute.
/// - `observer`: Mutable reference to an ObservationEmitter for emitting observations.
///
/// # Returns
///
/// - `true` if the actor executed the pipeline successfully.
///
/// # Examples
///
/// ```
/// use crate::intent::pipeline_map::Pipeline;
/// use crate::cognition::observation::ObservationEmitter;
/// use crate::pipeline::executor::execute_pipeline_with_actor;
///
/// let mut observer = ObservationEmitter::new();
/// let success = execute_pipeline_with_actor(&Pipeline::RenderPipeline, &mut observer);
/// assert!(success);
/// ```
pub fn execute_pipeline_with_actor(
    pipeline: &Pipeline,
    observer: &mut ObservationEmitter,
) -> bool {
    let actor = PipelineActor::new("pipeline_actor");

    actor.execute(pipeline.clone(), observer)
}
