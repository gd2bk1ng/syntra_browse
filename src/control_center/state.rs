/* ================================================================================================
   SYNTRAOS — ROBOTICS STATE MODEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/state.rs (excerpt)
   Module:      SyntraOS Control Center — Robotics State
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Canonical robotics subsystem state model for advanced platforms.

       Designed to cover:
         • Power & battery
         • Locomotion (wheeled / legged)
         • Manipulators & end-effectors
         • Joints, actuators, and kinematics
         • Localization & navigation
         • Perception & environment interaction
         • Safety, limits, and fault states
         • Telemetry & diagnostics

       This state is consumed by:
         • RoboticsPanel (read-only view)
         • AGI Core (embodiment & action lobe)
         • Simulation / sandbox
         • Diagnostics & safety governance
   ================================================================================================ */

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoboticsState {
    // --------------------------------------------------------------------------------------------
    //  Identity & Mode
    // --------------------------------------------------------------------------------------------

    /// Whether robot embodiment mode is active.
    pub robot_mode_enabled: bool,

    /// Robot model identifier (e.g., "Atlas-X3", "SyntraWalker-2").
    pub robot_model: Option<String>,

    /// High-level robot class (e.g., "quadruped", "humanoid", "mobile_base", "manipulator").
    pub robot_class: Option<String>,

    /// Current high-level behavior mode (e.g., "idle", "patrol", "follow", "manipulate").
    pub behavior_mode: Option<String>,

    // --------------------------------------------------------------------------------------------
    //  Power & Battery
    // --------------------------------------------------------------------------------------------

    /// Battery level in percent (0.0–100.0).
    pub battery_level_percent: Option<f32>,

    /// Estimated remaining runtime in minutes.
    pub battery_runtime_minutes: Option<f32>,

    /// Battery temperature in Celsius.
    pub battery_temperature_c: Option<f32>,

    /// Whether the robot is currently charging.
    pub charging: bool,

    /// Power source descriptor (e.g., "battery", "external", "hybrid").
    pub power_source: Option<String>,

    // --------------------------------------------------------------------------------------------
    //  Joints & Actuators
    // --------------------------------------------------------------------------------------------

    /// Per-joint temperature readings in Celsius.
    /// Example: { "hip_left": 42.1, "knee_right": 38.7 }
    pub joint_temperatures_c: HashMap<String, f32>,

    /// Per-joint load percentage (0.0–100.0).
    pub joint_load_percent: HashMap<String, f32>,

    /// Per-joint position in radians or degrees (implementation-defined).
    pub joint_positions: HashMap<String, f32>,

    /// Per-joint velocity (units implementation-defined).
    pub joint_velocities: HashMap<String, f32>,

    /// Per-joint torque (units implementation-defined).
    pub joint_torques: HashMap<String, f32>,

    /// Maximum joint load percentage across all joints.
    pub joint_load_max_percent: Option<f32>,

    /// Maximum joint temperature across all joints.
    pub joint_temperature_max_c: Option<f32>,

    // --------------------------------------------------------------------------------------------
    //  Locomotion & Gait
    // --------------------------------------------------------------------------------------------

    /// Current locomotion mode (e.g., "stand", "walk", "trot", "crawl", "drive").
    pub locomotion_mode: Option<String>,

    /// Gait stability score (0.0–1.0).
    pub gait_stability_score: Option<f32>,

    /// Current linear velocity (m/s).
    pub linear_velocity_mps: Option<f32>,

    /// Current angular velocity (rad/s).
    pub angular_velocity_rads: Option<f32>,

    /// Slip or traction indicator (0.0–1.0).
    pub traction_score: Option<f32>,

    // --------------------------------------------------------------------------------------------
    //  Localization & Navigation
    // --------------------------------------------------------------------------------------------

    /// Current pose in a serialized format (e.g., JSON, protobuf).
    pub pose: Option<String>,

    /// Current map frame identifier (e.g., "map", "world").
    pub map_frame: Option<String>,

    /// Localization status (e.g., "ok", "lost", "initializing").
    pub localization_status: Option<String>,

    /// Navigation mode (e.g., "manual", "autonomous", "assisted").
    pub navigation_mode: Option<String>,

    /// Current navigation goal (serialized).
    pub navigation_goal: Option<String>,

    /// Distance to goal in meters.
    pub navigation_distance_to_goal_m: Option<f32>,

    /// Estimated time to goal in seconds.
    pub navigation_eta_seconds: Option<f32>,

    // --------------------------------------------------------------------------------------------
    //  Perception & Environment Interaction
    // --------------------------------------------------------------------------------------------

    /// IMU orientation descriptor (e.g., "upright", "tilted", "falling").
    pub imu_orientation: Option<String>,

    /// IMU raw orientation data (serialized quaternion / Euler).
    pub imu_raw: Option<String>,

    /// List of active perception sensors (e.g., "lidar_front", "camera_head", "depth_rear").
    pub perception_sensors: Vec<String>,

    /// Per-sensor health status.
    pub perception_sensor_health: HashMap<String, bool>,

    /// Per-sensor last update timestamp (string-encoded).
    pub perception_last_update: HashMap<String, String>,

    /// List of detected obstacles (serialized descriptors).
    pub obstacles: Vec<String>,

    /// List of detected humans or agents (serialized descriptors).
    pub detected_agents: Vec<String>,

    // --------------------------------------------------------------------------------------------
    //  Manipulation & End-Effectors
    // --------------------------------------------------------------------------------------------

    /// List of manipulators (e.g., "left_arm", "right_arm").
    pub manipulators: Vec<String>,

    /// Per-manipulator end-effector pose (serialized).
    pub end_effector_pose: HashMap<String, String>,

    /// Per-manipulator grasp state (e.g., "open", "closed", "holding").
    pub grasp_state: HashMap<String, String>,

    /// Per-manipulator grasp force (units implementation-defined).
    pub grasp_force: HashMap<String, f32>,

    /// Currently held objects (serialized descriptors).
    pub held_objects: Vec<String>,

    // --------------------------------------------------------------------------------------------
    //  Safety, Limits & Faults
    // --------------------------------------------------------------------------------------------

    /// Whether an emergency stop is active.
    pub estop_active: bool,

    /// Whether motion is currently inhibited by safety.
    pub motion_inhibited: bool,

    /// List of active safety zones (e.g., "human_nearby", "restricted_area").
    pub safety_zones_active: Vec<String>,

    /// Per-joint limit violation flags.
    pub joint_limit_violations: HashMap<String, bool>,

    /// List of active fault codes.
    pub fault_codes: Vec<String>,

    /// Human-readable fault descriptions.
    pub fault_descriptions: Vec<String>,

    // --------------------------------------------------------------------------------------------
    //  Telemetry & Diagnostics
    // --------------------------------------------------------------------------------------------

    /// Overall robotics health score (0.0–1.0).
    pub health_score: f32,

    /// High-level status summary (e.g., "nominal", "degraded", "faulted").
    pub status_summary: Option<String>,

    /// Recent robotics log entries.
    pub logs: Vec<String>,

    /// Timestamp of last robotics update.
    pub last_update: Option<String>,
}
