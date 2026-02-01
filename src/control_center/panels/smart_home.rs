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
         • Active scene
         • Available scenes
         • Lights (per room)
         • Doors (locked/unlocked)
         • Thermostats
         • Cameras
         • Energy usage
         • Alerts

       This panel is UI-agnostic. It prepares smart home data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SmartHomePanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SmartHomePanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Scenes
    // --------------------------------------------------------------------------------------------

    /// Returns the currently active smart home scene.
    pub fn active_scene(&self) -> Option<String> {
        self.state.smart_home.active_scene.clone()
    }

    /// Returns the list of available scenes.
    pub fn available_scenes(&self) -> &[String] {
        &self.state.smart_home.available_scenes
    }

    // --------------------------------------------------------------------------------------------
    //  Lights
    // --------------------------------------------------------------------------------------------

    /// Returns a list of rooms that contain smart lights.
    pub fn light_rooms(&self) -> Vec<String> {
        self.state
            .smart_home
            .lights
            .keys()
            .cloned()
            .collect()
    }

    /// Returns the lights in a given room (if any).
    pub fn lights_in_room(&self, room: &str) -> Option<&Vec<String>> {
        self.state.smart_home.lights.get(room)
    }

    // --------------------------------------------------------------------------------------------
    //  Doors
    // --------------------------------------------------------------------------------------------

    /// Returns a list of all doors.
    pub fn doors(&self) -> &[String] {
        &self.state.smart_home.doors
    }

    /// Returns whether a specific door is locked.
    pub fn is_door_locked(&self, door: &str) -> Option<bool> {
        self.state.smart_home.door_locked.get(door).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Thermostats
    // --------------------------------------------------------------------------------------------

    /// Returns the current thermostat temperature (if available).
    pub fn thermostat_temp(&self) -> Option<f32> {
        self.state.smart_home.thermostat_temp_c
    }

    /// Returns the target thermostat temperature (if available).
    pub fn thermostat_target(&self) -> Option<f32> {
        self.state.smart_home.thermostat_target_c
    }

    // --------------------------------------------------------------------------------------------
    //  Cameras
    // --------------------------------------------------------------------------------------------

    /// Returns the list of camera names.
    pub fn cameras(&self) -> &[String] {
        &self.state.smart_home.cameras
    }

    /// Returns whether a camera is online.
    pub fn camera_online(&self, name: &str) -> Option<bool> {
        self.state.smart_home.camera_online.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Energy
    // --------------------------------------------------------------------------------------------

    /// Returns current energy usage in watts.
    pub fn energy_usage_watts(&self) -> f32 {
        self.state.smart_home.energy_usage_watts
    }

    /// Returns daily energy usage in kWh.
    pub fn energy_usage_kwh(&self) -> f32 {
        self.state.smart_home.energy_usage_kwh
    }

    // --------------------------------------------------------------------------------------------
    //  Alerts
    // --------------------------------------------------------------------------------------------

    /// Returns a list of smart home alerts.
    pub fn alerts(&self) -> &[String] {
        &self.state.smart_home.alerts
    }
}
