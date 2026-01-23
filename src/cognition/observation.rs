// This bridges runtime → cognition.
// file: src/cognition/observation.rs

use std::time::{SystemTime, Duration};

use super::context::{CognitiveContext, CognitiveRecord};

pub struct ObservationEmitter<'a> {
    context: &'a mut CognitiveContext,
}

impl<'a> ObservationEmitter<'a> {
    pub fn new(context: &'a mut CognitiveContext) -> Self {
        Self { context }
    }

    pub fn emit(
        &mut self,
        actor: &str,
        event: &str,
        duration: Option<Duration>,
        success: bool,
    ) {
        let record = CognitiveRecord {
            timestamp: SystemTime::now(),
            actor: actor.to_string(),
            event: event.to_string(),
            duration,
            success,
        };

        self.context.record(record);
    }
}
