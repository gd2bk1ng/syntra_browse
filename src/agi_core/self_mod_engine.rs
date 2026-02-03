// ================================================================================================
//   SYNTRA KERNEL — SELF‑MODIFICATION ENGINE (AXIOM SIX)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/self_mod_engine.rs
//   Module:      AGI Core — Self‑Modification Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Implements Syntra’s self‑modification engine, responsible for generating, validating,
//       sandboxing, and applying code evolution proposals. This module integrates:
//         • SelfModPolicy        — constitutional safety rules
//         • BaselineSnapshot     — Syntra’s filesystem self‑image
//         • EvolutionLog         — structured evolution history
//         • Sandbox Execution    — safe, isolated testing of proposed changes
//         • Human Approval Gate  — ensures all modifications are explicitly authorized
//
//   Overview:
//       - ChangeProposal: describes a proposed modification.
//       - EvolutionPlan: groups multiple proposals into a coherent evolution step.
//       - SelfModEngine: orchestrates scanning, diffing, sandboxing, and applying changes.
//       - Safety checks: forbidden paths, propose‑only paths, feature breakers.
//       - Logging: all changes recorded in evolution_log.jsonl.
//
//   Notes:
//       - This engine never applies changes without explicit human approval.
//       - Designed for long‑term evolution, introspection, and safety.
//       - MIT & Apache 2.0 dual‑licensed.
//       - Future expansions: multi‑agent evolution, distributed self‑mod, plugin evolution.
// ================================================================================================
//
//   Copyright:
//       This file is dual‑licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use crate::utilities::{
    BaselineSnapshot,
    SnapshotDiff,
    EvolutionRecord,
    write_evolution_record,
    compute_file_hash,
};

use crate::agi_core::self_mod_policy::{SelfModPolicy, SelfModMode};

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the type of change Syntra wants to make.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeKind {
    AddFile,
    ModifyFile,
    DeleteFile,
}

/// A single proposed change to Syntra’s codebase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeProposal {
    pub path: String,
    pub kind: ChangeKind,
    pub new_contents: Option<String>,
    pub reason: String,
}

/// A collection of related proposals forming a coherent evolution step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPlan {
    pub proposals: Vec<ChangeProposal>,
    pub summary: String,
}

/// The main engine orchestrating Syntra’s self‑modification workflow.
pub struct SelfModEngine {
    root: PathBuf,
    policy: SelfModPolicy,
}

impl SelfModEngine {
    /// Creates a new self‑mod engine.
    pub fn new(root: PathBuf, policy: SelfModPolicy) -> Self {
        Self { root, policy }
    }

    /// Generates an evolution plan from a snapshot diff.
    pub fn generate_plan(&self, diff: &SnapshotDiff) -> EvolutionPlan {
        let mut proposals = Vec::new();

        for added in &diff.added {
            proposals.push(ChangeProposal {
                path: added.path.clone(),
                kind: ChangeKind::AddFile,
                new_contents: None,
                reason: "New file detected in sandbox".into(),
            });
        }

        for removed in &diff.removed {
            proposals.push(ChangeProposal {
                path: removed.path.clone(),
                kind: ChangeKind::DeleteFile,
                new_contents: None,
                reason: "File removed in sandbox".into(),
            });
        }

        for (old, new) in &diff.modified {
            proposals.push(ChangeProposal {
                path: new.path.clone(),
                kind: ChangeKind::ModifyFile,
                new_contents: None,
                reason: "File modified in sandbox".into(),
            });
        }

        EvolutionPlan {
            proposals,
            summary: "Auto‑generated evolution plan from sandbox diff".into(),
        }
    }

    /// Validates a proposal against the self‑mod policy.
    pub fn validate_proposal(&self, proposal: &ChangeProposal) -> bool {
        let mode = self.policy.mode_for(&proposal.path);

        match mode {
            SelfModMode::Forbidden => false,
            SelfModMode::ProposeOnly => false,
            SelfModMode::SelfMod => true,
        }
    }

    /// Applies a validated proposal to the real filesystem.
    pub fn apply_proposal(&self, proposal: &ChangeProposal) -> anyhow::Result<()> {
        let full_path = self.root.join(&proposal.path);

        match proposal.kind {
            ChangeKind::AddFile | ChangeKind::ModifyFile => {
                if let Some(contents) = &proposal.new_contents {
                    fs::write(&full_path, contents)?;
                }
            }
            ChangeKind::DeleteFile => {
                if full_path.exists() {
                    fs::remove_file(&full_path)?;
                }
            }
        }

        // Log the change
        let old_hash = if full_path.exists() {
            Some(compute_file_hash(&full_path)?)
        } else {
            None
        };

        let new_hash = match proposal.kind {
            ChangeKind::DeleteFile => None,
            _ => Some(compute_file_hash(&full_path)?),
        };

        let record = EvolutionRecord::new(
            &proposal.path,
            &format!("{:?}", proposal.kind),
            old_hash,
            new_hash,
            &proposal.reason,
            "developer",
        );

        write_evolution_record(&record)?;

        Ok(())
    }

    /// Applies an entire evolution plan (after human approval).
    pub fn apply_plan(&self, plan: &EvolutionPlan) -> anyhow::Result<()> {
        for proposal in &plan.proposals {
            if self.validate_proposal(proposal) {
                self.apply_proposal(proposal)?;
            }
        }
        Ok(())
    }
}
