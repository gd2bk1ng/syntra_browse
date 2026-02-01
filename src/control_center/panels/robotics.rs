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
       Provides structured access to advanced robotics state:
         • Identity & behavior mode
         • Power & battery
         • Joints, actuators, and kinematics
         • Locomotion & gait
         • Localization & navigation
         • Perception & environment interaction
         • Manipulation & end-effectors
         • Safety, limits, and faults
         • Telemetry & health

       This panel is UI-agnostic. It prepares robotics data for:
         • SyntraOS Shell (desktop)
         • Robot HUD
         • Browser UI
         • AR overlays
         • Simulation dashboards
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;
use std::collections::HashMap;

pub struct RoboticsPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> RoboticsPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Identity & Mode
    // --------------------------------------------------------------------------------------------

    /// Returns whether robot mode is enabled.
    pub fn robot_mode_enabled(&self) -> bool {
        self.state.robotics.robot_mode_enabled
    }

    /// Returns the robot model name, if available.
    pub fn robot_model(&self) -> Option<String> {
        self.state.robotics.robot_model.clone()
    }

    /// Returns the high-level robot class (e.g., "quadruped", "humanoid").
    pub fn robot_class(&self) -> Option<String> {
        self.state.robotics.robot_class.clone()
    }

    /// Returns the current behavior mode (e.g., "idle", "patrol").
    pub fn behavior_mode(&self) -> Option<String> {
        self.state.robotics.behavior_mode.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Power & Battery
    // --------------------------------------------------------------------------------------------

    /// Returns the battery level percentage (0.0–100.0).
    pub fn battery_level(&self) -> Option<f32> {
        self.state.robotics.battery_level_percent
    }

    /// Returns the estimated remaining runtime in minutes.
    pub fn battery_runtime_minutes(&self) -> Option<f32> {
        self.state.robotics.battery_runtime_minutes
    }

    /// Returns the battery temperature in Celsius.
    pub fn battery_temperature_c(&self) -> Option<f32> {
        self.state.robotics.battery_temperature_c
    }

    /// Returns whether the robot is currently charging.
    pub fn is_charging(&self) -> bool {
        self.state.robotics.charging
    }

    /// Returns the power source descriptor.
    pub fn power_source(&self) -> Option<String> {
        self.state.robotics.power_source.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Joints & Actuators
    // --------------------------------------------------------------------------------------------

    /// Returns all joint temperature readings.
    pub fn joint_temperatures(&self) -> &HashMap<String, f32> {
        &self.state.robotics.joint_temperatures_c
    }

    /// Returns all joint load percentages.
    pub fn joint_loads(&self) -> &HashMap<String, f32> {
        &self.state.robotics.joint_load_percent
    }

    /// Returns all joint positions.
    pub fn joint_positions(&self) -> &HashMap<String, f32> {
        &self.state.robotics.joint_positions
    }

    /// Returns all joint velocities.
    pub fn joint_velocities(&self) -> &HashMap<String, f32> {
        &self.state.robotics.joint_velocities
    }

    /// Returns all joint torques.
    pub fn joint_torques(&self) -> &HashMap<String, f32> {
        &self.state.robotics.joint_torques
    }

    /// Returns the maximum joint load percentage across all joints.
    pub fn max_joint_load(&self) -> Option<f32> {
        self.state.robotics.joint_load_max_percent
    }

    /// Returns the maximum joint temperature across all joints.
    pub fn max_joint_temperature(&self) -> Option<f32> {
        self.state.robotics.joint_temperature_max_c
    }

    // --------------------------------------------------------------------------------------------
    //  Locomotion & Gait
    // --------------------------------------------------------------------------------------------

    /// Returns the current locomotion mode (e.g., "walk", "drive").
    pub fn locomotion_mode(&self) -> Option<String> {
        self.state.robotics.locomotion_mode.clone()
    }

    /// Returns the gait stability score (0.0–1.0).
    pub fn gait_stability(&self) -> Option<f32> {
        self.state.robotics.gait_stability_score
    }

    /// Returns the current linear velocity in m/s.
    pub fn linear_velocity_mps(&self) -> Option<f32> {
        self.state.robotics.linear_velocity_mps
    }

    /// Returns the current angular velocity in rad/s.
    pub fn angular_velocity_rads(&self) -> Option<f32> {
        self.state.robotics.angular_velocity_rads
    }

    /// Returns the traction score (0.0–1.0).
    pub fn traction_score(&self) -> Option<f32> {
        self.state.robotics.traction_score
    }

    // --------------------------------------------------------------------------------------------
    //  Localization & Navigation
    // --------------------------------------------------------------------------------------------

    /// Returns the current pose (serialized).
    pub fn pose(&self) -> Option<String> {
        self.state.robotics.pose.clone()
    }

    /// Returns the current map frame identifier.
    pub fn map_frame(&self) -> Option<String> {
        self.state.robotics.map_frame.clone()
    }

    /// Returns the localization status.
    pub fn localization_status(&self) -> Option<String> {
        self.state.robotics.localization_status.clone()
    }

    /// Returns the navigation mode.
    pub fn navigation_mode(&self) -> Option<String> {
        self.state.robotics.navigation_mode.clone()
    }

    /// Returns the current navigation goal (serialized).
    pub fn navigation_goal(&self) -> Option<String> {
        self.state.robotics.navigation_goal.clone()
    }

    /// Returns the distance to goal in meters.
    pub fn navigation_distance_to_goal_m(&self) -> Option<f32> {
        self.state.robotics.navigation_distance_to_goal_m
    }

    /// Returns the estimated time to goal in seconds.
    pub fn navigation_eta_seconds(&self) -> Option<f32> {
        self.state.robotics.navigation_eta_seconds
    }

    // --------------------------------------------------------------------------------------------
    //  Perception & Environment Interaction
    // --------------------------------------------------------------------------------------------

    /// Returns the IMU orientation descriptor.
    pub fn imu_orientation(&self) -> Option<String> {
        self.state.robotics.imu_orientation.clone()
    }

    /// Returns the raw IMU orientation data (serialized).
    pub fn imu_raw(&self) -> Option<String> {
        self.state.robotics.imu_raw.clone()
    }

    /// Returns the list of active perception sensors.
    pub fn perception_sensors(&self) -> &[String] {
        &self.state.robotics.perception_sensors
    }

    /// Returns whether a specific perception sensor is healthy.
    pub fn perception_sensor_health(&self, name: &str) -> Option<bool> {
        self.state
            .robotics
            .perception_sensor_health
            .get(name)
            .cloned()
    }

    /// Returns the last update timestamp for a specific perception sensor.
    pub fn perception_last_update(&self, name: &str) -> Option<String> {
        self.state
            .robotics
            .perception_last_update
            .get(name)
            .cloned()
    }

    /// Returns the list of detected obstacles.
    pub fn obstacles(&self) -> &[String] {
        &self.state.robotics.obstacles
    }

    /// Returns the list of detected agents (e.g., humans, other robots).
    pub fn detected_agents(&self) -> &[String] {
        &self.state.robotics.detected_agents
    }

    // --------------------------------------------------------------------------------------------
    //  Manipulation & End-Effectors
    // --------------------------------------------------------------------------------------------

    /// Returns the list of manipulators.
    pub fn manipulators(&self) -> &[String] {
        &self.state.robotics.manipulators
    }

    /// Returns the end-effector pose for a given manipulator (serialized).
    pub fn end_effector_pose(&self, manipulator: &str) -> Option<String> {
        self.state
            .robotics
            .end_effector_pose
            .get(manipulator)
            .cloned()
    }

    /// Returns the grasp state for a given manipulator.
    pub fn grasp_state(&self, manipulator: &str) -> Option<String> {
        self.state
            .robotics
            .grasp_state
            .get(manipulator)
            .cloned()
    }

    /// Returns the grasp force for a given manipulator.
    pub fn grasp_force(&self, manipulator: &str) -> Option<f32> {
        self.state
            .robotics
            .grasp_force
            .get(manipulator)
            .cloned()
    }

    /// Returns the list of currently held objects.
    pub fn held_objects(&self) -> &[String] {
        &self.state.robotics.held_objects
    }

    // --------------------------------------------------------------------------------------------
    //  Safety, Limits & Faults
    // --------------------------------------------------------------------------------------------

    /// Returns whether an emergency stop is active.
    pub fn estop_active(&self) -> bool {
        self.state.robotics.estop_active
    }

    /// Returns whether motion is currently inhibited by safety.
    pub fn motion_inhibited(&self) -> bool {
        self.state.robotics.motion_inhibited
    }

    /// Returns the list of active safety zones.
    pub fn safety_zones_active(&self) -> &[String] {
        &self.state.robotics.safety_zones_active
    }

    /// Returns whether a specific joint has a limit violation.
    pub fn joint_limit_violation(&self, joint: &str) -> Option<bool> {
        self.state
            .robotics
            .joint_limit_violations
            .get(joint)
            .cloned()
    }

    /// Returns the list of active fault codes.
    pub fn fault_codes(&self) -> &[String] {
        &self.state.robotics.fault_codes
    }

    /// Returns the list of human-readable fault descriptions.
    pub fn fault_descriptions(&self) -> &[String] {
        &self.state.robotics.fault_descriptions
    }

    // --------------------------------------------------------------------------------------------
    //  Telemetry & Diagnostics
    // --------------------------------------------------------------------------------------------

    /// Returns the overall robotics health score (0.0–1.0).
    pub fn health_score(&self) -> f32 {
        self.state.robotics.health_score
    }

    /// Returns the high-level robotics status summary.
    pub fn status_summary(&self) -> Option<String> {
        self.state.robotics.status_summary.clone()
    }

    /// Returns recent robotics log entries.
    pub fn logs(&self) -> &[String] {
        &self.state.robotics.logs
    }

    /// Returns the timestamp of the last robotics update.
    pub fn last_update(&self) -> Option<String> {
        self.state.robotics.last_update.clone()
    }
}
