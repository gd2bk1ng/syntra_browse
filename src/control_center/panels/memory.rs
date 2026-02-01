/* ================================================================================================
   SYNTRAOS — MEMORY PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/memory.rs
   Module:      SyntraOS Control Center — Memory Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Memory subsystem:
         • Episodic memory entries
         • Semantic memory entries
         • Memory embeddings
         • Recall confidence
         • Memory clusters
         • Tags & associations
         • Memory search results
         • Memory health score

       This panel is UI-agnostic. It prepares memory data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct MemoryPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> MemoryPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Episodic Memory
    // --------------------------------------------------------------------------------------------

    /// Returns the list of episodic memory entries.
    pub fn episodic(&self) -> &[String] {
        &self.state.memory.episodic
    }

    /// Returns the last episodic memory entry.
    pub fn last_episode(&self) -> Option<String> {
        self.state.memory.last_episode.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Semantic Memory
    // --------------------------------------------------------------------------------------------

    /// Returns the list of semantic memory entries.
    pub fn semantic(&self) -> &[String] {
        &self.state.memory.semantic
    }

    /// Returns the last semantic memory entry.
    pub fn last_semantic(&self) -> Option<String> {
        self.state.memory.last_semantic.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Embeddings & Clusters
    // --------------------------------------------------------------------------------------------

    /// Returns the list of memory clusters.
    pub fn clusters(&self) -> &[String] {
        &self.state.memory.clusters
    }

    /// Returns the cluster associated with a specific memory ID.
    pub fn cluster_of(&self, id: &str) -> Option<String> {
        self.state.memory.cluster_of.get(id).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Tags & Associations
    // --------------------------------------------------------------------------------------------

    /// Returns the list of memory tags.
    pub fn tags(&self) -> &[String] {
        &self.state.memory.tags
    }

    /// Returns the tags associated with a specific memory ID.
    pub fn tags_of(&self, id: &str) -> Option<&Vec<String>> {
        self.state.memory.tags_of.get(id)
    }

    // --------------------------------------------------------------------------------------------
    //  Recall & Search
    // --------------------------------------------------------------------------------------------

    /// Returns the recall confidence score (0.0–1.0).
    pub fn recall_confidence(&self) -> f32 {
        self.state.memory.recall_confidence
    }

    /// Returns the results of the last memory search.
    pub fn search_results(&self) -> &[String] {
        &self.state.memory.search_results
    }

    // --------------------------------------------------------------------------------------------
    //  Health
    // --------------------------------------------------------------------------------------------

    /// Returns the memory subsystem health score (0.0–1.0).
    pub fn health_score(&self) -> f32 {
        self.state.memory.health_score
    }
}
