// file: src/intent/pipeline_map.rs
// This is the binding layer.
// This is static resolution — and that’s exactly what we want.

use crate::intent::intent::Intent;

#[derive(Debug)]
pub enum Pipeline {
    RenderPipeline,
    InspectionPipeline,
    TrialPipeline,
}

pub fn resolve_pipeline(intent: &Intent) -> Pipeline {
    match intent {
        Intent::RenderPage => Pipeline::RenderPipeline,
        Intent::InspectSystem => Pipeline::InspectionPipeline,
        Intent::RunTrial => Pipeline::TrialPipeline,
    }
}
