/* ================================================================================================
   SYNTRAOS — ROBOTICS PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/robotics.rs
   Module:      SyntraOS Control Center — Robotics Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to robotics state:
         • Battery level
         • Joint temperatures
         • Joint load
         • IMU orientation
         • Gait stability
         • Sensor health
         • Robot mode (enabled/disabled)
         • Model identity

       This panel is UI-agnostic. It prepares robotics data for:
         • SyntraOS Shell (desktop)
         • Robot HUD
         • Browser UI
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct RoboticsPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> RoboticsPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    /// Returns whether robot mode is enabled.
    pub fn robot_mode_enabled(&self) -> bool {
        self.state.robotics.robot_mode_enabled
    }

    /// Returns the robot model name, if available.
    pub fn robot_model(&self) -> Option<String> {
        self.state.robotics.robot_model.clone()
    }

    /// Returns the maximum joint temperature.
    pub fn max_joint_temperature(&self) -> Option<f32> {
        self.state.robotics.joint_temperature_max_c
    }

    /// Returns the maximum joint load percentage.
    pub fn max_joint_load(&self) -> Option<f32> {
        self.state.robotics.joint_load_max_percent
    }

    /// Returns the gait stability score (0.0–1.0).
    pub fn gait_stability(&self) -> Option<f32> {
        self.state.robotics.gait_stability_score
    }

    /// Returns the IMU orientation string (e.g., "upright", "tilted", "falling").
    pub fn imu_orientation(&self) -> Option<String> {
        self.state.robotics.imu_orientation.clone()
    }

    /// Returns whether all sensors are healthy.
    pub fn sensor_health_ok(&self) -> bool {
        self.state.robotics.sensor_health_ok
    }
}
