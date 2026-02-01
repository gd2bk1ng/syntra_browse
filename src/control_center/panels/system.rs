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
       Read-only view over core system state:
         • Hostname, OS, kernel revision
         • Uptime
         • CPU / memory / disk usage
         • Power profile & mode
         • Thermal throttling
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SystemPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SystemPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    pub fn hostname(&self) -> Option<String> {
        self.state.system.hostname.clone()
    }

    pub fn os_version(&self) -> Option<String> {
        self.state.system.os_version.clone()
    }

    pub fn kernel_revision(&self) -> Option<String> {
        self.state.system.kernel_revision.clone()
    }

    pub fn uptime_seconds(&self) -> u64 {
        self.state.system.uptime_seconds
    }

    pub fn cpu_usage_percent(&self) -> f32 {
        self.state.system.cpu_usage_percent
    }

    pub fn memory_usage_percent(&self) -> f32 {
        self.state.system.memory_usage_percent
    }

    pub fn disk_usage_percent(&self) -> f32 {
        self.state.system.disk_usage_percent
    }

    pub fn thermal_throttling(&self) -> bool {
        self.state.system.thermal_throttling
    }

    pub fn power_profile(&self) -> Option<String> {
        self.state.system.power_profile.clone()
    }

    pub fn mode(&self) -> Option<String> {
        self.state.system.mode.clone()
    }
}
