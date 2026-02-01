/* ================================================================================================
   SYNTRAOS — WORLD MODEL PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/world_model.rs
   Module:      SyntraOS Control Center — World Model Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS World Model:
         • Spatial awareness
         • Semantic scene graph
         • Object memory
         • Environmental predictions
         • Uncertainty metrics
         • Temporal deltas
         • Contextual embeddings

       This panel is UI-agnostic. It prepares world model data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Simulation environments
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct WorldModelPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> WorldModelPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Spatial Awareness
    // --------------------------------------------------------------------------------------------

    /// Returns the current spatial map representation (if available).
    pub fn spatial_map(&self) -> Option<String> {
        self.state.world_model.spatial_map.clone()
    }

    /// Returns the list of known locations or zones.
    pub fn known_locations(&self) -> &[String] {
        &self.state.world_model.known_locations
    }

    // --------------------------------------------------------------------------------------------
    //  Semantic Scene Graph
    // --------------------------------------------------------------------------------------------

    /// Returns the semantic scene graph (objects + relationships).
    pub fn scene_graph(&self) -> Option<String> {
        self.state.world_model.scene_graph.clone()
    }

    /// Returns the list of recognized objects.
    pub fn recognized_objects(&self) -> &[String] {
        &self.state.world_model.recognized_objects
    }

    // --------------------------------------------------------------------------------------------
    //  Object Memory
    // --------------------------------------------------------------------------------------------

    /// Returns the list of persistent objects tracked over time.
    pub fn persistent_objects(&self) -> &[String] {
        &self.state.world_model.persistent_objects
    }

    /// Returns the last observed location of a specific object.
    pub fn object_last_seen(&self, name: &str) -> Option<String> {
        self.state.world_model.object_last_seen.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Predictions
    // --------------------------------------------------------------------------------------------

    /// Returns environmental predictions (e.g., movement, changes).
    pub fn predictions(&self) -> &[String] {
        &self.state.world_model.predictions
    }

    /// Returns the prediction confidence score (0.0–1.0).
    pub fn prediction_confidence(&self) -> f32 {
        self.state.world_model.prediction_confidence
    }

    // --------------------------------------------------------------------------------------------
    //  Uncertainty
    // --------------------------------------------------------------------------------------------

    /// Returns the world model uncertainty score (0.0–1.0).
    pub fn uncertainty(&self) -> f32 {
        self.state.world_model.uncertainty
    }

    // --------------------------------------------------------------------------------------------
    //  Temporal Deltas
    // --------------------------------------------------------------------------------------------

    /// Returns a list of world-state deltas (changes over time).
    pub fn deltas(&self) -> &[String] {
        &self.state.world_model.deltas
    }

    /// Returns the timestamp of the last world model update.
    pub fn last_update(&self) -> Option<String> {
        self.state.world_model.last_update.clone()
    }
}
