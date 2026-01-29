// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (INTENT LOG + INTROSPECTION UTILITIES)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/intent_log.rs
//   Module:      Intent Log (Ring Buffer + Debug Utilities)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Lightweight, deterministic ring buffer for storing recent IntentPlans. Used by
//       ThoughtStream, Cortex introspection, and Terminal commands. Includes ASCII-safe
//       JSON escaping utilities for overlays and debug output.
//
//   Architectural Role:
//       • Axiom Three — Cognitive Awareness (introspection).
//       • Axiom Four  — Routing visibility (debugging cognitive flow).
//       • Axiom Six   — Evolution (observing reasoning patterns).
//
//   Notes:
//       - No I/O, no global state.
//       - Purely in-memory, safe for all lobes.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::intent::IntentPlan;

// ================================================================================================
// Intent Log (Ring Buffer)
// ================================================================================================

/// Ring buffer for recent IntentPlans.
///
/// Used by ThoughtStream and introspection commands to show recent
/// reasoning activity without persisting everything indefinitely.
#[derive(Debug)]
pub struct IntentLog {
    buffer: Vec<IntentPlan>,
    capacity: usize,
}

impl IntentLog {
    /// Create a new IntentLog with a fixed capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Append a new plan to the log, evicting the oldest if full.
    pub fn push(&mut self, plan: IntentPlan) {
        if self.buffer.len() == self.capacity {
            self.buffer.remove(0);
        }
        self.buffer.push(plan);
    }

    /// Return a slice of all currently stored plans.
    pub fn all(&self) -> &[IntentPlan] {
        &self.buffer
    }

    /// Return the most recent N plans (or fewer if not available).
    pub fn recent(&self, n: usize) -> Vec<&IntentPlan> {
        let len = self.buffer.len();
        let start = len.saturating_sub(n);
        self.buffer[start..].iter().collect()
    }

    /// Return the latest plan, if any.
    pub fn latest(&self) -> Option<&IntentPlan> {
        self.buffer.last()
    }
}

// ================================================================================================
// JSON Escaping (ASCII-Safe)
// ================================================================================================

/// Escape a string for safe embedding inside JSON or debug overlays.
///
/// This is intentionally minimal and ASCII-safe.
pub fn escape_json(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}
