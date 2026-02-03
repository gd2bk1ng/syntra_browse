// ================================================================================================
//   SYNTRA KERNEL — BASELINE SNAPSHOT
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/baseline_snapshot.rs
//   Module:      Utilities — Baseline Snapshot Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides Syntra with a complete, structured snapshot of her filesystem. This snapshot
//       represents her “self‑image” — a record of what files exist, their hashes, sizes, and
//       modification permissions according to the self‑mod policy. Used for:
//         • Self‑modification safety (Axiom Six)
//         • Evolution tracking and diffing
//         • Dependency and integrity checks
//         • Cognitive introspection and self‑awareness
//
//   Overview:
//       - BaselineSnapshot: top‑level structure containing all file entries.
//       - FileEntry: metadata for each file (hash, size, mode).
//       - scan_tree(): recursively scans Syntra’s filesystem.
//       - compare(): computes diffs between snapshots.
//       - Integration with SelfModPolicy for forbidden/propose/self_mod classification.
//
//   Notes:
//       - ASCII‑safe, dependency‑minimal, stable.
//       - Designed for long‑term evolution and introspection.
//       - MIT & Apache 2.0 dual‑licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual‑licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::self_mod_policy::{SelfModPolicy, SelfModMode};
use crate::utilities::evolution_log::compute_file_hash;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a single file in Syntra’s filesystem snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub hash: String,
    pub mode: SelfModMode,
}

/// Represents Syntra’s entire filesystem snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineSnapshot {
    pub files: HashMap<String, FileEntry>,
}

impl BaselineSnapshot {
    /// Creates an empty snapshot.
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Scans the filesystem from the given root and builds a complete snapshot.
    pub fn scan_tree(root: &Path, policy: &SelfModPolicy) -> anyhow::Result<Self> {
        let mut snapshot = BaselineSnapshot::new();
        let mut stack = vec![root.to_path_buf()];

        while let Some(path) = stack.pop() {
            if path.is_dir() {
                for entry in fs::read_dir(&path)? {
                    let entry = entry?;
                    stack.push(entry.path());
                }
            } else if path.is_file() {
                let rel = path.strip_prefix(root).unwrap_or(&path);
                let rel_str = rel.to_string_lossy().replace('\\', "/");

                let metadata = fs::metadata(&path)?;
                let size = metadata.len();
                let hash = compute_file_hash(&path)?;

                let mode = policy.mode_for(rel);

                snapshot.files.insert(
                    rel_str.clone(),
                    FileEntry {
                        path: rel_str,
                        size,
                        hash,
                        mode,
                    },
                );
            }
        }

        Ok(snapshot)
    }

    /// Computes differences between two snapshots.
    pub fn compare(&self, other: &BaselineSnapshot) -> SnapshotDiff {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut modified = Vec::new();

        for (path, entry) in &other.files {
            if !self.files.contains_key(path) {
                added.push(entry.clone());
            }
        }

        for (path, entry) in &self.files {
            if !other.files.contains_key(path) {
                removed.push(entry.clone());
            }
        }

        for (path, old_entry) in &self.files {
            if let Some(new_entry) = other.files.get(path) {
                if old_entry.hash != new_entry.hash {
                    modified.push((old_entry.clone(), new_entry.clone()));
                }
            }
        }

        SnapshotDiff {
            added,
            removed,
            modified,
        }
    }
}

/// Represents differences between two snapshots.
#[derive(Debug)]
pub struct SnapshotDiff {
    pub added: Vec<FileEntry>,
    pub removed: Vec<FileEntry>,
    pub modified: Vec<(FileEntry, FileEntry)>,
}
