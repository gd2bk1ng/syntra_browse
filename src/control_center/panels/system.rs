/* ================================================================================================
   SYNTRAOS — SYSTEM PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/system.rs
   Module:      SyntraOS Control Center — System Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to system metrics:
         • CPU, RAM, GPU
         • Storage
         • Battery / Power
         • Uptime
         • Host identity

       This module does not render UI. It prepares data for any frontend:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SystemPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SystemPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    /// Example: return CPU usage as a simple value.
    pub fn cpu_usage(&self) -> f32 {
        self.state.system.cpu_usage_percent
    }

    /// Example: return a formatted uptime string.
    pub fn uptime_string(&self) -> String {
        let secs = self.state.system.uptime.as_secs();
        format!("{}h {}m {}s", secs / 3600, (secs / 60) % 60, secs % 60)
    }
}
