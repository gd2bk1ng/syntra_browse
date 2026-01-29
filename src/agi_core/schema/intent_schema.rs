// ================================================================================================
//   SYNTRA KERNEL — AXIOM TWO (INTENT SCHEMA)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/schema/intent_schema.rs
//   Module:      Intent Schema
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Defines the structure of an intent flowing through the kernel.
// ================================================================================================

#![allow(dead_code)]

/// Core intent structure.
#[derive(Debug, Clone)]
pub struct Intent {
    pub kind: String,
    pub content: String,
    pub metadata: Option<String>,
}

impl Intent {
    pub fn new(kind: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            content: content.into(),
            metadata: None,
        }
    }
}
