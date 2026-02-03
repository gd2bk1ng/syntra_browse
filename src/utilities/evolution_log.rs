/* ================================================================================================
   SYNTRA KERNEL — EVOLUTION LOG
   ------------------------------------------------------------------------------------------------
        .\s/.
       :: S ::
        '/s'

   File:        src/utilities/evolution_log.rs
   Module:      Utilities - Evolution Log
   Description: Provides structured logging for Syntra’s self-modification events. Records changes
                to files, including timestamps, old/new hashes, reasons, and human approvals.

   Notes:
     - Designed for long-term traceability and introspection.
     - Writes entries in JSONL format for easy parsing and analysis.
     - Integrates with Syntra’s self-modification and safety systems.
   ================================================================================================ */

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;
use sha2::{Sha256, Digest};

/// Represents a single evolution event in Syntra’s lifecycle.
#[derive(Debug, Serialize)]
pub struct EvolutionRecord {
    pub timestamp: String,
    pub file_path: String,
    pub change_type: String,
    pub old_hash: Option<String>,
    pub new_hash: Option<String>,
    pub reason: String,
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
        Self {
            timestamp: Utc::now().to_rfc3339(),
            file_path: file_path.to_string(),
            change_type: change_type.to_string(),
            old_hash,
            new_hash,
            reason: reason.to_string(),
            approved_by: approved_by.to_string(),
        }
    }
}

/// Writes an evolution record to the JSONL log file.
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

/// Computes a SHA-256 hash of a file’s contents.
pub fn compute_file_hash(path: &Path) -> anyhow::Result<String> {
    let data = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    Ok(format!("{:x}", hasher.finalize()))
}
