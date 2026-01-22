/* ================================================================================================
   SYNTRA BROWSER - AXIOM TWO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/memory_lobe.rs
   Module:      Cortex - Memory Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Stores past intents, responses, and system states. Enables reflection, planning,
                and long-term continuity across sessions.

   Notes:
     - In Axiom Two, memory is in-RAM only.
     - Axiom Three may introduce persistent memory.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{info, trace_enter, trace_exit};

#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub intent: String,
    pub classification: String,
    pub response: Option<String>,
}

pub struct MemoryLobe {
    pub entries: Vec<MemoryEntry>,
}

impl MemoryLobe {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn store_intent(&mut self, intent: &str, class: &str) {
        trace_enter("MemoryLobe::store_intent");
        self.entries.push(MemoryEntry {
            intent: intent.to_string(),
            classification: class.to_string(),
            response: None,
        });
        trace_exit("MemoryLobe::store_intent");
    }

    pub fn store_response(&mut self, response: &str) {
        trace_enter("MemoryLobe::store_response");
        if let Some(last) = self.entries.last_mut() {
            last.response = Some(response.to_string());
        }
        trace_exit("MemoryLobe::store_response");
    }

    pub fn recent(&self, count: usize) -> Vec<&MemoryEntry> {
        self.entries.iter().rev().take(count).collect()
    }
}
