// file: src/cognition/context.rs

use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
pub struct CognitiveRecord {
    pub timestamp: SystemTime,
    pub actor: String,
    pub event: String,
    pub duration: Option<Duration>,
    pub success: bool,
}

#[derive(Debug, Default)]
pub struct CognitiveContext {
    records: Vec<CognitiveRecord>,
}

impl CognitiveContext {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn record(&mut self, record: CognitiveRecord) {
        self.records.push(record);
    }

    pub fn recent(&self, count: usize) -> &[CognitiveRecord] {
        let len = self.records.len();
        let start = len.saturating_sub(count);
        &self.records[start..]
    }

    pub fn total_events(&self) -> usize {
        self.records.len()
    }
}
