/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/conduit/bridge.rs
   Module:      Conduit - Intent Bridge
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a stable, ASCII-safe interface between external tools (such as the
                Syntra Terminal) and Syntra's AGI Core intent semantics. This bridge converts
                freeform human input into structured, machine-readable intent packets.

   Overview:
     • IntentResponse — Structured output containing the raw intent, its classification, and
                        a high-level plan.
     • process_intent — Converts freeform text into a structured IntentResponse.
     • escape_json    — Ensures safe ASCII output for external tools and terminals.

   Notes:
     - This module forms the contract boundary between Syntra’s shell environments and her
       cognitive subsystems.
     - All output is ASCII-safe to ensure compatibility with PowerShell, Bash, and external
       automation tools.
     - The intent bridge is intentionally deterministic and dependency-free for long-term
       stability and reproducibility.
     - Future expansions may include richer metadata, confidence scores, or multi-step plans.
   ================================================================================================ */

use crate::agi_core::{classify_intent, escape_json, plan_for_intent};

use crate::agi_core::intent::{classify_intent, escape_json, plan_for_intent};

/// Represents a structured response from the intent bridge.
#[derive(Debug, Clone)]
pub struct IntentResponse {
    /// The raw intent text provided by the user or external system.
    pub intent: String,

    /// The semantic class assigned by the AGI Core (e.g., diagnostic, planning, navigation).
    pub class: String,

    /// A high-level plan or interpretation associated with the intent.
    pub plan: String,
}

impl IntentResponse {
    /// Converts the response into a single-line ASCII-safe JSON object.
    pub fn to_json_line(&self) -> String {
        format!(
            "{{\"intent\":\"{}\",\"class\":\"{}\",\"plan\":\"{}\"}}",
            escape_json(&self.intent),
            escape_json(&self.class),
            escape_json(&self.plan)
        )
    }
}

/// Processes a freeform intent string into a structured response.
///
/// This function:
///   1. Normalizes and trims the input.
///   2. Classifies the intent using Syntra's AGI Core.
///   3. Generates a high-level plan.
///   4. Returns a structured IntentResponse suitable for external tools.
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
