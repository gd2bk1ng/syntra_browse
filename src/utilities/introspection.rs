// ================================================================================================
//   SYNTRA KERNEL — INTROSPECTION HUB
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/introspection.rs
//   Module:      Utilities — Introspection Hub
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides a unified API for Syntra to introspect her own state:
//
//         • Baseline filesystem snapshot
//         • Semantic filesystem view (roles/regions)
//         • Dependency graph (internal + external)
//         • Code index (symbols)
//
//       This is the "self-awareness" utility layer that higher-level lobes can query to
//       understand Syntra's current shape before proposing evolution plans.
//
//   Notes:
//       - Non-mutating; read-only view over the current state.
//       - Designed to be cheap to query and easy to extend.
// ================================================================================================

#![allow(dead_code)]

use crate::utilities::{
    BaselineSnapshot,
    SemanticFsView,
    DependencyGraph,
    CodeIndex,
    fs_utils,
    path_utils::normalize_path,
};

use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

/// Aggregated introspection snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectionSnapshot {
    pub root: String,
    pub baseline: BaselineSnapshot,
    pub semantic_fs: SemanticFsView,
    pub dependency_graph: DependencyGraph,
    pub code_index: CodeIndex,
}

/// Introspection hub — orchestrates gathering of self-awareness data.
#[derive(Debug)]
pub struct IntrospectionHub {
    root: PathBuf,
}

impl IntrospectionHub {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Build a full introspection snapshot.
    ///
    /// Future:
    ///   - Wire dependency_graph from cargo metadata.
    ///   - Build code_index from BaselineSnapshot + fs_utils.
    pub fn snapshot(&self) -> anyhow::Result<IntrospectionSnapshot> {
        let root_norm = normalize_path(&self.root);

        // 1) Baseline snapshot
        let policy_stub = crate::agi_core::SelfModPolicy::default_for_introspection();
        let baseline = BaselineSnapshot::scan_tree(&self.root, &policy_stub)?;

        // 2) Semantic FS view
        let semantic_fs = SemanticFsView::from_snapshot(&self.root, &baseline);

        // 3) Dependency graph (placeholder: empty for now, ready to be filled)
        let dependency_graph = DependencyGraph::new();

        // 4) Code index (placeholder: scan all .rs files via fs_utils)
        let mut files = Vec::new();
        for (rel, entry) in &baseline.files {
            if rel.ends_with(".rs") {
                let full = self.root.join(rel);
                if let Ok(text) = fs_utils::read_text_file(&full) {
                    files.push((rel.clone(), text));
                }
            }
        }
        let indexer = crate::utilities::code_index::CodeIndexer::new();
        let code_index = indexer.build_index(files);

        Ok(IntrospectionSnapshot {
            root: root_norm,
            baseline,
            semantic_fs,
            dependency_graph,
            code_index,
        })
    }
}

