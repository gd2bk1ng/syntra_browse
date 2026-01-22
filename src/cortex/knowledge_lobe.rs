/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/knowledge_lobe.rs
   Module:      Cortex - Knowledge Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Stores and retrieves knowledge extracted from perceptions and interactions.
                This is Syntra's long-term semantic memory in Axiom Three.

   Notes:
     - In Axiom Three, knowledge is in-RAM only.
     - Future axioms may introduce persistent stores and embeddings.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{info, trace_enter, trace_exit};
use crate::cortex::perception_lobe::Perception;

#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    pub source: String,
    pub perception: Perception,
}

pub struct KnowledgeLobe {
    entries: Vec<KnowledgeEntry>,
}

impl KnowledgeLobe {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn store(&mut self, source: &str, perception: Perception) {
        trace_enter("KnowledgeLobe::store");
        self.entries.push(KnowledgeEntry {
            source: source.to_string(),
            perception,
        });
        trace_exit("KnowledgeLobe::store");
    }

    /// Very simple search: match substring in title or summary.
    pub fn search(&self, query: &str) -> Vec<&KnowledgeEntry> {
        trace_enter("KnowledgeLobe::search");
        let q = query.to_lowercase();
        let mut results = Vec::new();

        for entry in &self.entries {
            let title = entry.perception.title.as_deref().unwrap_or("");
            if title.to_lowercase().contains(&q)
                || entry.perception.summary.to_lowercase().contains(&q)
            {
                results.push(entry);
            }
        }

        trace_exit("KnowledgeLobe::search");
        results
    }

    pub fn all(&self) -> &[KnowledgeEntry] {
        &self.entries
    }
}
