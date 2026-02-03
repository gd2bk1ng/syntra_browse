// ================================================================================================
//   SYNTRA KERNEL — AXIOM SIX (SELF-MODIFICATION ENGINE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/self_mod.rs
//   Module:      AGI Core — Self-Modification Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Unified self-modification subsystem for Syntra. This module defines a shared evolution
//       vocabulary (ChangeKind, ChangeProposal, EvolutionPlan) and two complementary engines:
//
//         • AdvisorySelfModEngine
//             - High-level, advisory-only analysis.
//             - Proposes new lobes, refactors, dependency fixes, dead code cleanups,
//               and meta-evolution steps.
//             - NEVER mutates the filesystem.
//
//         • SelfModEngine
//             - Policy-aware, file-level evolution executor.
//             - Interprets diffs, validates proposals against SelfModPolicy,
//               applies allowed changes, and logs them via EvolutionLog.
//             - All mutations are explicit, logged, and gated by human approval.
//
//   Overview:
//       - ChangeKind          — Unified enum for architectural + file-level changes.
//       - ChangeProposal      — Structured proposal for evolution, advisory or concrete.
//       - EvolutionPlan       — Grouped set of proposals with a narrative summary.
//       - RefactorSuggestion  — Heuristic refactor hints.
//       - DeadCodeReport      — Suspected dead code symbols.
//       - CircularDependency  — Detected module cycles.
//       - AdvisorySelfModEngine — Ecosystem-based advisory engine (no mutations).
//       - SelfModEngine       — Policy-aware executor for concrete changes.
//
//   Notes:
//       - Advisory engine is purely cognitive: it thinks about evolution.
//       - Execution engine is constrained: it acts on evolution safely.
//       - Both share a unified vocabulary so Syntra can reason coherently about her own growth.
//       - MIT & Apache 2.0 dual-licensed.
//       - Designed for long-term evolution, introspection, and study.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::fs;

use serde::{Serialize, Deserialize};

use crate::agi_core::ecosystem::EcosystemModel;
use crate::agi_core::telemetry::{TelemetryBus, TelemetryLevel};
use crate::agi_core::self_mod_policy::{SelfModPolicy, SelfModMode};

use crate::utilities::{
    BaselineSnapshot,
    SnapshotDiff,
    EvolutionRecord,
    write_evolution_record,
    compute_file_hash,
};

// ================================================================================================
// Unified Evolution Vocabulary
// ================================================================================================

/// Kind of change being proposed by the self-mod system.
///
/// This enum unifies both high-level architectural evolution and low-level file changes.
/// Syntra can reason about all of these as part of a single evolution narrative.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeKind {
    // --- High-level / architectural changes (advisory) ---
    /// Introduce a new lobe (module/subsystem).
    NewLobe,
    /// Structural refactor of existing code.
    Refactor,
    /// Removal or consolidation of unused code.
    DeadCodeCleanup,
    /// Fix or simplify dependencies between modules.
    DependencyFix,
    /// Upgrade an existing lobe (fill in missing logic, tests, docs).
    Upgrade,
    /// Meta-evolution change (high-level architectural evolution).
    Evolution,

    // --- Low-level / file-level changes (concrete) ---
    /// Add a new file to the codebase.
    AddFile,
    /// Modify an existing file.
    ModifyFile,
    /// Delete an existing file.
    DeleteFile,
}

/// A single change proposal emitted by the self-mod system.
///
/// This can represent either:
///   - a high-level advisory proposal (no direct mutation), or
///   - a concrete file-level change (with optional new contents).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeProposal {
    pub kind: ChangeKind,
    /// Human-readable title for the proposal.
    pub title: String,
    /// Detailed description / rationale.
    pub description: String,
    /// Optional target path (file or module).
    pub target: Option<String>,
    /// Optional patch-like text (unified diff or instructions).
    pub patch_hint: Option<String>,
    /// Optional new contents for file-level changes (AddFile/ModifyFile).
    pub new_contents: Option<String>,
}

/// High-level evolution plan: a grouped set of proposals with a narrative.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPlan {
    pub summary: String,
    pub proposals: Vec<ChangeProposal>,
}

/// Refactor suggestion for a specific module.
#[derive(Debug, Clone)]
pub struct RefactorSuggestion {
    pub module: String,
    pub reason: String,
    pub suggestion: String,
}

/// Report of suspected dead code in a module.
#[derive(Debug, Clone)]
pub struct DeadCodeReport {
    pub module: String,
    pub symbols: Vec<String>,
}

/// Description of a circular dependency between modules.
#[derive(Debug, Clone)]
pub struct CircularDependency {
    pub modules: Vec<String>,
}

// ================================================================================================
// Advisory Self-Mod Engine (High-Level, Non-Mutating)
// ================================================================================================

/// Advisory self-modification engine.
///
/// Produces structured proposals based on ecosystem analysis. It never
/// mutates the filesystem or applies patches directly.
#[derive(Debug, Default)]
pub struct AdvisorySelfModEngine {
    telemetry: Option<TelemetryBus>,
}

impl AdvisorySelfModEngine {
    /// Construct an advisory self-mod engine without telemetry.
    pub fn new() -> Self {
        Self { telemetry: None }
    }

    /// Construct an advisory self-mod engine with telemetry enabled.
    pub fn with_telemetry(telemetry: TelemetryBus) -> Self {
        Self {
            telemetry: Some(telemetry),
        }
    }

    /// High-level entry point: analyze the ecosystem and produce change proposals.
    ///
    /// This function is advisory only. It never mutates the filesystem.
    pub fn analyze_ecosystem(
        &self,
        root: impl AsRef<Path>,
        ecosystem: &EcosystemModel,
    ) -> EvolutionPlan {
        let root = root.as_ref();

        let mut proposals = Vec::new();

        // 1) Missing / incomplete lobes → new lobe / upgrade proposals.
        self.propose_missing_lobes(&mut proposals, ecosystem);
        self.propose_incomplete_lobes(&mut proposals, ecosystem);

        // 2) Heuristic dead code / unused module detection (hook).
        let dead_code_reports = self.detect_dead_code(root);
        self.propose_dead_code_cleanups(&mut proposals, &dead_code_reports);

        // 3) Circular dependency detection (hook).
        let cycles = self.detect_circular_dependencies(root);
        self.propose_dependency_fixes(&mut proposals, &cycles);

        // 4) High-level refactor suggestions (hook).
        let refactors = self.propose_refactors(root);
        self.propose_refactors_from_suggestions(&mut proposals, &refactors);

        // 5) Evolution narrative.
        let summary = self.build_evolution_summary(&proposals);

        let plan = EvolutionPlan { summary, proposals };

        // 6) Telemetry: emit a high-level evolution summary (future-proof hook).
        if let Some(t) = &self.telemetry {
            t.log(
                TelemetryLevel::Info,
                format!(
                    "AdvisorySelfModEngine generated evolution plan with {} proposals.",
                    plan.proposals.len()
                ),
            );
        }

        plan
    }

    // --------------------------------------------------------------------------------------------
    // Proposal Builders
    // --------------------------------------------------------------------------------------------

    fn propose_missing_lobes(
        &self,
        proposals: &mut Vec<ChangeProposal>,
        ecosystem: &EcosystemModel,
    ) {
        for missing in &ecosystem.missing {
            proposals.push(ChangeProposal {
                kind: ChangeKind::NewLobe,
                title: format!("Create missing lobe '{}'", missing),
                description: format!(
                    "Lobe '{}' is missing. Propose generating a scaffold module with Rust \
                     skeletons, tests, and documentation.",
                    missing
                ),
                target: None,
                patch_hint: Some(format!(
                    "Scaffold suggestion: create module for '{}' with lib.rs, tests, and docs.",
                    missing
                )),
                new_contents: None,
            });
        }
    }

    fn propose_incomplete_lobes(
        &self,
        proposals: &mut Vec<ChangeProposal>,
        ecosystem: &EcosystemModel,
    ) {
        for incomplete in &ecosystem.incomplete {
            proposals.push(ChangeProposal {
                kind: ChangeKind::Upgrade,
                title: format!("Upgrade incomplete lobe '{}'", incomplete),
                description: format!(
                    "Lobe '{}' exists but appears incomplete or empty. Propose adding core logic, \
                     documentation, and integration tests.",
                    incomplete
                ),
                target: None,
                patch_hint: None,
                new_contents: None,
            });
        }
    }

    fn propose_dead_code_cleanups(
        &self,
        proposals: &mut Vec<ChangeProposal>,
        reports: &[DeadCodeReport],
    ) {
        for report in reports {
            proposals.push(ChangeProposal {
                kind: ChangeKind::DeadCodeCleanup,
                title: format!("Clean dead code in '{}'", report.module),
                description: format!(
                    "Module '{}' appears to contain unused symbols: {:?}. \
                     Recommend removing or consolidating them after human review.",
                    report.module, report.symbols
                ),
                target: Some(report.module.clone()),
                patch_hint: None,
                new_contents: None,
            });
        }
    }

    fn propose_dependency_fixes(
        &self,
        proposals: &mut Vec<ChangeProposal>,
        cycles: &[CircularDependency],
    ) {
        for cycle in cycles {
            proposals.push(ChangeProposal {
                kind: ChangeKind::DependencyFix,
                title: "Resolve circular dependency".into(),
                description: format!(
                    "Detected circular dependency between modules: {:?}. \
                     Recommend extracting shared interfaces into a separate lobe.",
                    cycle.modules
                ),
                target: None,
                patch_hint: None,
                new_contents: None,
            });
        }
    }

    fn propose_refactors_from_suggestions(
        &self,
        proposals: &mut Vec<ChangeProposal>,
        refactors: &[RefactorSuggestion],
    ) {
        for r in refactors {
            proposals.push(ChangeProposal {
                kind: ChangeKind::Refactor,
                title: format!("Refactor module '{}'", r.module),
                description: format!("Reason: {}. Suggestion: {}", r.reason, r.suggestion),
                target: Some(r.module.clone()),
                patch_hint: None,
                new_contents: None,
            });
        }
    }

    // --------------------------------------------------------------------------------------------
    // Analysis Hooks (Stubs)
    // --------------------------------------------------------------------------------------------

    /// Placeholder: scan for dead code (hook for future static analysis).
    ///
    /// Future: integrate with `cargo check` output, rust-analyzer, or custom static analysis.
    fn detect_dead_code(&self, _root: &Path) -> Vec<DeadCodeReport> {
        Vec::new()
    }

    /// Placeholder: scan for circular dependencies (hook for future graph analysis).
    ///
    /// Future: parse `mod` graph and `use` graph, detect cycles.
    fn detect_circular_dependencies(&self, _root: &Path) -> Vec<CircularDependency> {
        Vec::new()
    }

    /// Placeholder: propose refactors based on simple heuristics.
    ///
    /// Future: look for large files, long functions, duplicated patterns, etc.
    fn propose_refactors(&self, _root: &Path) -> Vec<RefactorSuggestion> {
        Vec::new()
    }

    // --------------------------------------------------------------------------------------------
    // Evolution Summary
    // --------------------------------------------------------------------------------------------

    /// Build a human-readable evolution summary from proposals.
    ///
    /// Includes safety-aware narrative so operators understand that all proposals are
    /// advisory and will be evaluated by the safety subsystem before any application.
    fn build_evolution_summary(&self, proposals: &[ChangeProposal]) -> String {
        let mut out = String::new();

        out.push_str("=== Syntra Evolution Plan (Axiom Six — Advisory) ===\n");
        out.push_str("This plan is advisory. All changes require explicit human approval.\n");
        out.push_str("Safety subsystem (Axiom Seven) will evaluate each proposal before any action.\n\n");

        let mut new_lobes = 0;
        let mut upgrades = 0;
        let mut refactors = 0;
        let mut cleanups = 0;
        let mut dep_fixes = 0;
        let mut meta_evolution = 0;

        for p in proposals {
            match p.kind {
                ChangeKind::NewLobe => new_lobes += 1,
                ChangeKind::Upgrade => upgrades += 1,
                ChangeKind::Refactor => refactors += 1,
                ChangeKind::DeadCodeCleanup => cleanups += 1,
                ChangeKind::DependencyFix => dep_fixes += 1,
                ChangeKind::Evolution => meta_evolution += 1,
                _ => {}
            }
        }

        out.push_str(&format!("New lobes proposed: {}\n", new_lobes));
        out.push_str(&format!("Upgrades proposed: {}\n", upgrades));
        out.push_str(&format!("Refactors proposed: {}\n", refactors));
        out.push_str(&format!("Dead code cleanups proposed: {}\n", cleanups));
        out.push_str(&format!("Dependency fixes proposed: {}\n", dep_fixes));
        out.push_str(&format!("Meta-evolution proposals: {}\n", meta_evolution));

        out
    }
}

// ================================================================================================
// Self-Mod Engine (Policy-Aware Executor)
// ================================================================================================

/// The main engine orchestrating Syntra’s concrete self-modification workflow.
///
/// This engine operates at the file level. It is expected to be driven by:
///   - sandbox diffs,
///   - approved proposals,
///   - and SelfModPolicy constraints.
pub struct SelfModEngine {
    root: PathBuf,
    policy: SelfModPolicy,
}

impl SelfModEngine {
    /// Creates a new self-mod engine.
    pub fn new(root: PathBuf, policy: SelfModPolicy) -> Self {
        Self { root, policy }
    }

    /// Generates an evolution plan from a snapshot diff.
    ///
    /// This is a low-level, file-centric plan derived from observed changes
    /// (e.g., in a sandbox). It uses the unified ChangeKind / ChangeProposal
    /// vocabulary so it can be combined with advisory plans if desired.
    pub fn generate_plan_from_diff(&self, diff: &SnapshotDiff) -> EvolutionPlan {
        let mut proposals = Vec::new();

        for added in &diff.added {
            proposals.push(ChangeProposal {
                kind: ChangeKind::AddFile,
                title: format!("Add file '{}'", added.path),
                description: "New file detected in sandbox; propose adding to main tree.".into(),
                target: Some(added.path.clone()),
                patch_hint: None,
                new_contents: None,
            });
        }

        for removed in &diff.removed {
            proposals.push(ChangeProposal {
                kind: ChangeKind::DeleteFile,
                title: format!("Delete file '{}'", removed.path),
                description: "File removed in sandbox; propose deleting from main tree.".into(),
                target: Some(removed.path.clone()),
                patch_hint: None,
                new_contents: None,
            });
        }

        for (old, new) in &diff.modified {
            proposals.push(ChangeProposal {
                kind: ChangeKind::ModifyFile,
                title: format!("Modify file '{}'", new.path),
                description: format!(
                    "File '{}' modified in sandbox; propose applying updated contents.",
                    new.path
                ),
                target: Some(new.path.clone()),
                patch_hint: None,
                new_contents: None,
            });
        }

        EvolutionPlan {
            proposals,
            summary: "Auto-generated evolution plan from sandbox diff".into(),
        }
    }

    /// Validates a proposal against the self-mod policy.
    pub fn validate_proposal(&self, proposal: &ChangeProposal) -> bool {
        let target = proposal
            .target
            .as_deref()
            .unwrap_or("");

        let mode = self.policy.mode_for(target);

        match mode {
            SelfModMode::Forbidden => false,
            SelfModMode::ProposeOnly => false,
            SelfModMode::SelfMod => true,
        }
    }

    /// Applies a validated proposal to the real filesystem.
    ///
    /// This function assumes the proposal has already been approved by a human
    /// and validated against the SelfModPolicy.
    pub fn apply_proposal(&self, proposal: &ChangeProposal) -> anyhow::Result<()> {
        let target = match &proposal.target {
            Some(t) => t,
            None => return Ok(()), // nothing to do without a concrete target
        };

        let full_path = self.root.join(target);

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
            _ => {
                // High-level advisory kinds are not applied here.
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
            ChangeKind::AddFile | ChangeKind::ModifyFile => {
                if full_path.exists() {
                    Some(compute_file_hash(&full_path)?)
                } else {
                    None
                }
            }
            _ => None,
        };

        let record = EvolutionRecord::new(
            target,
            &format!("{:?}", proposal.kind),
            old_hash,
            new_hash,
            &proposal.description,
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

    /// Convenience: capture a fresh baseline snapshot for the current root.
    pub fn snapshot(&self) -> anyhow::Result<BaselineSnapshot> {
        BaselineSnapshot::scan_tree(&self.root, &self.policy)
    }
}
