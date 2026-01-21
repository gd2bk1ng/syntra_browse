/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/utilities/logging.rs
   Module:      Utilities - Logging Facade
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a lightweight, structured logging facade for Syntra. Supports both
                human-readable and JSON-style logs, suitable for terminals and CI pipelines.

   Notes:
     - Designed to remain dependency-minimal and ASCII-safe.
     - Can be wired into future telemetry and tracing systems.
   ================================================================================================ */

#![allow(dead_code)]

use chrono::{DateTime, Utc};

/// Log severity levels.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// Returns the current UTC timestamp as a formatted RFC3339 string.
pub fn timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/// Emit a human-readable log line.
pub fn log_human(level: LogLevel, msg: &str) {
    let ts = timestamp();
    let level_str = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    };
    match level {
        LogLevel::Error => eprintln!("[{} {}] {}", level_str, ts, msg),
        _ => println!("[{} {}] {}", level_str, ts, msg),
    }
}

/// Emit a JSON-style log line (single-line, ASCII-safe).
pub fn log_json(level: LogLevel, msg: &str, context: &str) {
    let ts = timestamp();
    let level_str = match level {
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
    };
    println!(
        "{{\"level\":\"{}\",\"timestamp\":\"{}\",\"message\":\"{}\",\"context\":\"{}\"}}",
        level_str,
        ts,
        escape_json(msg),
        escape_json(context)
    );
}

/// Minimal JSON escaping for log messages.
fn escape_json(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

/// Convenience helpers.
pub fn info(msg: &str) {
    log_human(LogLevel::Info, msg);
}

pub fn warn(msg: &str) {
    log_human(LogLevel::Warn, msg);
}

pub fn error(msg: &str) {
    log_human(LogLevel::Error, msg);
}
