// file: src/utilities/refactor_engine.rs
// Author: Alexandr Roussinov (gd2bk1ng)

use serde::{Deserialize, Serialize};

/// Refactor operation category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RefactorKind {
    Rename,
    Extract,
    Inline,
    Move,
    Cleanup,
}

/// Single unit of refactor work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorOperation {
    pub kind: RefactorKind,
    pub target_file: String,
    pub target_symbol: Option<String>,
    pub description: String,
}

/// Collection of operations planned for one refactor iteration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefactorPlan {
    pub title: String,
    pub rationale: String,
    pub operations: Vec<RefactorOperation>,
}

#[derive(Debug, Clone, Default)]
pub struct RefactorEngine;

impl RefactorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn plan_cleanup(
        &self,
        target_file: impl Into<String>,
        rationale: impl Into<String>,
    ) -> RefactorPlan {
        RefactorPlan {
            title: "Cleanup pass".to_string(),
            rationale: rationale.into(),
            operations: vec![RefactorOperation {
                kind: RefactorKind::Cleanup,
                target_file: target_file.into(),
                target_symbol: None,
                description: "Normalize imports/comments and reduce noise".to_string(),
            }],
        }
    }

    pub fn operation_count(&self, plan: &RefactorPlan) -> usize {
        plan.operations.len()
    }
}
