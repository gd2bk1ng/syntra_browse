/* ================================================================================================
   SYNTRAOS — PREDICTIVE PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/predictive.rs
   Module:      SyntraOS Control Center — Predictive Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Predictive subsystem:
         • Forecasted events
         • Prediction windows
         • Confidence scores
         • Trend analysis
         • Anomaly detection
         • Future state projections
         • Risk forecasts
         • Predictive model health

       This panel is UI-agnostic. It prepares predictive data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct PredictivePanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> PredictivePanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Forecasts
    // --------------------------------------------------------------------------------------------

    /// Returns the list of predicted future events.
    pub fn forecasts(&self) -> &[String] {
        &self.state.predictive.forecasts
    }

    /// Returns the prediction window (e.g., "5s", "30s", "2m", "1h").
    pub fn window(&self) -> Option<String> {
        self.state.predictive.window.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Confidence
    // --------------------------------------------------------------------------------------------

    /// Returns the average prediction confidence score (0.0–1.0).
    pub fn confidence(&self) -> f32 {
        self.state.predictive.confidence
    }

    /// Returns the confidence score for a specific forecast index.
    pub fn confidence_for(&self, index: usize) -> Option<f32> {
        self.state.predictive.forecast_confidence.get(index).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Trends
    // --------------------------------------------------------------------------------------------

    /// Returns the list of detected trends.
    pub fn trends(&self) -> &[String] {
        &self.state.predictive.trends
    }

    /// Returns the dominant trend (if any).
    pub fn dominant_trend(&self) -> Option<String> {
        self.state.predictive.dominant_trend.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Anomalies
    // --------------------------------------------------------------------------------------------

    /// Returns the list of detected anomalies.
    pub fn anomalies(&self) -> &[String] {
        &self.state.predictive.anomalies
    }

    /// Returns the anomaly severity score (0.0–1.0).
    pub fn anomaly_severity(&self) -> f32 {
        self.state.predictive.anomaly_severity
    }

    // --------------------------------------------------------------------------------------------
    //  Future State Projection
    // --------------------------------------------------------------------------------------------

    /// Returns the projected future system state (serialized).
    pub fn projected_state(&self) -> Option<String> {
        self.state.predictive.projected_state.clone()
    }

    /// Returns the timestamp of the last predictive update.
    pub fn last_update(&self) -> Option<String> {
        self.state.predictive.last_update.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Health
    // --------------------------------------------------------------------------------------------

    /// Returns the predictive subsystem health score (0.0–1.0).
    pub fn health_score(&self) -> f32 {
        self.state.predictive.health_score
    }
}
