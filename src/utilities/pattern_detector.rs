// file: src/utilities/pattern_detector.rs
// Author: Alexandr Roussinov (gd2bk1ng)
 
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatternSeverity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatternKind {
    TodoMarker,
    LargeFunction,
    DeepNesting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetectedPattern {
    pub file: String,
    pub line: usize,
    pub kind: PatternKind,
    pub severity: PatternSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct PatternDetector;

impl PatternDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_todo_markers(&self, file: &str, content: &str) -> Vec<DetectedPattern> {
        content
            .lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                if line.contains("TODO") {
                    Some(DetectedPattern {
                        file: file.to_string(),
                        line: idx + 1,
                        kind: PatternKind::TodoMarker,
                        severity: PatternSeverity::Medium,
                        message: "TODO marker indicates incomplete logic".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect()
  }
}
  
