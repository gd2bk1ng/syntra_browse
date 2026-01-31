// ================================================================================================
//   SYNTRAOS — CORE RUNTIME NODE
// ------------------------------------------------------------------------------------------------
//   File:        src/agi_core/node.rs
//   Module:      SyntraNode Runtime
//   Description:
//       Central runtime struct that ties together themes, ecosystem model, and the Control Center
//       state. This is the long-lived process that frontends (desktop UI, web UI, robot HUD) talk to.
// ================================================================================================

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use crate::agi_core::ecosystem::EcosystemModel;
use crate::agi_core::telemetry::{TelemetryBus, TelemetryLevel};
use crate::agi_core::theme::{ThemePack, ThemeRegistry};
use crate::control_center::ControlCenterState;

/// Core runtime node for SyntraOS.
///
/// This is the "brain" process that:
///   - loads themes
///   - scans the ecosystem
///   - maintains Control Center state
///   - exposes APIs (later) for UIs and robots to connect to
pub struct SyntraNode {
    root: PathBuf,

    pub telemetry: TelemetryBus,
    pub themes: ThemeRegistry,
    pub ecosystem: EcosystemModel,
    pub control_center: ControlCenterState,
}

impl SyntraNode {
    /// Create a new SyntraNode rooted at the given path.
    ///
    /// `root` is typically the project or installation directory, containing:
    ///   - themes/
    ///   - assets/
    ///   - src/
    pub fn new(root: impl AsRef<Path>, telemetry: TelemetryBus) -> Self {
        let root = root.as_ref().to_path_buf();

        // Initialize theme registry.
        let themes_dir = root.join("themes");
        let mut themes = ThemeRegistry::new(&themes_dir);
        themes.load_all();

        // Pick an active theme.
        let active_theme = themes
            .get("Syntra Default")
            .cloned()
            .or_else(|| themes.list().first().and_then(|n| themes.get(n).cloned()));

        if let Some(t) = &active_theme {
            telemetry.log(
                TelemetryLevel::Info,
                format!("SyntraNode: active theme set to '{}'", t.name),
            );
        } else {
            telemetry.log(
                TelemetryLevel::Warn,
                "SyntraNode: no themes found; running without active theme".to_string(),
            );
        }

        // Initialize ecosystem model.
        let mut ecosystem = EcosystemModel::with_telemetry(telemetry.clone());
        ecosystem.scan_repo(&root);

        // Initialize Control Center state from core subsystems.
        let mut control_center =
            ControlCenterState::from_core(active_theme.clone(), Some(&ecosystem));
        control_center.set_available_themes(themes.list());

        SyntraNode {
            root,
            telemetry,
            themes,
            ecosystem,
            control_center,
        }
    }

    /// Reload themes from disk and update Control Center state.
    pub fn reload_themes(&mut self) {
        self.telemetry.log(
            TelemetryLevel::Info,
            "SyntraNode: reloading themes".to_string(),
        );

        self.themes.reload();
        let names = self.themes.list();
        self.control_center.set_available_themes(names.clone());

        // Try to keep the same active theme if possible.
        let current_name = self
            .control_center
            .themes_identity
            .active_theme_name
            .clone();

        let new_active = if let Some(name) = current_name {
            self.themes.get(&name).cloned().or_else(|| {
                names
                    .first()
                    .and_then(|fallback| self.themes.get(fallback).cloned())
            })
        } else {
            names
                .first()
                .and_then(|fallback| self.themes.get(fallback).cloned())
        };

        if let Some(theme) = new_active {
            self.control_center.set_active_theme(theme.clone());
            self.telemetry.log(
                TelemetryLevel::Info,
                format!("SyntraNode: active theme now '{}'", theme.name),
            );
        }
    }

    /// Switch the active theme by name.
    pub fn set_active_theme(&mut self, name: &str) {
        if let Some(theme) = self.themes.get(name).cloned() {
            self.control_center.set_active_theme(theme.clone());
            self.telemetry.log(
                TelemetryLevel::Info,
                format!("SyntraNode: active theme switched to '{}'", name),
            );
        } else {
            self.telemetry.log(
                TelemetryLevel::Warn,
                format!(
                    "SyntraNode: requested theme '{}' not found in registry",
                    name
                ),
            );
        }
    }

    /// Re-scan the ecosystem and update Control Center evolution summary.
    pub fn rescan_ecosystem(&mut self) {
        self.telemetry.log(
            TelemetryLevel::Info,
            "SyntraNode: rescanning ecosystem".to_string(),
        );
        self.ecosystem.scan_repo(&self.root);

        self.control_center.evolution.ecosystem_health_score = self.ecosystem.health_score;
        self.control_center.evolution.missing_lobes = self.ecosystem.missing.clone();
        self.control_center.evolution.incomplete_lobes = self.ecosystem.incomplete.clone();
        self.control_center.evolution.upgrade_recommendations =
            self.ecosystem.upgrade_recommendations.clone();
    }

    /// Access the current Control Center state (for UI frontends).
    pub fn control_center_state(&self) -> &ControlCenterState {
        &self.control_center
    }

    /// Mutable access to Control Center state (for internal updates).
    pub fn control_center_state_mut(&mut self) -> &mut ControlCenterState {
        &mut self.control_center
    }
}
