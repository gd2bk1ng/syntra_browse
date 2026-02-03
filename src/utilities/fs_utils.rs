// ================================================================================================
//   SYNTRA KERNEL — FILESYSTEM UTILITIES
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/fs_utils.rs
//   Module:      Utilities — Filesystem Primitives
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides safe, policy-aware filesystem utilities for Syntra’s AGI Kernel. These functions
//       form the low-level “hands” Syntra uses to read, write, and manipulate her own codebase,
//       sandboxes, and artifacts.
//
//   Overview:
//       - read_text_file(): safe UTF‑8 file reads.
//       - write_text_file_atomic(): atomic text writes with temp file + rename.
//       - ensure_dir(): create directories recursively if missing.
//       - copy_tree(): recursive directory copy (for sandboxes).
//       - remove_tree(): recursive directory removal (for sandboxes).
//       - is_path_allowed(): policy-aware check before mutation.
//
//   Notes:
//       - All mutating operations are designed to be used *through* SelfModPolicy.
//       - ASCII‑safe, cross‑platform, stable.
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
use crate::utilities::path_utils::{normalize_path, canonicalize_lossy};

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Read a UTF‑8 text file into a String.
///
/// Returns an error if the file cannot be read or decoded.
pub fn read_text_file(path: &Path) -> anyhow::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

/// Write a UTF‑8 text file atomically.
///
/// Writes to a temporary file in the same directory, then renames it into place.
/// This avoids partially written files on crash or interruption.
pub fn write_text_file_atomic(path: &Path, contents: &str) -> anyhow::Result<()> {
    let parent = path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    ensure_dir(&parent)?;

    let tmp_path = parent.join(format!(
        ".syntra_tmp_{}.tmp",
        uuid::Uuid::new_v4().to_string()
    ));

    {
        let mut tmp_file = fs::File::create(&tmp_path)?;
        tmp_file.write_all(contents.as_bytes())?;
        tmp_file.sync_all()?;
    }

    fs::rename(&tmp_path, path)?;
    Ok(())
}

/// Ensure a directory exists, creating it (and parents) if necessary.
pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Recursively copy a directory tree from `src` to `dst`.
///
/// Existing files may be overwritten. Directories are created as needed.
pub fn copy_tree(src: &Path, dst: &Path) -> anyhow::Result<()> {
    ensure_dir(dst)?;

    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();

        let rel = path.strip_prefix(src).unwrap_or(path);
        let target = dst.join(rel);

        if entry.file_type().is_dir() {
            ensure_dir(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                ensure_dir(parent)?;
            }
            fs::copy(path, &target)?;
        }
    }

    Ok(())
}

/// Recursively remove a directory tree.
///
/// Intended primarily for sandbox cleanup.
pub fn remove_tree(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// Check if a path is allowed to be mutated under the given policy.
///
/// This function canonicalizes and normalizes the path, then consults the policy.
/// It returns:
///   - true  if mode is SelfMod
///   - false otherwise
pub fn is_path_allowed(path: &Path, policy: &SelfModPolicy) -> bool {
    let canon = canonicalize_lossy(path);
    let norm = normalize_path(&canon);
    policy.mode_for(norm) == SelfModMode::SelfMod
}
