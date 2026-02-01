/* ================================================================================================
   SYNTRAOS — CONTROL CENTER STATE
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/state.rs
   Module:      SyntraOS Control Center — State Model
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Canonical state model for the SyntraOS Control Center.

       This is the single source of truth for:
         • System status
         • Network & robotics
         • Cognitive / world model
         • Agents, memory, safety
         • Simulation, plugins, developer diagnostics

       Panels provide read-only, structured views over this state.
       Commands mutate this state in controlled, auditable ways.
   ================================================================================================ */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCenterState {
    pub system: SystemState,
    pub cognition: CognitionState,
    pub robotics: RoboticsState,
    pub network: NetworkState,
    pub smart_home: SmartHomeState,
    pub security: SecurityState,
    pub diagnostics: DiagnosticsState,
    pub evolution: EvolutionState,
    pub assistant: AssistantState,
    pub world_model: WorldModelState,
    pub agents: AgentsState,
    pub memory: MemoryState,
    pub safety: SafetyState,
    pub predictive: PredictiveState,
    pub simulation: SimulationState,
    pub plugins: PluginsState,
    pub developer: DeveloperState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub hostname: Option<String>,
    pub os_version: Option<String>,
    pub kernel_revision: Option<String>,
    pub uptime_seconds: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub thermal_throttling: bool,
    pub power_profile: Option<String>,
    pub mode: Option<String>, // e.g. "normal", "maintenance", "sandbox"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionState {
    pub active_context: Option<String>,
    pub focus_stack: Vec<String>,
    pub last_observation: Option<String>,
    pub last_thought: Option<String>,
    pub cognitive_load: f32, // 0.0–1.0
    pub loop_phase: Option<String>, // e.g. "perceive", "plan", "act", "reflect"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoboticsState {
    pub robot_mode_enabled: bool,
    pub robot_model: Option<String>,
    pub joint_temperature_max_c: Option<f32>,
    pub joint_load_max_percent: Option<f32>,
    pub gait_stability_score: Option<f32>,
    pub imu_orientation: Option<String>,
    pub sensor_health_ok: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub online: bool,
    pub active_interface: Option<String>,
    pub upload_mbps: f32,
    pub download_mbps: f32,
    pub latency_ms: f32,
    pub packet_loss_percent: f32,
    pub connected_devices: Vec<String>,
    pub topology_map: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartHomeState {
    pub active_scene: Option<String>,
    pub available_scenes: Vec<String>,

    pub lights: HashMap<String, Vec<String>>, // room -> lights
    pub doors: Vec<String>,
    pub door_locked: HashMap<String, bool>,

    pub thermostat_temp_c: Option<f32>,
    pub thermostat_target_c: Option<f32>,

    pub cameras: Vec<String>,
    pub camera_online: HashMap<String, bool>,

    pub energy_usage_watts: f32,
    pub energy_usage_kwh: f32,

    pub alerts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityState {
    pub armed: bool,
    pub mode: Option<String>,
    pub threat_level: f32,

    pub sensors: Vec<String>,
    pub sensor_triggered: HashMap<String, bool>,
    pub last_triggered_sensor: Option<String>,

    pub cameras: Vec<String>,
    pub camera_online: HashMap<String, bool>,

    pub doors: Vec<String>,
    pub door_locked: HashMap<String, bool>,

    pub alerts: Vec<String>,
    pub intrusion_detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsState {
    pub system_health_score: f32,
    pub kernel_health_score: f32,
    pub provider_health_score: f32,

    pub last_run: Option<String>,
    pub last_run_passed: bool,

    pub error_logs: Vec<String>,
    pub warnings: Vec<String>,

    pub integrity_ok: bool,
    pub integrity_violations: Vec<String>,

    pub performance_anomalies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionState {
    pub version: Option<String>,
    pub lineage: Vec<String>,

    pub capability_growth_score: f32,
    pub new_capabilities: Vec<String>,

    pub evolved_modules: Vec<String>,
    pub upcoming_evolution: Vec<String>,

    pub changelog: Vec<String>,

    pub experimental_features: Vec<String>,
    pub experimental_mode: bool,

    pub maturity_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantState {
    pub mode: Option<String>,
    pub status: Option<String>,

    pub conversation: Vec<String>,
    pub last_user_message: Option<String>,
    pub last_assistant_response: Option<String>,

    pub last_intent: Option<String>,
    pub routing_confidence: f32,

    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelState {
    pub spatial_map: Option<String>,
    pub known_locations: Vec<String>,

    pub scene_graph: Option<String>,
    pub recognized_objects: Vec<String>,

    pub persistent_objects: Vec<String>,
    pub object_last_seen: HashMap<String, String>,

    pub predictions: Vec<String>,
    pub prediction_confidence: f32,

    pub uncertainty: f32,

    pub deltas: Vec<String>,
    pub last_update: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsState {
    pub active_agents: Vec<String>,
    pub agent_role: HashMap<String, String>,
    pub agent_state: HashMap<String, String>,

    pub agent_task: HashMap<String, String>,
    pub global_tasks: Vec<String>,

    pub agent_health: HashMap<String, f32>,
    pub agent_confidence: HashMap<String, f32>,

    pub coordination_messages: Vec<String>,
    pub agent_last_message: HashMap<String, String>,

    pub agent_capabilities: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryState {
    pub episodic: Vec<String>,
    pub last_episode: Option<String>,

    pub semantic: Vec<String>,
    pub last_semantic: Option<String>,

    pub clusters: Vec<String>,
    pub cluster_of: HashMap<String, String>,

    pub tags: Vec<String>,
    pub tags_of: HashMap<String, Vec<String>>,

    pub recall_confidence: f32,
    pub search_results: Vec<String>,

    pub health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyState {
    pub mode: Option<String>,
    pub override_active: bool,

    pub active_blocks: Vec<String>,
    pub triggered_rules: Vec<String>,

    pub risk_score: f32,
    pub risk_factors: Vec<String>,

    pub logs: Vec<String>,
    pub last_event: Option<String>,

    pub confidence: f32,
    pub health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveState {
    pub forecasts: Vec<String>,
    pub window: Option<String>,

    pub confidence: f32,
    pub forecast_confidence: Vec<f32>,

    pub trends: Vec<String>,
    pub dominant_trend: Option<String>,

    pub anomalies: Vec<String>,
    pub anomaly_severity: f32,

    pub projected_state: Option<String>,
    pub last_update: Option<String>,

    pub health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub scenarios: Vec<String>,
    pub active_scenario: Option<String>,

    pub physics_model: Option<String>,
    pub behavior_model: Option<String>,

    pub environment_state: Option<String>,
    pub environment_variables: Vec<String>,

    pub results: Vec<String>,
    pub summary: Option<String>,

    pub logs: Vec<String>,
    pub last_log: Option<String>,

    pub last_run: Option<String>,
    pub last_run_duration_ms: Option<u64>,

    pub health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsState {
    pub installed: Vec<String>,
    pub active: Vec<String>,

    pub metadata: HashMap<String, Vec<String>>,
    pub version: HashMap<String, String>,

    pub capabilities: HashMap<String, Vec<String>>,
    pub permissions: HashMap<String, Vec<String>>,

    pub health: HashMap<String, f32>,
    pub subsystem_health: f32,

    pub logs: HashMap<String, Vec<String>>,
    pub last_log: HashMap<String, String>,

    pub update_available: HashMap<String, bool>,
    pub plugins_with_updates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperState {
    pub build_id: Option<String>,
    pub build_timestamp: Option<String>,
    pub compiler_version: Option<String>,

    pub feature_flags: HashMap<String, bool>,

    pub debug_channels: Vec<String>,
    pub debug_last: HashMap<String, String>,

    pub cpu_profile: Option<String>,
    pub memory_profile: Option<String>,
    pub hotspots: Vec<String>,

    pub hot_reload_enabled: bool,
    pub hot_reload_log: Vec<String>,

    pub modules: Vec<String>,
    pub module_metadata: HashMap<String, Vec<String>>,

    pub experimental_toggles: Vec<String>,
    pub experimental_active: HashMap<String, bool>,

    pub logs: Vec<String>,
    pub last_log: Option<String>,

    pub kv: HashMap<String, String>,
    pub config: HashMap<String, String>,

    pub hooks: Vec<String>,
    pub last_hook: Option<String>,

    pub sandbox_state: Option<String>,
    pub sandbox_warnings: Vec<String>,
}
