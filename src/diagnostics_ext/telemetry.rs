// ================================================================================================
//   SYNTRA KERNEL — DIAGNOSTICS TELEMETRY
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/diagnostics_ext/telemetry.rs
//   Module:      Diagnostics — Telemetry Bus
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Structured telemetry events for the Syntra kernel. This module provides a
//                lightweight publish/subscribe bus for runtime events, suitable for logging,
//                dashboards, and external observability tools.
// ================================================================================================

use std::sync::{Arc, Mutex};

/// Severity level for telemetry events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// A structured telemetry event.
#[derive(Debug, Clone)]
pub struct TelemetryEvent {
    pub level: TelemetryLevel,
    pub component: String,
    pub message: String,
    pub context: Option<String>,
}

/// Subscriber callback type.
pub type TelemetrySubscriber = Box<dyn Fn(&TelemetryEvent) + Send + Sync + 'static>;

/// Telemetry bus with in-process subscribers.
#[derive(Default, Clone)]
pub struct TelemetryBus {
    inner: Arc<Mutex<Vec<TelemetrySubscriber>>>,
}

impl TelemetryBus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscribe to telemetry events.
    pub fn subscribe<F>(&self, f: F)
    where
        F: Fn(&TelemetryEvent) + Send + Sync + 'static,
    {
        self.inner.lock().expect("telemetry bus poisoned").push(Box::new(f));
    }

    /// Emit a telemetry event to all subscribers.
    pub fn emit(&self, event: TelemetryEvent) {
        for sub in self.inner.lock().expect("telemetry bus poisoned").iter() {
            sub(&event);
        }
    }

    /// Convenience helper for emitting a simple message.
    pub fn log(
        &self,
        level: TelemetryLevel,
        component: impl Into<String>,
        message: impl Into<String>,
        context: Option<String>,
    ) {
        let event = TelemetryEvent {
            level,
            component: component.into(),
            message: message.into(),
            context,
        };
        self.emit(event);
    }
}
