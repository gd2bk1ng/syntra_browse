/* ================================================================================================
   SYNTRAOS — SAFETY PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/safety.rs
   Module:      SyntraOS Control Center — Safety Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Safety subsystem:
         • Safety mode (strict, balanced, permissive)
         • Safety blocks
         • Triggered safety rules
         • Override state
         • Risk assessments
         • Safety logs
         • Safety confidence score
         • Subsystem health

       This panel is UI-agnostic. It prepares safety data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SafetyPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SafetyPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Mode & Status
    // --------------------------------------------------------------------------------------------

    /// Returns the current safety mode (e.g., "strict", "balanced", "permissive").
    pub fn mode(&self) -> Option<String> {
        self.state.safety.mode.clone()
    }

    /// Returns whether safety override is currently active.
    pub fn override_active(&self) -> bool {
        self.state.safety.override_active
    }

    // --------------------------------------------------------------------------------------------
    //  Safety Blocks & Triggers
    // --------------------------------------------------------------------------------------------

    /// Returns the list of active safety blocks.
    pub fn active_blocks(&self) -> &[String] {
        &self.state.safety.active_blocks
    }

    /// Returns the list of triggered safety rules.
    pub fn triggered_rules(&self) -> &[String] {
        &self.state.safety.triggered_rules
    }

    // --------------------------------------------------------------------------------------------
    //  Risk Assessment
    // --------------------------------------------------------------------------------------------

    /// Returns the current risk score (0.0–1.0).
    pub fn risk_score(&self) -> f32 {
        self.state.safety.risk_score
    }

    /// Returns the list of risk factors.
    pub fn risk_factors(&self) -> &[String] {
        &self.state.safety.risk_factors
    }

    // --------------------------------------------------------------------------------------------
    //  Logs
    // --------------------------------------------------------------------------------------------

    /// Returns the list of safety log entries.
    pub fn logs(&self) -> &[String] {
        &self.state.safety.logs
    }

    /// Returns the last safety event.
    pub fn last_event(&self) -> Option<String> {
        self.state.safety.last_event.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Confidence & Health
    // --------------------------------------------------------------------------------------------

    /// Returns the safety confidence score (0.0–1.0).
    pub fn confidence(&self) -> f32 {
        self.state.safety.confidence
    }

    /// Returns the safety subsystem health score (0.0–1.0).
    pub fn health_score(&self) -> f32 {
        self.state.safety.health_score
    }
}
