/* ================================================================================================
   SYNTRA BROWSER - AXIOM ONE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/utilities/tracing.rs
   Module:      Utilities - Tracing Hooks
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides simple tracing hooks for instrumenting Syntra's internal operations.
                In Axiom One, this is a lightweight wrapper around structured logging, designed
                to be easily replaced by a full tracing framework in future axioms.

   Overview:
     • trace_event - Emit a named trace event with context.
     • trace_enter - Mark function entry.
     • trace_exit  - Mark function exit.

   Notes:
     - All traces are ASCII-safe and single-line.
     - Intended for low-friction instrumentation of critical paths.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::logging::{log_json, LogLevel};

/// Emit a trace event with a name and context string.
pub fn trace_event(name: &str, context: &str) {
    let msg = format!("trace:{}", name);
    log_json(LogLevel::Info, &msg, context);
}

/// Convenience wrapper for tracing function entry.
pub fn trace_enter(fn_name: &str) {
    trace_event("enter", fn_name);
}

/// Convenience wrapper for tracing function exit.
pub fn trace_exit(fn_name: &str) {
    trace_event("exit", fn_name);
}
