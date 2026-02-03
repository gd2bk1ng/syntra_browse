// ================================================================================================
//   SYNTRA KERNEL — SEMANTIC FILESYSTEM VIEW
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/semantic_fs.rs
//   Module:      Utilities — Semantic Filesystem View
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides a semantic layer over Syntra's filesystem. Instead of just paths, Syntra can
//       reason in terms of roles and regions:
//
//         • Cortex / AGI Core
//         • Lobes (ecosystem, providers, planners, etc.)
//         • Utilities (logging, diagnostics, snapshots)
//         • Tests, experiments, sandboxes
//
//       This is used for:
//         • Targeted evolution (e.g., "refactor only lobes").
//         • Safety policies by region.
//         • Richer introspection and reporting.
//
//   Notes:
//       - Built on top of BaselineSnapshot and path_utils.
//       - Designed to be extended with more roles and regions over time.
// ================================================================================================

#![allow(dead_code)]

use crate::utilities::baseline_snapshot::FileEntry;
use crate::utilities::BaselineSnapshot;
use crate::utilities::path_utils::{normalize_path, is_within};

use serde::{Serialize, Deserialize};
use std::path::Path;

/// High-level semantic role of a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SemanticRole {
    AgiCore,
    Lobe,
    Utility,
    Test,
    Sandbox,
    Config,
    Unknown,
}

/// A semantic classification for a single file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFile {
    pub path: String,
    pub role: SemanticRole,
}

/// A semantic view over the filesystem snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFsView {
    pub files: Vec<SemanticFile>,
}

impl SemanticFsView {
    pub fn from_snapshot(root: &Path, snapshot: &BaselineSnapshot) -> Self {
        let root_norm = normalize_path(root);
        let mut files = Vec::new();

        for entry in snapshot.files.values() {
            let role = classify_role(&root_norm, &entry.path);
            files.push(SemanticFile {
                path: entry.path.clone(),
                role,
            });
        }

        Self { files }
    }

    pub fn by_role(&self, role: SemanticRole) -> Vec<&SemanticFile> {
        self.files.iter().filter(|f| f.role == role).collect()
    }
}

/// Simple heuristic-based classifier.
/// This can be made more sophisticated as Syntra evolves.
fn classify_role(root_norm: &str, rel_path: &str) -> SemanticRole {
    let full = format!("{}/{}", root_norm, rel_path);
    let full_path = Path::new(&full);

    let agi_core = Path::new("src/agi_core");
    let utilities = Path::new("src/utilities");
    let tests = Path::new("tests");
    let sandbox = Path::new("sandbox");
    let config = Path::new("config");

    if is_within(full_path, agi_core) {
        SemanticRole::AgiCore
    } else if is_within(full_path, utilities) {
        SemanticRole::Utility
    } else if is_within(full_path, tests) {
        SemanticRole::Test
    } else if is_within(full_path, sandbox) {
        SemanticRole::Sandbox
    } else if is_within(full_path, config) {
        SemanticRole::Config
    } else {
        // Future: detect lobes by naming conventions or metadata.
        SemanticRole::Unknown
    }
}

