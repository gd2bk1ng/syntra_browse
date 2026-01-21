/* ================================================================================================
   SYNTRA BROWSER - AXIOM ONE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/utilities/diagnostics.rs
   Module:      Utilities - Diagnostic Event Bus
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a lightweight diagnostic event bus for broadcasting internal health
                signals across Syntra's subsystems. Events can be consumed by telemetry lobes,
                loggers, or future monitoring tools.

   Overview:
     • DiagnosticKind  - Types of diagnostic events (health, warning, anomaly, info).
     • DiagnosticEvent - Structured diagnostic payload.
     • DiagnosticBus   - Simple MPSC-based event bus.

   Notes:
     - Observational only in Axiom One (no self-modification).
     - Designed to integrate with logging and telemetry without external dependencies.
   ================================================================================================ */

#![allow(dead_code)]

use std::sync::mpsc::{channel, Receiver, Sender};

use crate::utilities::logging::{log_json, LogLevel};

/// Diagnostic event kinds.
#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    HealthCheck,
    Warning,
    Anomaly,
    Info,
}

/// A diagnostic event emitted by Syntra.
#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    pub kind: DiagnosticKind,
    pub source: String,
    pub message: String,
}

impl DiagnosticEvent {
    /// Emit this event as a JSON-style log line.
    pub fn emit_json(&self) {
        let kind_str = match self.kind {
            DiagnosticKind::HealthCheck => "health_check",
            DiagnosticKind::Warning => "warning",
            DiagnosticKind::Anomaly => "anomaly",
            DiagnosticKind::Info => "info",
        };
        let context = format!("source={},kind={}", self.source, kind_str);
        let level = match self.kind {
            DiagnosticKind::Warning | DiagnosticKind::Anomaly => LogLevel::Warn,
            _ => LogLevel::Info,
        };
        log_json(level, &self.message, &context);
    }
}

/// A simple diagnostic bus for broadcasting events.
#[derive(Debug)]
pub struct DiagnosticBus {
    pub tx: Sender<DiagnosticEvent>,
    pub rx: Receiver<DiagnosticEvent>,
}

impl DiagnosticBus {
    /// Create a new diagnostic bus.
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { tx, rx }
    }

    /// Emit an event onto the bus.
    pub fn emit(&self, event: DiagnosticEvent) {
        let _ = self.tx.send(event);
    }

    /// Attempt to receive an event without blocking.
    pub fn try_recv(&self) -> Option<DiagnosticEvent> {
        self.rx.try_recv().ok()
    }
}
