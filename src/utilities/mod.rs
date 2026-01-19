/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/utilities/mod.rs
   Module:      Utilities (Helpers & System Tools)
   Author:      Alexandr Roussinov
   Description: Shared helper functions, logging utilities, and cross‑module tools used throughout
                the Syntra architecture.

   Overview:
     • timestamp  — RFC3339 UTC timestamp generator.
     • log_info   — Pretty info‑level logger.
     • log_warn   — Pretty warning logger.
     • log_error  — Pretty error logger.

   Notes:
     Keep utilities lightweight and dependency‑free where possible.
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
