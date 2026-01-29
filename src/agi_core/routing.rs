// ================================================================================================
//   SYNTRA KERNEL — AXIOM FOUR (COGNITIVE ROUTING)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/routing.rs
//   Module:      Cognitive Routing Layer
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Routes intents to the appropriate cognitive lobes. Includes arbitration, scoring,
//       fallback routing, safety hooks, and telemetry emission.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::schema::intent_schema::Intent;
use crate::agi_core::schema::plan_schema::RoutePlan;
use crate::agi_core::telemetry::{TelemetryBus, TelemetryEvent, TelemetryLevel};
use crate::agi_core::safety::SafetyGate;

/// Routing decision for a single intent.
#[derive(Debug, Clone)]
pub struct RouteDecision {
    pub target_lobe: String,
    pub confidence: f32,
    pub reason: String,
}

/// Main router for Syntra Kernel.
#[derive(Debug)]
pub struct Router {
    pub safety_gate: SafetyGate,
    pub telemetry: TelemetryBus,
}

impl Router {
    pub fn new(safety_gate: SafetyGate, telemetry: TelemetryBus) -> Self {
        Self { safety_gate, telemetry }
    }

    /// Route an intent to the appropriate lobe.
    pub fn route_intent(&self, intent: &Intent) -> RouteDecision {
        // Basic heuristic routing (placeholder for now).
        let (target, reason) = match intent.kind.as_str() {
            "analysis" => ("reasoner", "Analytical intent → Reasoner"),
            "plan" => ("planner", "Planning intent → Planner"),
            "classify" => ("classifier", "Classification intent → Classifier"),
            "feedback" => ("feedback", "Feedback intent → Feedback Engine"),
            _ => ("reasoner", "Unknown intent → fallback to Reasoner"),
        };

        let decision = RouteDecision {
            target_lobe: target.into(),
            confidence: 0.85,
            reason: reason.into(),
        };

        // Emit routing telemetry.
        self.telemetry.record(TelemetryEvent::Routing {
            target: decision.target_lobe.clone(),
            confidence: decision.confidence,
            reason: decision.reason.clone(),
        });

        decision
    }

    /// Build a route plan for execution.
    pub fn build_route_plan(&self, intent: Intent) -> RoutePlan {
        let decision = self.route_intent(&intent);

        RoutePlan {
            intent,
            target_lobe: decision.target_lobe,
            confidence: decision.confidence,
            reason: decision.reason,
        }
    }

    /// Convenience helper to log routing anomalies.
    pub fn log_routing_anomaly(&self, message: impl Into<String>) {
        self.telemetry
            .log(TelemetryLevel::Warn, message.into());
    }
}
