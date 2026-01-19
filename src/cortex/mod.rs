/* ================================================================================================
   Syntra Browser — Axiom Zero
   Advanced AGI-Driven Intent Engine & Cognitive Rendering System
   ------------------------------------------------------------------------------------------------
   File:        src/cortex/mod.rs
   Module:      Cortex (Cognitive Orchestration Layer)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Created:     2026
   License:     MIT
   Repository:  https://github.com/gd2bk1ng/syntra_browse
   ------------------------------------------------------------------------------------------------
   Overview:
   The Cortex module orchestrates Syntra’s high-level cognitive behavior. It integrates reasoning
   engines, interprets intents, and coordinates actions across the renderer, AGI core, and utilities.
   This is the executive control center of the system.

   Notes for Future Engineers (2050+):
   - Maintain separation between perception, reasoning, and action.
   - The Cortex should remain the conductor, not the orchestra.
   - Keep the orchestration logic transparent and traceable.
   ================================================================================================ */

#![allow(dead_code)]

use crate::agi_core::{Intent, Reasoner, NullReasoner};
use crate::conduit::{Conduit, ConduitMessage};
use crate::utilities::log_info;

/// The Cortex orchestrates high-level system behavior, routing intents and coordinating
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

        self.conduit
            .send(ConduitMessage::Intent(refined.label.clone()));
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
