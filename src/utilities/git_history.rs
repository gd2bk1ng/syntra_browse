// ================================================================================================
//   SYNTRA KERNEL — GIT HISTORY & CHURN ANALYZER
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/git_history.rs
//   Module:      Utilities — Git History & Churn Analyzer
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides a Git-aware view of Syntra’s evolution over time. This module is responsible for:
//
//         • Reading commit history for the current repository
//         • Computing real churn metrics (adds, deletes, modifications per file)
//         • Tracking authorship and contribution patterns
//         • Identifying hotspots (high-churn + high-complexity files)
//         • Exposing data for repo_sync and publish_policy
//
//       This is the temporal backbone of Syntra’s self-awareness: not just how she looks now,
//       but how she has changed, who changed her, and where the code is most volatile.
//
//   Notes:
//       - Designed to be implemented using a Git backend (e.g., `git2`) behind a thin abstraction.
//       - All operations that touch remote repositories must be explicitly human-gated.
//       - This module is read-only; pushing/pulling is handled by repo_sync.rs.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;

/// Summary of a single commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitSummary {
    pub id: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64,
    pub message: String,
    pub touched_files: Vec<String>,
}

/// Churn metrics for a single file across history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChurn {
    pub path: String,
    pub additions: usize,
    pub deletions: usize,
    pub modifications: usize,
    pub commits_touching: usize,
}

/// High-level history snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHistorySnapshot {
    pub commits: Vec<CommitSummary>,
    pub file_churn: Vec<FileChurn>,
}

/// Abstraction over the underlying Git backend.
///
/// Implementations can use `git2`, shelling out to `git`, or any other mechanism.
/// Syntra only depends on this trait, not on a specific Git library.
pub trait GitBackend {
    fn list_commits(&self, repo_root: &Path, max_commits: Option<usize>) -> anyhow::Result<Vec<CommitSummary>>;
    fn compute_file_churn(&self, repo_root: &Path) -> anyhow::Result<Vec<FileChurn>>;
}

/// Git history analyzer — orchestrates GitBackend to produce a history snapshot.
#[derive(Debug)]
pub struct GitHistoryAnalyzer<B: GitBackend> {
    backend: B,
}

impl<B: GitBackend> GitHistoryAnalyzer<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Build a full history snapshot (commits + churn).
    pub fn snapshot(
        &self,
        repo_root: &Path,
        max_commits: Option<usize>,
    ) -> anyhow::Result<GitHistorySnapshot> {
        let commits = self.backend.list_commits(repo_root, max_commits)?;
        let file_churn = self.backend.compute_file_churn(repo_root)?;

        Ok(GitHistorySnapshot { commits, file_churn })
    }

    /// Compute a simple hotspot map: file path → churn score.
    pub fn hotspot_scores(
        &self,
        snapshot: &GitHistorySnapshot,
    ) -> HashMap<String, f32> {
        let mut scores = HashMap::new();

        for fc in &snapshot.file_churn {
            let score = (fc.additions + fc.deletions + fc.modifications) as f32
                * (fc.commits_touching as f32);
            scores.insert(fc.path.clone(), score);
        }

        scores
    }
}

/// A no-op backend placeholder for environments without Git.
///
/// This allows Syntra to compile and run even when no Git integration is available.
/// Real deployments can provide a concrete backend implementation.
#[derive(Debug, Default)]
pub struct NoopGitBackend;

impl GitBackend for NoopGitBackend {
    fn list_commits(&self, _repo_root: &Path, _max_commits: Option<usize>) -> anyhow::Result<Vec<CommitSummary>> {
        Ok(Vec::new())
    }

    fn compute_file_churn(&self, _repo_root: &Path) -> anyhow::Result<Vec<FileChurn>> {
        Ok(Vec::new())
    }
}

