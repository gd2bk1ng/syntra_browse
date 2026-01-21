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
   Description: Shared helper functions, lightweight logging utilities, and cross-module tools
                used throughout the Syntra architecture. This module provides foundational
                building blocks that higher-level lobes rely on for diagnostics, timestamps,
                and ecosystem introspection.

   Overview:
     • timestamp   - RFC3339 UTC timestamp generator.
     • log_info    - Pretty info-level logger.
     • log_warn    - Pretty warning logger.
     • log_error   - Pretty error logger.
     • ecosystem   - Filesystem introspection utilities for self-analysis.

   Notes:
     - Utilities should remain lightweight and dependency-minimal for long-term stability.
     - Logging helpers are intentionally simple and ASCII-safe for terminal and CI output.
     - The ecosystem module supports Axiom One self-analysis routines.
   ================================================================================================ */

#![allow(dead_code)]

pub mod ecosystem;

use chrono::{DateTime, Utc};

/// Returns the current UTC timestamp as a formatted RFC3339 string.
pub fn timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/// Pretty logging helper for informational messages.
pub fn log_info(msg: &str) {
    println!("[INFO {}] {}", timestamp(), msg);
}

/// Pretty logging helper for warnings.
pub fn log_warn(msg: &str) {
    println!("[WARN {}] {}", timestamp(), msg);
}

/// Pretty logging helper for errors.
pub fn log_error(msg: &str) {
    eprintln!("[ERROR {}] {}", timestamp(), msg);
}
