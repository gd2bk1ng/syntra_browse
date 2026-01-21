// ================================================================================================
// SYNTRA BROWSER - AXIOM ZERO
// ------------------------------------------------------------------------------------------------
// SIGIL:
//       .\s/.
//      :: S ::
//       '/s\'
//
// File:        src/conduit/bridge.rs
// Module:      Syntra Conduit - Intent Bridge
// Author:      Alexandr Roussinov (gd2bk1ng)
// Description: Provides a stable, ASCII-safe interface between external tools (such as the
//              Syntra Terminal) and the AGI core's intent semantics.
//
// Notes:
//   - This is the contract layer between shell environments and Syntra's cognition.
//   - Designed to be stable and well-documented for long-term ecosystem adoption.
// ================================================================================================

use crate::agi_core::intent::{classify_intent, escape_json, plan_for_intent};

/// Represents a structured response from the intent bridge.
pub struct IntentResponse {
    pub intent: String,
    pub class: String,
    pub plan: String,
}

impl IntentResponse {
    pub fn to_json_line(&self) -> String {
        format!(
            "{{\"intent\":\"{}\",\"class\":\"{}\",\"plan\":\"{}\"}}",
            escape_json(&self.intent),
            escape_json(&self.class),
            escape_json(&self.plan)
        )
    }
}

/// Process a freeform intent string into a structured response.
pub fn process_intent(intent_text: &str) -> IntentResponse {
    let trimmed = intent_text.trim();
    let classification = classify_intent(trimmed);
    let plan = plan_for_intent(&classification, trimmed);

    IntentResponse {
        intent: trimmed.to_string(),
        class: classification,
        plan,
    }
}
