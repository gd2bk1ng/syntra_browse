/* ================================================================================================
   SYNTRAOS — SMART HOME PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/smart_home.rs
   Module:      SyntraOS Control Center — Smart Home Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to smart home state:
         • Scenes
         • Lights
         • Doors
         • Cameras
         • Energy usage
         • Alerts

       UI-agnostic. Prepares data for any frontend.
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SmartHomePanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SmartHomePanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    pub fn active_scene(&self) -> Option<String> {
        self.state.smart_home.active_scene.clone()
    }
}
