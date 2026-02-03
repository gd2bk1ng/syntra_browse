/* ================================================================================================
   SYNTRA BROWSER - AXIOM ONE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/utilities/logging.rs
   Module:      Utilities - Logging Facade (Advanced)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Enhanced structured logging for Syntra. Supports human-readable and JSON logs,
                cognitive trace IDs, channels, tags, and optional file logging.

   Overview:
     • LogLevel       - Debug, Info, Warn, Error.
     • LogRecord      - Structured log entry.
     • log()          - Unified structured logger.
     • log_human      - Human-readable output.
     • log_json       - JSON-style output.
     • log_info       - Cognitive info-level helper.
     • quiet mode     - Optional global toggle for silent operation.

   Notes:
     - ASCII-safe, dependency-minimal, stable.
     - Forms the backbone of Syntra’s introspection and telemetry.
   ================================================================================================ */

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use std::fs::OpenOptions;
use std::io::Write;

/// Global quiet mode toggle.
static QUIET_MODE: AtomicBool = AtomicBool::new(false);

/// Optional log file path.
static LOG_FILE: OnceLock<String> = OnceLock::new();

/// Log severity levels.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Structured log record.
pub struct LogRecord<'a> {
    pub level: LogLevel,
    pub message: &'a str,
    pub channel: &'a str,
    pub tag: &'a str,
    pub trace_id: Option<&'a str>,
}

/// Enable or disable quiet mode.
pub fn set_quiet_mode(enabled: bool) {
    QUIET_MODE.store(enabled, Ordering::Relaxed);
}

/// Set a file path for persistent logging.
pub fn set_log_file(path: &str) {
    let _ = LOG_FILE.set(path.to_string());
}

/// Returns the current UTC timestamp as RFC3339.
pub fn timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.to_rfc3339()
}

/// Unified structured logger.
pub fn log(record: LogRecord) {
    if QUIET_MODE.load(Ordering::Relaxed) && record.level == LogLevel::Debug {
        return;
    }

    log_human(&record);
    log_file(&record);
}

/// Human-readable log output.
pub fn log_human(record: &LogRecord) {
    let ts = timestamp();
    let level_str = match record.level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    };

    let trace = record.trace_id.unwrap_or("-");
    let line = format!(
        "[{} {}] [{}:{}] {}",
        level_str, ts, record.channel, record.tag, record.message
    );

    match record.level {
        LogLevel::Error => eprintln!("{}", line),
        _ => println!("{}", line),
    }
}

/// Optional file logging.
fn log_file(record: &LogRecord) {
    if let Some(path) = LOG_FILE.get() {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(
                file,
                "[{}] [{}:{}] {}",
                timestamp(),
                record.channel,
                record.tag,
                record.message
            );
        }
    }
}

/// JSON-style log output.
pub fn log_json(level: LogLevel, msg: &str, context: &str) {
    let ts = timestamp();
    let level_str = match level {
        LogLevel::Debug => "debug",
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

/// Minimal JSON escaping.
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
pub fn debug(msg: &str) {
    log(LogRecord {
        level: LogLevel::Debug,
        message: msg,
        channel: "general",
        tag: "debug",
        trace_id: None,
    });
}

pub fn info(msg: &str) {
    log(LogRecord {
        level: LogLevel::Info,
        message: msg,
        channel: "general",
        tag: "info",
        trace_id: None,
    });
}

/// Syntra-style cognitive info log.
pub fn log_info(msg: &str) {
    log(LogRecord {
        level: LogLevel::Info,
        message: msg,
        channel: "cortex",
        tag: "cognition",
        trace_id: None,
    });
}

pub fn warn(msg: &str) {
    log(LogRecord {
        level: LogLevel::Warn,
        message: msg,
        channel: "general",
        tag: "warn",
        trace_id: None,
    });
}

pub fn error(msg: &str) {
    log(LogRecord {
        level: LogLevel::Error,
        message: msg,
        channel: "general",
        tag: "error",
        trace_id: None,
    });
}
