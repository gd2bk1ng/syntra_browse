// ================================================================================================
//   SyntraOS — CONTROL CENTER STATE (FULL EXPANDED MODEL)
// ------------------------------------------------------------------------------------------------
//   File:        src/control_center/state.rs
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Central nervous system for the SyntraOS shell. This file models:
//         - OS spaces (desktop, robot HUD, home, console)
//         - System, cognition, robotics, network, diagnostics, evolution, settings
//         - Smart home, security, assistant persona, notifications
//         - Thoughtstream, world model, multi-agent runtime, memory architecture
//         - Safety & governance, robotics HAL, network topology, evolution engine
//         - Plugin system, simulation sandbox, predictive engine, ML/federated learning
//         - Continuity/session timeline, UI shell state, HUD rendering
//       Frontends (desktop, web, robot HUD, AR) bind to this, not to raw internals.
//       Designed to be extended safely as Syntra Kernel evolves.
// ================================================================================================

#![allow(dead_code)]

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::agi_core::ecosystem::EcosystemModel;
use crate::agi_core::theme::ThemePack;

/// Top-level navigation sections in the Control Center.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ControlCenterSection {
    System,
    Cognition,
    Robotics,
    Network,
    ThemesIdentity,
    Diagnostics,
    Evolution,
    Settings,
    SmartHome,
    Security,
    Assistant,
    WorldModel,
    Agents,
    Memory,
    Safety,
    Predictive,
    Simulation,
    Plugins,
    Continuity,
}

/// High-level SyntraOS "spaces" / modes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SyntraSpace {
    Desktop,
    RobotHud,
    Home,
    Console,
}

/// Severity for notifications / alerts.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// High-level status summary for the System panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemStatusSummary {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub uptime: Duration,

    pub cpu_model: String,
    pub cpu_cores: u8,
    pub cpu_threads: u8,
    pub cpu_usage_percent: f32,
    pub cpu_temperature_c: Option<f32>,

    pub ram_total_gb: f32,
    pub ram_used_gb: f32,

    pub gpu_model: Option<String>,
    pub gpu_usage_percent: Option<f32>,
    pub gpu_temperature_c: Option<f32>,

    pub storage_total_gb: f32,
    pub storage_used_gb: f32,

    pub battery_percent: Option<f32>,
    pub power_source: Option<String>, // "AC", "Battery", "Unknown"
}

/// High-level cognition summary for the Cognition panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CognitionSummary {
    pub active_intents: Vec<String>,
    pub active_tasks: Vec<String>,
    pub routing_confidence_avg: f32,
    pub safety_block_count_recent: u32,
    pub feedback_events_recent: u32,
}

/// High-level robotics summary for the Robotics panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoboticsSummary {
    pub robot_mode_enabled: bool,
    pub robot_model: Option<String>,
    pub battery_percent: Option<f32>,
    pub joint_temperature_max_c: Option<f32>,
    pub joint_load_max_percent: Option<f32>,
    pub gait_stability_score: Option<f32>, // 0.0–1.0
    pub imu_orientation: Option<String>,   // e.g., "upright", "tilted", "falling"
    pub sensor_health_ok: bool,
}

/// High-level network summary for the Network panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSummary {
    pub primary_interface: Option<String>,
    pub ip_address: Option<String>,
    pub link_speed_mbps: Option<u32>,
    pub latency_ms: Option<f32>,
    pub packet_loss_percent: Option<f32>,
    pub connected_devices_count: u32,
    pub syntra_nodes_online: u32,
}

/// Theme & identity summary for the Themes & Identity panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeIdentitySummary {
    pub active_theme_name: Option<String>,
    pub available_themes: Vec<String>,
    pub institution_name: Option<String>,
    pub watermark: Option<String>,
    pub logo_path: Option<String>,
}

/// Diagnostics summary for the Diagnostics panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagnosticsSummary {
    pub hardware_health_ok: bool,
    pub storage_health_ok: bool,
    pub thermal_health_ok: bool,
    pub last_diagnostic_run: Option<String>,
    pub warnings_recent: Vec<String>,
    pub errors_recent: Vec<String>,
}

/// Evolution summary for the Evolution panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvolutionSummary {
    pub ecosystem_health_score: f32,
    pub missing_lobes: Vec<String>,
    pub incomplete_lobes: Vec<String>,
    pub upgrade_recommendations: Vec<String>,
    pub evolution_stage: Option<String>,
    pub scheduled_upgrades: Vec<String>,
    pub lobe_health: Vec<LobeHealth>,
}

/// Per-lobe health snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LobeHealth {
    pub name: String,
    pub status: String,
    pub fitness_score: f32,
}

/// Settings summary for the Settings panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsSummary {
    pub syntra_version: String,
    pub update_channel: String, // "stable", "beta", "nightly"
    pub auto_update_enabled: bool,
    pub telemetry_enabled: bool,
    pub robot_mode_enabled: bool,
}

/// Smart home summary for the Smart Home panel / Home space.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmartHomeSummary {
    pub scenes: Vec<String>,                 // e.g., "Night", "Away", "Studio"
    pub active_scene: Option<String>,
    pub lights_on_count: u32,
    pub lights_total_count: u32,
    pub doors_locked_count: u32,
    pub doors_total_count: u32,
    pub cameras_online_count: u32,
    pub cameras_total_count: u32,
    pub energy_usage_kw: Option<f32>,
    pub alerts_recent: Vec<String>,
    pub device_graph_summary: Vec<RoomDevicesSummary>,
}

/// Per-room device summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoomDevicesSummary {
    pub room_name: String,
    pub devices: Vec<String>, // e.g., "Ceiling Light", "Door Lock", "Thermostat"
}

/// Security summary for the Security panel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecuritySummary {
    pub system_armed: bool,
    pub perimeter_secure: bool,
    pub last_breach: Option<String>,
    pub active_alerts: Vec<String>,
    pub sandbox_enabled: bool,
    pub sandbox_policy_profile: Option<String>,
    pub security_zones: Vec<SecurityZone>,
}

/// Security zone (e.g., "Perimeter", "Interior", "Garage").
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityZone {
    pub name: String,
    pub secure: bool,
    pub last_event: Option<String>,
}

/// Assistant summary for the Assistant panel / Console space.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssistantSummary {
    pub persona_name: String,          // e.g., "Syntra", "Friday", etc.
    pub active_conversation_id: Option<String>,
    pub last_user_utterance: Option<String>,
    pub last_assistant_reply: Option<String>,
    pub pending_actions: Vec<String>,  // high-level descriptions of queued actions
    pub attention_level: f32,          // 0.0–1.0, conceptual "focus" metric
    pub mood: Option<String>,          // optional persona mood
    pub tone: Option<String>,          // optional conversational tone
}

/// Notification / alert item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub severity: Severity,
    pub timestamp: String,
    pub read: bool,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            message: String::new(),
            severity: Severity::Info,
            timestamp: String::new(),
            read: false,
        }
    }
}

/// Thoughtstream / cognitive loop summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThoughtstreamSummary {
    pub current_thought: Option<String>,
    pub last_reasoning_step: Option<String>,
    pub active_pipeline: Option<String>,
    pub cognitive_load: f32, // 0.0–1.0
    pub safety_state: Option<String>,
    pub emotional_tone: Option<String>,
}

/// World model summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldModelSummary {
    pub known_entities_count: u32,
    pub known_locations_count: u32,
    pub active_context: Option<String>,
    pub uncertainty_score: f32,
    pub prediction_horizon_seconds: u32,
    pub highlighted_entities: Vec<String>,
}

/// Multi-agent runtime summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentRuntimeSummary {
    pub active_agents: Vec<String>,
    pub sleeping_agents: Vec<String>,
    pub agent_health: Vec<AgentHealth>,
    pub agent_conflicts: Vec<String>,
    pub agent_bandwidth_percent: f32,
}

/// Per-agent health snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentHealth {
    pub name: String,
    pub status: String,
    pub load_percent: f32,
}

/// Memory architecture summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemorySummary {
    pub episodic_memory_recent: Vec<String>,
    pub semantic_memory_topics: Vec<String>,
    pub working_memory_slots_used: u32,
    pub working_memory_slots_total: u32,
    pub memory_pressure: f32,
    pub memory_retention_score: f32,
}

/// Safety & governance summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SafetySummary {
    pub last_safety_verdict: Option<String>,
    pub blocked_actions_recent: Vec<String>,
    pub policy_profile: Option<String>,
    pub risk_score: f32,
    pub compliance_state: Option<String>,
}

/// Robotics HAL / HUD summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoboticsHalSummary {
    pub motor_status: Option<String>,
    pub sensor_map: Vec<String>,
    pub localization_state: Option<String>,
    pub path_planning_state: Option<String>,
    pub robot_mode: Option<String>, // e.g., "idle", "patrol", "follow"
}

/// Network topology summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkTopologySummary {
    pub network_graph_nodes: u32,
    pub network_graph_edges: u32,
    pub syntra_nodes: Vec<String>,
    pub mesh_health_score: f32,
}

/// Predictive engine summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PredictiveSummary {
    pub predictions: Vec<String>,
    pub confidence_scores: Vec<f32>,
    pub anomalies: Vec<String>,
    pub trends: Vec<String>,
}

/// Simulation sandbox summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimulationSummary {
    pub simulation_running: bool,
    pub simulation_time_seconds: f32,
    pub simulation_entities_count: u32,
    pub simulation_metrics: Vec<String>,
}

/// Plugin / extension system summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginSummary {
    pub installed_plugins: Vec<String>,
    pub plugin_health: Vec<PluginHealth>,
    pub plugin_events_recent: Vec<String>,
}

/// Per-plugin health snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginHealth {
    pub name: String,
    pub status: String,
    pub permissions: Vec<String>,
}

/// ML / federated learning summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MlSummary {
    pub training_jobs: Vec<String>,
    pub model_versions: Vec<String>,
    pub accuracy_metrics: Vec<String>,
    pub drift_score: f32,
}

/// Continuity / session timeline summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContinuitySummary {
    pub session_history: Vec<String>,
    pub last_unlock: Option<String>,
    pub last_activity: Option<String>,
    pub continuity_score: f32,
}

/// UI shell state for holographic desktop.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiShellState {
    pub open_panels: Vec<String>,
    pub panel_layout: Option<String>,
    pub active_window: Option<String>,
    pub hologram_intensity: f32, // 0.0–1.0
    pub depth_effects_enabled: bool,
    pub gesture_mode_enabled: bool,
}

/// HUD rendering state (robot HUD / AR).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HudState {
    pub hud_layers: Vec<String>,
    pub hud_focus_target: Option<String>,
    pub hud_alert_level: f32, // 0.0–1.0
    pub hud_color_profile: Option<String>,
}

/// The full Control Center state.
///
/// Single source of truth for the SyntraOS shell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCenterState {
    /// Which section/panel is currently active in the Control Center UI.
    pub active_section: ControlCenterSection,

    /// Which SyntraOS "space" is currently active (desktop, robot HUD, home, console).
    pub active_space: SyntraSpace,

    pub system: SystemStatusSummary,
    pub cognition: CognitionSummary,
    pub robotics: RoboticsSummary,
    pub network: NetworkSummary,
    pub themes_identity: ThemeIdentitySummary,
    pub diagnostics: DiagnosticsSummary,
    pub evolution: EvolutionSummary,
    pub settings: SettingsSummary,
    pub smart_home: SmartHomeSummary,
    pub security: SecuritySummary,
    pub assistant: AssistantSummary,

    pub thoughtstream: ThoughtstreamSummary,
    pub world_model: WorldModelSummary,
    pub agents: AgentRuntimeSummary,
    pub memory: MemorySummary,
    pub safety: SafetySummary,
    pub robotics_hal: RoboticsHalSummary,
    pub network_topology: NetworkTopologySummary,
    pub predictive: PredictiveSummary,
    pub simulation: SimulationSummary,
    pub plugins: PluginSummary,
    pub ml: MlSummary,
    pub continuity: ContinuitySummary,

    pub ui_shell: UiShellState,
    pub hud: HudState,

    pub notifications: Vec<Notification>,

    #[serde(skip)]
    pub active_theme: Option<ThemePack>,
}

impl Default for ControlCenterState {
    fn default() -> Self {
        Self {
            active_section: ControlCenterSection::System,
            active_space: SyntraSpace::Desktop,
            system: SystemStatusSummary::default(),
            cognition: CognitionSummary::default(),
            robotics: RoboticsSummary::default(),
            network: NetworkSummary::default(),
            themes_identity: ThemeIdentitySummary::default(),
            diagnostics: DiagnosticsSummary::default(),
            evolution: EvolutionSummary::default(),
            settings: SettingsSummary {
                syntra_version: "0.1.0-dev".to_string(),
                update_channel: "dev".to_string(),
                auto_update_enabled: false,
                telemetry_enabled: true,
                robot_mode_enabled: false,
            },
            smart_home: SmartHomeSummary::default(),
            security: SecuritySummary::default(),
            assistant: AssistantSummary {
                persona_name: "Syntra".to_string(),
                ..AssistantSummary::default()
            },
            thoughtstream: ThoughtstreamSummary::default(),
            world_model: WorldModelSummary::default(),
            agents: AgentRuntimeSummary::default(),
            memory: MemorySummary::default(),
            safety: SafetySummary::default(),
            robotics_hal: RoboticsHalSummary::default(),
            network_topology: NetworkTopologySummary::default(),
            predictive: PredictiveSummary::default(),
            simulation: SimulationSummary::default(),
            plugins: PluginSummary::default(),
            ml: MlSummary::default(),
            continuity: ContinuitySummary::default(),
            ui_shell: UiShellState {
                hologram_intensity: 0.8,
                depth_effects_enabled: true,
                gesture_mode_enabled: false,
                ..UiShellState::default()
            },
            hud: HudState::default(),
            notifications: Vec::new(),
            active_theme: None,
        }
    }
}

impl ControlCenterState {
    pub fn from_core(
        active_theme: Option<ThemePack>,
        ecosystem: Option<&EcosystemModel>,
    ) -> Self {
        let mut state = ControlCenterState::default();
        state.active_theme = active_theme;

        if let Some(ecosystem) = ecosystem {
            state.evolution.ecosystem_health_score = ecosystem.health_score;
            state.evolution.missing_lobes = ecosystem.missing.clone();
            state.evolution.incomplete_lobes = ecosystem.incomplete.clone();
            state.evolution.upgrade_recommendations =
                ecosystem.upgrade_recommendations.clone();
        }

        if let Some(theme) = &state.active_theme {
            state.themes_identity.active_theme_name = Some(theme.name.clone());
            state.themes_identity.institution_name =
                theme.identity.institution.clone();
            state.themes_identity.watermark = theme.identity.watermark.clone();
            state.themes_identity.logo_path = theme.identity.logo_path.clone();
        }

        state
    }

    pub fn set_active_section(&mut self, section: ControlCenterSection) {
        self.active_section = section;
    }

    pub fn set_active_space(&mut self, space: SyntraSpace) {
        self.active_space = space;
    }

    pub fn set_active_theme(&mut self, theme: ThemePack) {
        self.themes_identity.active_theme_name = Some(theme.name.clone());
        self.themes_identity.institution_name = theme.identity.institution.clone();
        self.themes_identity.watermark = theme.identity.watermark.clone();
        self.themes_identity.logo_path = theme.identity.logo_path.clone();
        self.active_theme = Some(theme);
    }

    pub fn set_available_themes(&mut self, names: Vec<String>) {
        self.themes_identity.available_themes = names;
    }

    pub fn push_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }
}
