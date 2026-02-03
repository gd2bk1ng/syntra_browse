// ================================================================================================
//   SYNTRA KERNEL — UTILITIES (HELPERS & SYSTEM TOOLS)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s'
//
//   File:        src/utilities/mod.rs
//   Module:      Utilities (Helpers & System Tools)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Aggregates Syntra's lightweight utility subsystems, including structured logging,
//       diagnostics, filesystem ecosystem introspection, tracing hooks, baseline snapshots,
//       and evolution logging. These tools form the backbone of Axiom Zero and Axiom One,
//       enabling self-analysis, observability, and safe self-modification.
//
//   Overview:
//       - logging           — Human-readable & JSON-style structured logging.
//       - diagnostics       — Diagnostic event bus for internal health signals.
//       - ecosystem         — Filesystem introspection utilities for self-awareness.
//       - tracing           — Lightweight tracing hooks for internal instrumentation.
//       - evolution_log     — Records Syntra’s self-modification events.
//       - baseline_snapshot — Syntra’s filesystem self-image for evolution & safety.
//
//   Notes:
//       - Dependency-minimal and ASCII-safe for long-term stability.
//       - Provides the shared API surface for all higher-level lobes.
//       - MIT & Apache 2.0 dual-licensed.
//       - Designed for future expansion as Syntra evolves.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

pub mod ecosystem;
pub mod logging;
pub mod diagnostics;
pub mod tracing;
pub mod evolution_log;
pub mod baseline_snapshot;

// ================================================================================================
// Re-exports — Common utilities exposed for convenience
// ================================================================================================

// Logging
pub use logging::{
    error,
    info,
    warn,
    LogLevel,
    log_human,
    log_json,
    timestamp,
};

// Diagnostics
pub use diagnostics::{
    DiagnosticBus,
    DiagnosticEvent,
    DiagnosticKind,
};

// Tracing
pub use tracing::{
    trace_event,
    trace_enter,
    trace_exit,
};

// Evolution Log
pub use evolution_log::{
    EvolutionRecord,
    write_evolution_record,
    compute_file_hash,
};

// Baseline Snapshot
pub use baseline_snapshot::{
    BaselineSnapshot,
    SnapshotDiff,
    FileEntry,
};
