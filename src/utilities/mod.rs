/* ================================================================================================
   Syntra Browser — Axiom Zero
   Advanced AGI-Driven Intent Engine & Cognitive Rendering System
   ------------------------------------------------------------------------------------------------
   File:        src/utilities/mod.rs
   Module:      Utilities (Helpers & System Tools)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Created:     2026
   License:     MIT
   Repository:  https://github.com/gd2bk1ng/syntra_browse
   ------------------------------------------------------------------------------------------------
   Overview:
   The Utilities module provides shared helper functions, logging tools, time utilities, and other
   cross-cutting concerns used throughout the Syntra architecture.

   Notes for Future Engineers (2050+):
   - Keep utilities lightweight and dependency-free.
   - Avoid letting this module become a dumping ground.
   - Every utility should have a clear, single responsibility.
   ================================================================================================ */

#![allow(dead_code)]

use chrono::{DateTime, Utc};

/// Returns the current UTC timestamp as a formatted string.
pub fn timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/// Pretty logging helper.
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
