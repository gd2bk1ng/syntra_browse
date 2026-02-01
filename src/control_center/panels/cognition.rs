/* ================================================================================================
   SYNTRAOS — COGNITION PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/cognition.rs
   Module:      SyntraOS Control Center — Cognition Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to cognition state:
         • Active intents
         • Active tasks
         • Routing confidence
         • Safety block counts
         • Feedback events

       This panel is UI-agnostic. It prepares cognition data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct CognitionPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> CognitionPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    /// Returns the list of active cognitive intents.
    pub fn active_intents(&self) -> &[String] {
        &self.state.cognition.active_intents
    }

    /// Returns the list of active cognitive tasks.
    pub fn active_tasks(&self) -> &[String] {
        &self.state.cognition.active_tasks
    }

    /// Returns the average routing confidence.
    pub fn routing_confidence(&self) -> f32 {
        self.state.cognition.routing_confidence_avg
    }
}
