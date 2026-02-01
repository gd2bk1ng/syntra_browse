/* ================================================================================================
   SYNTRAOS — ASSISTANT PANEL
   ------------------------------------------------------------------------------------------------
         .\s/.
        :: S ::
         '/s\'

   File:        src/control_center/panels/assistant.rs
   Module:      SyntraOS Control Center — Assistant Panel
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description:
       Provides structured access to the SyntraOS Assistant subsystem:
         • Assistant mode (chat, command, reasoning)
         • Active conversation history
         • Last user message
         • Last assistant response
         • Intent routing
         • Confidence scores
         • Suggested actions
         • Assistant status (idle, thinking, responding)

       This panel is UI-agnostic. It prepares assistant data for:
         • SyntraOS Shell (desktop)
         • Browser UI
         • Robot HUD
         • AR overlays
         • Console UI
   ================================================================================================ */

use crate::control_center::state::ControlCenterState;

pub struct AssistantPanel<'a> {
    pub state: &'a ControlCenterState,
}

impl<'a> AssistantPanel<'a> {
    pub fn new(state: &'a ControlCenterState) -> Self {
        Self { state }
    }

    // --------------------------------------------------------------------------------------------
    //  Mode & Status
    // --------------------------------------------------------------------------------------------

    /// Returns the current assistant mode (e.g., "chat", "command", "reasoning").
    pub fn mode(&self) -> Option<String> {
        self.state.assistant.mode.clone()
    }

    /// Returns the assistant's current status (e.g., "idle", "thinking", "responding").
    pub fn status(&self) -> Option<String> {
        self.state.assistant.status.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Conversation
    // --------------------------------------------------------------------------------------------

    /// Returns the full conversation history.
    pub fn conversation(&self) -> &[String] {
        &self.state.assistant.conversation
    }

    /// Returns the last user message.
    pub fn last_user_message(&self) -> Option<String> {
        self.state.assistant.last_user_message.clone()
    }

    /// Returns the last assistant response.
    pub fn last_assistant_response(&self) -> Option<String> {
        self.state.assistant.last_assistant_response.clone()
    }

    // --------------------------------------------------------------------------------------------
    //  Intent Routing
    // --------------------------------------------------------------------------------------------

    /// Returns the last detected intent.
    pub fn last_intent(&self) -> Option<String> {
        self.state.assistant.last_intent.clone()
    }

    /// Returns the routing confidence score (0.0–1.0).
    pub fn routing_confidence(&self) -> f32 {
        self.state.assistant.routing_confidence
    }

    // --------------------------------------------------------------------------------------------
    //  Suggestions
    // --------------------------------------------------------------------------------------------

    /// Returns assistant-suggested actions or follow-ups.
    pub fn suggestions(&self) -> &[String] {
        &self.state.assistant.suggestions
    }
}
