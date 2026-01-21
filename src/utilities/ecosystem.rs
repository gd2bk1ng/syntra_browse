/* ================================================================================================
   SYNTRA BROWSER - AXIOM ONE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/utilities/ecosystem.rs
   Module:      Utilities - Ecosystem Introspection
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides utilities for inspecting Syntra's filesystem ecosystem, including
                structural lobe detection and basic banner validation. Supports Axiom One
                self-analysis and diagnostic routines.

   Overview:
     • ExpectedLobe         - Describes a structural lobe Syntra expects to find.
     • scan_lobes           - Check presence of expected lobes under a repo root.
     • has_syntra_banner    - Detect Syntra's standard banner in a file.
     • find_files_missing_banner - Recursively find Rust files missing the banner.

   Notes:
     - Observational only in Axiom One (no self-modification).
     - Designed to integrate with AGI Core ecosystem models and diagnostics.
   ================================================================================================ */

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use crate::agi_core::EcosystemLobe;

/// Describes a structural lobe Syntra expects to find.
#[derive(Debug, Clone)]
pub struct ExpectedLobe {
    pub name: String,
    pub relative_path: String,
}

impl ExpectedLobe {
    pub fn new(name: &str, relative_path: &str) -> Self {
        Self {
            name: name.to_string(),
            relative_path: relative_path.to_string(),
        }
    }
}

/// Scan the filesystem for expected lobes and return their presence state.
pub fn scan_lobes(repo_root: &Path, expected: &[ExpectedLobe]) -> Vec<EcosystemLobe> {
    expected
        .iter()
        .map(|e| {
            let full = repo_root.join(&e.relative_path);
            EcosystemLobe {
                name: e.name.clone(),
                path: full.to_string_lossy().to_string(),
                present: full.exists(),
            }
        })
        .collect()
}

/// Perform a lightweight banner check on a file (does it contain the SYNTRA header marker?).
pub fn has_syntra_banner(path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(path) {
        content.contains("SYNTRA BROWSER - AXIOM")
    } else {
        false
    }
}

/// Recursively scan for Rust source files missing the Syntra banner.
pub fn find_files_missing_banner(repo_root: &Path) -> Vec<PathBuf> {
    let mut missing = Vec::new();
    visit_dirs(repo_root, &mut |entry| {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                if !has_syntra_banner(&path) {
                    missing.push(path);
                }
            }
        }
    });
    missing
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&fs::DirEntry)) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb);
            } else {
                cb(&entry);
            }
        }
    }
}
