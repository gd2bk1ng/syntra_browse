/* ================================================================================================
   SYNTRAOS — EVOLUTION PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/evolution.rs
   Module:      SyntraOS Control Center — Evolution Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to SyntraOS evolution state:
         • Version lineage
         • Capability growth
         • Module evolution
         • Changelogs
         • Upgrade recommendations
         • Experimental features
         • Evolutionary score / maturity index

       This panel is UI-agnostic. It prepares evolution data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct EvolutionPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> EvolutionPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Versioning & Lineage
    // --------------------------------------------------------------------------------------------

    /// Returns the current SyntraOS version string.
    pub fn version(&self) -> Option<String> {
        self.state.evolution.version.clone()
    }

    /// Returns the lineage chain (e.g., ["Axiom One", "Axiom Two", "Axiom Three"]).
    pub fn lineage(&self) -> &[String] {
        &self.state.evolution.lineage
    }

    // --------------------------------------------------------------------------------------------
    //  Capability Growth
    // --------------------------------------------------------------------------------------------

    /// Returns the capability growth score (0.0–1.0).
    pub fn capability_growth(&self) -> f32 {
        self.state.evolution.capability_growth_score
    }

    /// Returns the list of newly added capabilities.
    pub fn new_capabilities(&self) -> &[String] {
        &self.state.evolution.new_capabilities
    }

    // --------------------------------------------------------------------------------------------
    //  Module Evolution
    // --------------------------------------------------------------------------------------------

    /// Returns the list of modules that evolved in the last update.
    pub fn evolved_modules(&self) -> &[String] {
        &self.state.evolution.evolved_modules
    }

    /// Returns the list of modules scheduled for future evolution.
    pub fn upcoming_evolution(&self) -> &[String] {
        &self.state.evolution.upcoming_evolution
    }

    // --------------------------------------------------------------------------------------------
    //  Changelogs
    // --------------------------------------------------------------------------------------------

    /// Returns the changelog entries for the current version.
    pub fn changelog(&self) -> &[String] {
        &self.state.evolution.changelog
    }

    // --------------------------------------------------------------------------------------------
    //  Experimental Features
    // --------------------------------------------------------------------------------------------

    /// Returns a list of experimental features.
    pub fn experimental_features(&self) -> &[String] {
        &self.state.evolution.experimental_features
    }

    /// Returns whether experimental mode is enabled.
    pub fn experimental_mode_enabled(&self) -> bool {
        self.state.evolution.experimental_mode
    }

    // --------------------------------------------------------------------------------------------
    //  Evolution Score
    // --------------------------------------------------------------------------------------------

    /// Returns the overall evolutionary maturity score (0.0–1.0).
    pub fn maturity_score(&self) -> f32 {
        self.state.evolution.maturity_score
    }
}
