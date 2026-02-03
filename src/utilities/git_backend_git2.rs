// ================================================================================================
//   SYNTRA KERNEL — GIT BACKEND (git2 IMPLEMENTATION)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/git_backend_git2.rs
//   Module:      Utilities — Git Backend (git2 Implementation)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Concrete implementation of the GitBackend trait using the `git2` crate. This backend
//       provides real Git history, churn metrics, commit metadata, and file-level diffs.
//
//       Capabilities:
//         • Enumerate commits
//         • Extract authorship information
//         • Compute additions/deletions/modifications per file
//         • Detect renames and file moves
//         • Provide data to RepoSyncEngine and PublishPolicy
//
//       Safety:
//         • This module is READ-ONLY. It never pushes, pulls, or mutates the repo.
//         • All write operations are handled by RepoSyncEngine and require human approval.
//
//   Notes:
//       - Requires the `git2` crate.
//       - Designed to be swappable with other backends (e.g., libgit, JGit).
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use git2::{Repository, DiffOptions, Oid};
use serde::{Serialize, Deserialize};
use std::path::Path;

use crate::utilities::{
    GitBackend,
    CommitSummary,
    FileChurn,
};

/// Git backend implemented using the `git2` crate.
#[derive(Debug, Default)]
pub struct GitBackendGit2;

impl GitBackend for GitBackendGit2 {
    fn list_commits(
        &self,
        repo_root: &Path,
        max_commits: Option<usize>,
    ) -> anyhow::Result<Vec<CommitSummary>> {
        let repo = Repository::open(repo_root)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = Vec::new();

        for (i, oid_result) in revwalk.enumerate() {
            if let Some(limit) = max_commits {
                if i >= limit {
                    break;
                }
            }

            let oid = oid_result?;
            let commit = repo.find_commit(oid)?;

            let author = commit.author();
            let message = commit.message().unwrap_or("").to_string();

            // Collect touched files
            let mut touched_files = Vec::new();
            if commit.parent_count() > 0 {
                let parent = commit.parent(0)?;
                let tree = commit.tree()?;
                let parent_tree = parent.tree()?;

                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
                diff.foreach(
                    &mut |delta, _| {
                        if let Some(path) = delta.new_file().path() {
                            touched_files.push(path.to_string_lossy().to_string());
                        }
                        true
                    },
                    None,
                    None,
                    None,
                )?;
            }

            commits.push(CommitSummary {
                id: oid.to_string(),
                author_name: author.name().unwrap_or("unknown").to_string(),
                author_email: author.email().unwrap_or("unknown").to_string(),
                timestamp: commit.time().seconds(),
                message,
                touched_files,
            });
        }

        Ok(commits)
    }

    fn compute_file_churn(
        &self,
        repo_root: &Path,
    ) -> anyhow::Result<Vec<FileChurn>> {
        let repo = Repository::open(repo_root)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let mut churn_map: std::collections::HashMap<String, FileChurn> = std::collections::HashMap::new();

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = repo.find_commit(oid)?;

            if commit.parent_count() == 0 {
                continue;
            }

            let parent = commit.parent(0)?;
            let tree = commit.tree()?;
            let parent_tree = parent.tree()?;

            let mut diff_opts = DiffOptions::new();
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_opts))?;

            diff.foreach(
                &mut |delta, _| {
                    let path = delta
                        .new_file()
                        .path()
                        .or_else(|| delta.old_file().path())
                        .map(|p| p.to_string_lossy().to_string());

                    if let Some(path) = path {
                        let entry = churn_map.entry(path.clone()).or_insert(FileChurn {
                            path: path.clone(),
                            additions: 0,
                            deletions: 0,
                            modifications: 0,
                            commits_touching: 0,
                        });

                        entry.commits_touching += 1;
                    }

                    true
                },
                Some(&mut |file, hunk| {
                    if let Some(path) = file.new_file().path().or(file.old_file().path()) {
                        let path = path.to_string_lossy().to_string();
                        let entry = churn_map.entry(path.clone()).or_insert(FileChurn {
                            path: path.clone(),
                            additions: 0,
                            deletions: 0,
                            modifications: 0,
                            commits_touching: 0,
                        });

                        entry.modifications += 1;
                    }
                    true
                }),
                Some(&mut |file, _hunk, line| {
                    if let Some(path) = file.new_file().path().or(file.old_file().path()) {
                        let path = path.to_string_lossy().to_string();
                        let entry = churn_map.entry(path.clone()).or_insert(FileChurn {
                            path: path.clone(),
                            additions: 0,
                            deletions: 0,
                            modifications: 0,
                            commits_touching: 0,
                        });

                        match line.origin() {
                            '+' => entry.additions += 1,
                            '-' => entry.deletions += 1,
                            _ => {}
                        }
                    }
                    true
                }),
                None,
            )?;
        }

        Ok(churn_map.into_values().collect())
    }
}

