// file: src/pipeline/executor.rs
// This is a non-intelligent pipeline executor.

use crate::intent::pipeline_map::Pipeline;

pub fn execute_pipeline(pipeline: Pipeline) {
    match pipeline {
        Pipeline::RenderPipeline => {
            println!("🖼️ Executing Render Pipeline");
        }
        Pipeline::InspectionPipeline => {
            println!("🔍 Executing Inspection Pipeline");
        }
        Pipeline::TrialPipeline => {
            println!("🧪 Executing Trial Pipeline");
        }
    }
}
