/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/cortex/mod.rs
   Module:      Cortex (Cognitive Orchestration Layer)
   Author:      Alexandr Roussinov
   Description: High‑level cognitive coordinator. The Cortex integrates reasoning engines,
                interprets intents, and dispatches actions across Syntra’s subsystems.

   Overview:
     • Cortex<R>     — Generic orchestrator over any Reasoner implementation.
     • handle_intent — Converts raw input into structured intent and refines it.
     • pump_messages — Processes conduit messages and logs them.

   Notes:
     The Cortex is the conductor of Syntra’s cognitive orchestra. Keep orchestration logic clean,
     transparent, and traceable.
   ================================================================================================ */

#![allow(dead_code)]

use crate::agi_core::{Intent, Reasoner, NullReasoner};
use crate::conduit::{Conduit, ConduitMessage};
use crate::utilities::log_info;

/// The Cortex orchestrates high‑level system behavior, routing intents and coordinating
/// subsystems such as the renderer and AGI core.
pub struct Cortex<R: Reasoner = NullReasoner> {
    pub reasoner: R,
    pub conduit: Conduit,
}

impl<R: Reasoner> Cortex<R> {
    pub fn new(reasoner: R, conduit: Conduit) -> Self {
        Self { reasoner, conduit }
    }

    /// Processes an incoming raw intent string and dispatches the refined result.
    pub fn handle_intent(&self, raw: &str) {
        log_info(&format!("Cortex received raw intent: {}", raw));

        let intent = Intent {
            label: raw.to_string(),
            confidence: 0.9,
        };

        let refined = self.reasoner.process(intent);

        log_info(&format!(
            "Cortex refined intent: {} (confidence: {:.2})",
            refined.label, refined.confidence
        ));

        self.conduit.send(ConduitMessage::Intent(refined.label));
    }

    /// Polls the conduit for messages and logs them.
    pub fn pump_messages(&self) {
        while let Some(msg) = self.conduit.try_recv() {
            match msg {
                ConduitMessage::Log(text) => log_info(&format!("[Conduit] {}", text)),
                ConduitMessage::Intent(label) => {
                    log_info(&format!("[Conduit] Intent dispatched: {}", label))
                }
                ConduitMessage::Shutdown => {
                    log_info("[Conduit] Shutdown signal received.");
                    break;
                }
            }
        }
    }
}
