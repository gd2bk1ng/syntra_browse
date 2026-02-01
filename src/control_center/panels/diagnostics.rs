/* ================================================================================================
   SYNTRAOS — DIAGNOSTICS PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/diagnostics.rs
   Module:      SyntraOS Control Center — Diagnostics Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to diagnostics state:
         • System health
         • Kernel health
         • Provider health
         • Last diagnostics run
         • Error logs
         • Warnings
         • Integrity checks
         • Performance anomalies

       This panel is UI-agnostic. It prepares diagnostics data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct DiagnosticsPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> DiagnosticsPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Health Metrics
    // --------------------------------------------------------------------------------------------

    /// Returns the overall system health score (0.0–1.0).
    pub fn system_health(&self) -> f32 {
        self.state.diagnostics.system_health_score
    }

    /// Returns the kernel health score (0.0–1.0).
    pub fn kernel_health(&self) -> f32 {
        self.state.diagnostics.kernel_health_score
    }

    /// Returns the provider health score (0.0–1.0).
    pub fn provider_health(&self) -> f32 {
        self.state.diagnostics.provider_health_score
    }

    // --------------------------------------------------------------------------------------------
    //  Diagnostics Runs
    // --------------------------------------------------------------------------------------------

    /// Returns the timestamp of the last diagnostics run.
    pub fn last_run(&self) -> Option<String> {
        self.state.diagnostics.last_run.clone()
    }

    /// Returns whether the last diagnostics run passed.
    pub fn last_run_passed(&self) -> bool {
        self.state.diagnostics.last_run_passed
    }

    // --------------------------------------------------------------------------------------------
    //  Logs
    // --------------------------------------------------------------------------------------------

    /// Returns a list of recent error logs.
    pub fn error_logs(&self) -> &[String] {
        &self.state.diagnostics.error_logs
    }

    /// Returns a list of recent warnings.
    pub fn warnings(&self) -> &[String] {
        &self.state.diagnostics.warnings
    }

    // --------------------------------------------------------------------------------------------
    //  Integrity
    // --------------------------------------------------------------------------------------------

    /// Returns whether the OS integrity check passed.
    pub fn integrity_ok(&self) -> bool {
        self.state.diagnostics.integrity_ok
    }

    /// Returns a list of integrity violations (if any).
    pub fn integrity_violations(&self) -> &[String] {
        &self.state.diagnostics.integrity_violations
    }

    // --------------------------------------------------------------------------------------------
    //  Performance
    // --------------------------------------------------------------------------------------------

    /// Returns a list of performance anomalies detected.
    pub fn performance_anomalies(&self) -> &[String] {
        &self.state.diagnostics.performance_anomalies
    }
}
