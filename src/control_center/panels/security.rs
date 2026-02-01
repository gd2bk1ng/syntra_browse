/* ================================================================================================
   SYNTRAOS — SECURITY PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/security.rs
   Module:      SyntraOS Control Center — Security Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to security state:
         • System armed/disarmed
         • Sensors (motion, window, door)
         • Cameras
         • Intrusion alerts
         • Last triggered sensor
         • Security mode (Home, Away, Night)
         • Lock status
         • Threat level

       This panel is UI-agnostic. It prepares security data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SecurityPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SecurityPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  System Status
    // --------------------------------------------------------------------------------------------

    /// Returns whether the security system is armed.
    pub fn is_armed(&self) -> bool {
        self.state.security.armed
    }

    /// Returns the current security mode (e.g., "Home", "Away", "Night").
    pub fn mode(&self) -> Option<String> {
        self.state.security.mode.clone()
    }

    /// Returns the current threat level (0.0–1.0).
    pub fn threat_level(&self) -> f32 {
        self.state.security.threat_level
    }

    // --------------------------------------------------------------------------------------------
    //  Sensors
    // --------------------------------------------------------------------------------------------

    /// Returns a list of all security sensors.
    pub fn sensors(&self) -> &[String] {
        &self.state.security.sensors
    }

    /// Returns whether a specific sensor is triggered.
    pub fn sensor_triggered(&self, name: &str) -> Option<bool> {
        self.state.security.sensor_triggered.get(name).cloned()
    }

    /// Returns the last triggered sensor, if any.
    pub fn last_triggered_sensor(&self) -> Option<String> {
        self.state.security.last_triggered_sensor.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Cameras
    // --------------------------------------------------------------------------------------------

    /// Returns the list of security cameras.
    pub fn cameras(&self) -> &[String] {
        &self.state.security.cameras
    }

    /// Returns whether a camera is online.
    pub fn camera_online(&self, name: &str) -> Option<bool> {
        self.state.security.camera_online.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Doors & Locks
    // --------------------------------------------------------------------------------------------

    /// Returns a list of all monitored doors.
    pub fn doors(&self) -> &[String] {
        &self.state.security.doors
    }

    /// Returns whether a specific door is locked.
    pub fn is_door_locked(&self, door: &str) -> Option<bool> {
        self.state.security.door_locked.get(door).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Alerts
    // --------------------------------------------------------------------------------------------

    /// Returns a list of active security alerts.
    pub fn alerts(&self) -> &[String] {
        &self.state.security.alerts
    }

    /// Returns whether an intrusion alert is active.
    pub fn intrusion_detected(&self) -> bool {
        self.state.security.intrusion_detected
    }
}
