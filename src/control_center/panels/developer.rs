/* ================================================================================================
   SYNTRAOS — DEVELOPER PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/developer.rs
   Module:      SyntraOS Control Center — Developer Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       The most advanced and customizable panel in SyntraOS.

       Provides structured access to developer‑facing internals:
         • Build metadata
         • Feature flags
         • Debug channels
         • Profiling data
         • Hot‑reload state
         • Module registry
         • Experimental toggles
         • Developer logs
         • Custom KV store
         • Dynamic configuration
         • Internal hooks
         • Sandbox state

       This panel is UI‑agnostic. It prepares developer data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
         • DevTools
         • Plugin ecosystems
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;
use std::collections::HashMap;

pub struct DeveloperPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> DeveloperPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Build Metadata
    // --------------------------------------------------------------------------------------------

    /// Returns the SyntraOS build ID.
    pub fn build_id(&self) -> Option<String> {
        self.state.developer.build_id.clone()
    }

    /// Returns the build timestamp.
    pub fn build_timestamp(&self) -> Option<String> {
        self.state.developer.build_timestamp.clone()
    }

    /// Returns the compiler version used for this build.
    pub fn compiler_version(&self) -> Option<String> {
        self.state.developer.compiler_version.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Feature Flags
    // --------------------------------------------------------------------------------------------

    /// Returns all feature flags and their states.
    pub fn feature_flags(&self) -> &HashMap<String, bool> {
        &self.state.developer.feature_flags
    }

    /// Returns whether a specific feature flag is enabled.
    pub fn feature_enabled(&self, flag: &str) -> Option<bool> {
        self.state.developer.feature_flags.get(flag).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Debug Channels
    // --------------------------------------------------------------------------------------------

    /// Returns the list of active debug channels.
    pub fn debug_channels(&self) -> &[String] {
        &self.state.developer.debug_channels
    }

    /// Returns the last message from a specific debug channel.
    pub fn debug_last(&self, channel: &str) -> Option<String> {
        self.state.developer.debug_last.get(channel).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Profiling
    // --------------------------------------------------------------------------------------------

    /// Returns the current CPU profiler snapshot.
    pub fn cpu_profile(&self) -> Option<String> {
        self.state.developer.cpu_profile.clone()
    }

    /// Returns the current memory profiler snapshot.
    pub fn memory_profile(&self) -> Option<String> {
        self.state.developer.memory_profile.clone()
    }

    /// Returns the list of performance hotspots.
    pub fn hotspots(&self) -> &[String] {
        &self.state.developer.hotspots
    }

    // --------------------------------------------------------------------------------------------
    //  Hot Reload
    // --------------------------------------------------------------------------------------------

    /// Returns whether hot‑reload is enabled.
    pub fn hot_reload_enabled(&self) -> bool {
        self.state.developer.hot_reload_enabled
    }

    /// Returns the list of modules reloaded in the last cycle.
    pub fn hot_reload_log(&self) -> &[String] {
        &self.state.developer.hot_reload_log
    }

    // --------------------------------------------------------------------------------------------
    //  Module Registry
    // --------------------------------------------------------------------------------------------

    /// Returns the list of registered modules.
    pub fn modules(&self) -> &[String] {
        &self.state.developer.modules
    }

    /// Returns metadata for a specific module.
    pub fn module_metadata(&self, name: &str) -> Option<&Vec<String>> {
        self.state.developer.module_metadata.get(name)
    }

    // --------------------------------------------------------------------------------------------
    //  Experimental Toggles
    // --------------------------------------------------------------------------------------------

    /// Returns the list of experimental toggles.
    pub fn experimental_toggles(&self) -> &[String] {
        &self.state.developer.experimental_toggles
    }

    /// Returns whether a specific experimental toggle is active.
    pub fn experimental_active(&self, name: &str) -> Option<bool> {
        self.state.developer.experimental_active.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Developer Logs
    // --------------------------------------------------------------------------------------------

    /// Returns the developer log entries.
    pub fn logs(&self) -> &[String] {
        &self.state.developer.logs
    }

    /// Returns the last developer log entry.
    pub fn last_log(&self) -> Option<String> {
        self.state.developer.last_log.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Custom Key/Value Store
    // --------------------------------------------------------------------------------------------

    /// Returns the entire custom KV store.
    pub fn kv(&self) -> &HashMap<String, String> {
        &self.state.developer.kv
    }

    /// Returns a specific KV entry.
    pub fn kv_get(&self, key: &str) -> Option<String> {
        self.state.developer.kv.get(key).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Dynamic Configuration
    // --------------------------------------------------------------------------------------------

    /// Returns the dynamic configuration map.
    pub fn config(&self) -> &HashMap<String, String> {
        &self.state.developer.config
    }

    /// Returns a specific config value.
    pub fn config_get(&self, key: &str) -> Option<String> {
        self.state.developer.config.get(key).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Internal Hooks
    // --------------------------------------------------------------------------------------------

    /// Returns the list of internal hooks.
    pub fn hooks(&self) -> &[String] {
        &self.state.developer.hooks
    }

    /// Returns the last hook triggered.
    pub fn last_hook(&self) -> Option<String> {
        self.state.developer.last_hook.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Sandbox
    // --------------------------------------------------------------------------------------------

    /// Returns the sandbox state (serialized).
    pub fn sandbox_state(&self) -> Option<String> {
        self.state.developer.sandbox_state.clone()
    }

    /// Returns sandbox warnings.
    pub fn sandbox_warnings(&self) -> &[String] {
        &self.state.developer.sandbox_warnings
    }
}
