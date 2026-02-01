/* ================================================================================================
   SYNTRAOS — AGENTS PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/agents.rs
   Module:      SyntraOS Control Center — Agents Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Agents subsystem:
         • Active agents
         • Agent roles
         • Agent tasks
         • Agent health
         • Confidence scores
         • Coordination messages
         • Lifecycle state (idle, running, paused, failed)
         • Agent capabilities

       This panel is UI-agnostic. It prepares agent data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct AgentsPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> AgentsPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Active Agents
    // --------------------------------------------------------------------------------------------

    /// Returns the list of active agent names.
    pub fn active_agents(&self) -> &[String] {
        &self.state.agents.active_agents
    }

    /// Returns the role of a specific agent.
    pub fn agent_role(&self, name: &str) -> Option<String> {
        self.state.agents.agent_role.get(name).cloned()
    }

    /// Returns the lifecycle state of a specific agent.
    pub fn agent_state(&self, name: &str) -> Option<String> {
        self.state.agents.agent_state.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Tasks
    // --------------------------------------------------------------------------------------------

    /// Returns the current task assigned to an agent.
    pub fn agent_task(&self, name: &str) -> Option<String> {
        self.state.agents.agent_task.get(name).cloned()
    }

    /// Returns the list of global agent tasks.
    pub fn global_tasks(&self) -> &[String] {
        &self.state.agents.global_tasks
    }

    // --------------------------------------------------------------------------------------------
    //  Health & Confidence
    // --------------------------------------------------------------------------------------------

    /// Returns the health score of a specific agent (0.0–1.0).
    pub fn agent_health(&self, name: &str) -> Option<f32> {
        self.state.agents.agent_health.get(name).cloned()
    }

    /// Returns the confidence score of a specific agent (0.0–1.0).
    pub fn agent_confidence(&self, name: &str) -> Option<f32> {
        self.state.agents.agent_confidence.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Coordination
    // --------------------------------------------------------------------------------------------

    /// Returns the list of coordination messages between agents.
    pub fn coordination_messages(&self) -> &[String] {
        &self.state.agents.coordination_messages
    }

    /// Returns the last message sent by a specific agent.
    pub fn agent_last_message(&self, name: &str) -> Option<String> {
        self.state.agents.agent_last_message.get(name).cloned()
    }

    // --------------------------------------------------------------------------------------------
    //  Capabilities
    // --------------------------------------------------------------------------------------------

    /// Returns the list of capabilities for a specific agent.
    pub fn agent_capabilities(&self, name: &str) -> Option<&Vec<String>> {
        self.state.agents.agent_capabilities.get(name)
    }
}
