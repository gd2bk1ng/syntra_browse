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
       Read-only view over the cognitive loop:
         • Active cognitive context
         • Focus stack
         • Last observation / thought
         • Cognitive load
         • Loop phase (perceive, plan, act, reflect)
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct CognitionPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> CognitionPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    pub fn active_context(&self) -> Option<String> {
        self.state.cognition.active_context.clone()
    }

    pub fn focus_stack(&self) -> &[String] {
        &self.state.cognition.focus_stack
    }

    pub fn last_observation(&self) -> Option<String> {
        self.state.cognition.last_observation.clone()
    }

    pub fn last_thought(&self) -> Option<String> {
        self.state.cognition.last_thought.clone()
    }

    pub fn cognitive_load(&self) -> f32 {
        self.state.cognition.cognitive_load
    }

    pub fn loop_phase(&self) -> Option<String> {
        self.state.cognition.loop_phase.clone()
    }
}
