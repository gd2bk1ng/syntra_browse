/* ================================================================================================
   SYNTRAOS — SIMULATION PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/simulation.rs
   Module:      SyntraOS Control Center — Simulation Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Simulation subsystem:
         • Simulation scenarios
         • Physics models
         • Agent behavior models
         • Environment states
         • Simulation results
         • Simulation logs
         • Last run metadata
         • Simulation subsystem health

       This panel is UI-agnostic. It prepares simulation data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
         • Research & testing environments
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct SimulationPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> SimulationPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Scenarios
    // --------------------------------------------------------------------------------------------

    /// Returns the list of available simulation scenarios.
    pub fn scenarios(&self) -> &[String] {
        &self.state.simulation.scenarios
    }

    /// Returns the currently active simulation scenario.
    pub fn active_scenario(&self) -> Option<String> {
        self.state.simulation.active_scenario.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Models
    // --------------------------------------------------------------------------------------------

    /// Returns the physics model currently in use.
    pub fn physics_model(&self) -> Option<String> {
        self.state.simulation.physics_model.clone()
    }

    /// Returns the agent behavior model currently in use.
    pub fn behavior_model(&self) -> Option<String> {
        self.state.simulation.behavior_model.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Environment
    // --------------------------------------------------------------------------------------------

    /// Returns the serialized environment state.
    pub fn environment_state(&self) -> Option<String> {
        self.state.simulation.environment_state.clone()
    }

    /// Returns the list of environment variables.
    pub fn environment_variables(&self) -> &[String] {
        &self.state.simulation.environment_variables
    }

    // --------------------------------------------------------------------------------------------
    //  Results
    // --------------------------------------------------------------------------------------------

    /// Returns the results of the last simulation run.
    pub fn results(&self) -> &[String] {
        &self.state.simulation.results
    }

    /// Returns the summary of the last simulation run.
    pub fn summary(&self) -> Option<String> {
        self.state.simulation.summary.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Logs
    // --------------------------------------------------------------------------------------------

    /// Returns the simulation log entries.
    pub fn logs(&self) -> &[String] {
        &self.state.simulation.logs
    }

    /// Returns the last log entry.
    pub fn last_log(&self) -> Option<String> {
        self.state.simulation.last_log.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Metadata
    // --------------------------------------------------------------------------------------------

    /// Returns the timestamp of the last simulation run.
    pub fn last_run(&self) -> Option<String> {
        self.state.simulation.last_run.clone()
    }

    /// Returns the duration of the last simulation run (in ms).
    pub fn last_run_duration_ms(&self) -> Option<u64> {
        self.state.simulation.last_run_duration_ms
    }

    // --------------------------------------------------------------------------------------------
    //  Health
    // --------------------------------------------------------------------------------------------

    /// Returns the simulation subsystem health score (0.0–1.0).
    pub fn health_score(&self) -> f32 {
        self.state.simulation.health_score
    }
}
