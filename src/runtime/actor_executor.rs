// file: src/runtime/actor_executor.rs

use crate::pipeline::executor::execute_pipeline;
use crate::intent::pipeline_map::Pipeline;
use crate::cognition::observation::ObservationEmitter;

pub struct PipelineActor {
    name: &'static str,
}

impl PipelineActor {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn execute(
        &self,
        pipeline: Pipeline,
        observer: &mut ObservationEmitter,
    ) -> bool {
        observer.emit(
            self.name,
            "actor_execution_started",
            None,
            true,
        );

        let result = execute_pipeline(pipeline);

        observer.emit(
            self.name,
            "actor_execution_finished",
            None,
            result,
        );

        result
    }
}
