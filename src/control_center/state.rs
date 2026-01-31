// ================================================================================================
//   SyntraOS — CONTROL CENTER STATE
// ------------------------------------------------------------------------------------------------
//   File:        src/control_center/state.rs
//   Description:
//   Author: Alexandr Roussinov (gd2bk1ng)
//       Shared state model for the SyntraOS Control Center. Frontends (desktop, web, robot HUD)
//       bind to this, not to raw internals. Designed to be extended safely.
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

/// The full Control Center state.
///
/// Single source of truth for the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCenterState {
    pub active_section: ControlCenterSection,

    pub system: SystemStatusSummary,
    pub cognition: CognitionSummary,
    pub robotics: RoboticsSummary,
    pub network: NetworkSummary,
    pub themes_identity: ThemeIdentitySummary,
    pub diagnostics: DiagnosticsSummary,
    pub evolution: EvolutionSummary,
    pub settings: SettingsSummary,

    #[serde(skip)]
    pub active_theme: Option<ThemePack>,
}

impl Default for ControlCenterState {
    fn default() -> Self {
        Self {
            active_section: ControlCenterSection::System,
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
}
