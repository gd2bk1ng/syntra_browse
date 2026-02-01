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
         • Safety blocks
         • Feedback events
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct CognitionPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> CognitionPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }
}
