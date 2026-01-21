/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/utilities/mod.rs
   Module:      Utilities (Helpers & System Tools)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Aggregates Syntra's lightweight utility subsystems, including structured logging,
                diagnostics, filesystem ecosystem introspection, and tracing hooks. These tools
                form the backbone of Axiom One's self-analysis and observability capabilities.

   Overview:
     • logging      - Human-readable and JSON-style structured logging.
     • diagnostics  - Diagnostic event bus for internal health signals.
     • ecosystem    - Filesystem introspection utilities for self-analysis.
     • tracing      - Lightweight tracing hooks for instrumenting internal operations.

   Notes:
     - Utilities remain dependency-minimal and ASCII-safe for long-term stability.
     - This module provides the shared API surface for all higher-level lobes.
     - Axiom One introduces structured diagnostics and tracing for deeper introspection.
   ================================================================================================ */

#![allow(dead_code)]

pub mod ecosystem;
pub mod logging;
pub mod diagnostics;
pub mod tracing;

// Re-export commonly used utilities for convenience.
pub use logging::{
    error,
    info,
    warn,
    LogLevel,
    log_human,
    log_json,
    timestamp,
};

pub use diagnostics::{
    DiagnosticBus,
    DiagnosticEvent,
    DiagnosticKind,
};

pub use tracing::{
    trace_event,
    trace_enter,
    trace_exit,
};
