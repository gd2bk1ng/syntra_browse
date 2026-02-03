// ================================================================================================
//   SYNTRA KERNEL — EVOLUTION LOG
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/evolution_log.rs
//   Module:      Utilities — Evolution Log
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides structured logging for Syntra’s self‑modification and evolution events.
//       Records changes to files, including timestamps, old/new hashes, reasons, and
//       human approvals. This forms Syntra’s long‑term “evolution diary” and enables
//       introspection, auditing, and research into her growth over time.
//
//   Overview:
//       - EvolutionRecord          — single evolution event (who/what/why/when).
//       - write_evolution_record   — append JSONL entry to artifacts/evolution_log.jsonl.
//       - compute_file_hash        — SHA‑256 hash of a file’s contents.
//       - parse_timestamp          — parse RFC3339 timestamps into DateTime<Utc>.
//       - human_time_delta         — human‑friendly “time since event” formatting.
//
//   Notes:
//       - JSONL format for easy streaming and offline analysis.
//       - ASCII‑safe and dependency‑minimal by design.
//       - MIT & Apache 2.0 dual‑licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Represents a single evolution event in Syntra’s lifecycle.
#[derive(Debug, Serialize)]
pub struct EvolutionRecord {
    /// RFC 3339 timestamp of the event.
    pub timestamp: String,

    /// Parsed timestamp (not serialized) for internal use.
    #[serde(skip_serializing)]
    pub timestamp_parsed: DateTime<Utc>,

    /// Path of the file affected (relative to repo root).
    pub file_path: String,

    /// High-level change type (e.g., "AddFile", "ModifyFile", "DeleteFile").
    pub change_type: String,

    /// Optional old hash of the file (before change).
    pub old_hash: Option<String>,

    /// Optional new hash of the file (after change).
    pub new_hash: Option<String>,

    /// Human-readable reason or narrative for the change.
    pub reason: String,

    /// Who approved or initiated the change (e.g., "developer", "operator").
    pub approved_by: String,
}

impl EvolutionRecord {
    /// Creates a new evolution record with the current timestamp.
    pub fn new(
        file_path: &str,
        change_type: &str,
        old_hash: Option<String>,
        new_hash: Option<String>,
        reason: &str,
        approved_by: &str,
    ) -> Self {
        let now = Utc::now();
        Self {
            timestamp: now.to_rfc3339(),
            timestamp_parsed: now,
            file_path: file_path.to_string(),
            change_type: change_type.to_string(),
            old_hash,
            new_hash,
            reason: reason.to_string(),
            approved_by: approved_by.to_string(),
        }
    }

    /// Returns a human-friendly description of how long ago this event occurred.
    pub fn age_human(&self) -> String {
        human_time_delta(self.timestamp_parsed, Utc::now())
    }
}

/// Writes an evolution record to the JSONL log file.
///
/// The log is stored at:
///     artifacts/evolution_log.jsonl
///
/// Each line is a single JSON object representing one EvolutionRecord.
pub fn write_evolution_record(record: &EvolutionRecord) -> anyhow::Result<()> {
    let log_path = Path::new("artifacts/evolution_log.jsonl");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let json = serde_json::to_string(record)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

/// Computes a SHA‑256 hash of a file’s contents.
///
/// Returns the hex‑encoded digest as a lowercase string.
pub fn compute_file_hash(path: &Path) -> anyhow::Result<String> {
    let data = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(format!("{:x}", hasher.finalize()))
}

/// Parse an RFC3339 timestamp into a DateTime<Utc>.
pub fn parse_timestamp(ts: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(ts)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

/// Produce a human-friendly time delta like:
///   "just now", "3 minutes ago", "2 hours ago", "yesterday", "5 days ago"
pub fn human_time_delta(start: DateTime<Utc>, end: DateTime<Utc>) -> String {
    let delta = end - start;

    if delta < Duration::seconds(30) {
        return "just now".into();
    }
    if delta < Duration::minutes(1) {
        return format!("{} seconds ago", delta.num_seconds());
    }
    if delta < Duration::hours(1) {
        return format!("{} minutes ago", delta.num_minutes());
    }
    if delta < Duration::hours(24) {
        return format!("{} hours ago", delta.num_hours());
    }
    if delta < Duration::days(2) {
        return "yesterday".into();
    }

    format!("{} days ago", delta.num_days())
}
