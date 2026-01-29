// ================================================================================================
//   SYNTRA KERNEL — AXIOM THREE (COGNITIVE AWARENESS / TELEMETRY)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/telemetry.rs
//   Module:      Telemetry & Cognitive Awareness
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Defines telemetry events and a lightweight telemetry bus for observing routing,
//       safety decisions, and evolution behavior across the kernel.
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::safety::SafetyVerdict;
use crate::agi_core::schema::plan_schema::RoutePlan;

/// Telemetry events emitted by the kernel.
#[derive(Debug, Clone)]
pub enum TelemetryEvent {
    /// Routing decision for an intent.
    Routing {
        target: String,
        confidence: f32,
        reason: String,
    },
    /// Safety evaluation result for a proposal.
    SafetyDecision {
        verdict: SafetyVerdict,
    },
    /// High-level evolution summary (e.g., number of proposals, allowed vs blocked).
    EvolutionSummary {
        total: usize,
        allowed: usize,
        blocked: usize,
    },
    /// Generic log message.
    Log {
        level: TelemetryLevel,
        message: String,
    },
}

/// Severity level for telemetry logs.
#[derive(Debug, Clone, Copy)]
pub enum TelemetryLevel {
    Info,
    Warn,
    Error,
}

/// Simple telemetry sink trait.
///
/// In the future this can be extended to support multiple backends
/// (console, file, remote collector, etc.).
pub trait TelemetrySink: Send + Sync {
    fn record(&self, event: TelemetryEvent);
}

/// A basic in-memory / console telemetry sink.
#[derive(Debug, Default)]
pub struct ConsoleTelemetrySink;

impl TelemetrySink for ConsoleTelemetrySink {
    fn record(&self, event: TelemetryEvent) {
        // For now, just print to stdout. This can be replaced with a more
        // sophisticated logging / metrics pipeline later.
        println!("[TELEMETRY] {:?}", event);
    }
}

/// Telemetry bus: central point for emitting events.
#[derive(Debug)]
pub struct TelemetryBus {
    sink: Box<dyn TelemetrySink>,
}

impl TelemetryBus {
    pub fn new(sink: Box<dyn TelemetrySink>) -> Self {
        Self { sink }
    }

    pub fn with_console_sink() -> Self {
        Self {
            sink: Box::new(ConsoleTelemetrySink::default()),
        }
    }

    pub fn record(&self, event: TelemetryEvent) {
        self.sink.record(event);
    }

    /// Convenience helper for routing telemetry.
    pub fn record_routing(&self, plan: &RoutePlan) {
        self.record(TelemetryEvent::Routing {
            target: plan.target_lobe.clone(),
            confidence: plan.confidence,
            reason: plan.reason.clone(),
        });
    }

    /// Convenience helper for safety telemetry.
    pub fn record_safety_verdict(&self, verdict: SafetyVerdict) {
        self.record(TelemetryEvent::SafetyDecision { verdict });
    }

    /// Convenience helper for evolution summary.
    pub fn record_evolution_summary(&self, total: usize, allowed: usize, blocked: usize) {
        self.record(TelemetryEvent::EvolutionSummary {
            total,
            allowed,
            blocked,
        });
    }

    /// Convenience helper for generic logs.
    pub fn log(&self, level: TelemetryLevel, message: impl Into<String>) {
        self.record(TelemetryEvent::Log {
            level,
            message: message.into(),
        });
    }
}
