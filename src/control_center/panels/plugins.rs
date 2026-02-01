/* ================================================================================================
   SYNTRAOS — PLUGINS PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/plugins.rs
   Module:      SyntraOS Control Center — Plugins Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Plugins subsystem:
         • Installed plugins
         • Active plugins
         • Plugin metadata
         • Plugin capabilities
         • Plugin permissions
         • Plugin health
         • Plugin logs
         • Update availability

       This panel is UI-agnostic. It prepares plugin data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
         • Extension ecosystems
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct PluginsPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> PluginsPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Installed & Active Plugins
    // --------------------------------------------------------------------------------------------

    /// Returns the list of installed plugins.
    pub fn installed(&self) -> &[String] {
        &self.state.plugins.installed
    }

    /// Returns the list of active plugins.
    pub fn active(&self) -> &[String] {
        &self.state.plugins.active
    }

    // --------------------------------------------------------------------------------------------
    //  Metadata
    // --------------------------------------------------------------------------------------------

    /// Returns metadata for a specific plugin.
    pub fn metadata(&self, name: &str) -> Option<&Vec<String>> {
        self.state.plugins.metadata.get(name)
    }

    /// Returns the version of a specific plugin.
    pub fn version(&self, name: &str) -> Option<String> {
        self.state.plugins.version.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Capabilities
    // --------------------------------------------------------------------------------------------

    /// Returns the capabilities of a specific plugin.
    pub fn capabilities(&self, name: &str) -> Option<&Vec<String>> {
        self.state.plugins.capabilities.get(name)
    }

    // --------------------------------------------------------------------------------------------
    //  Permissions
    // --------------------------------------------------------------------------------------------

    /// Returns the permissions granted to a specific plugin.
    pub fn permissions(&self, name: &str) -> Option<&Vec<String>> {
        self.state.plugins.permissions.get(name)
    }

    // --------------------------------------------------------------------------------------------
    //  Health
    // --------------------------------------------------------------------------------------------

    /// Returns the health score of a specific plugin (0.0–1.0).
    pub fn health(&self, name: &str) -> Option<f32> {
        self.state.plugins.health.get(name).cloned()
    }

    /// Returns the overall plugin subsystem health score.
    pub fn subsystem_health(&self) -> f32 {
        self.state.plugins.subsystem_health
    }

    // --------------------------------------------------------------------------------------------
    //  Logs
    // --------------------------------------------------------------------------------------------

    /// Returns the logs for a specific plugin.
    pub fn logs(&self, name: &str) -> Option<&Vec<String>> {
        self.state.plugins.logs.get(name)
    }

    /// Returns the last log entry for a specific plugin.
    pub fn last_log(&self, name: &str) -> Option<String> {
        self.state.plugins.last_log.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Updates
    // --------------------------------------------------------------------------------------------

    /// Returns whether an update is available for a specific plugin.
    pub fn update_available(&self, name: &str) -> Option<bool> {
        self.state.plugins.update_available.get(name).cloned()
    }

    /// Returns the list of plugins with updates available.
    pub fn plugins_with_updates(&self) -> &[String] {
        &self.state.plugins.plugins_with_updates
    }
}
