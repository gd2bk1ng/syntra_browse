// ================================================================================================
//   SYNTRA KERNEL — SELF-HEALING ADVISOR
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/self_healing_advisor.rs
//   Module:      AGI Core — Self-Healing Advisor
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Interprets sandbox execution results and produces structured healing reports. The advisor
//       is responsible for turning raw build/test output into:
//
//         • Human-readable diagnoses
//         • Severity levels
//         • Proposed healing actions
//         • Rationale for each proposal
//
//       This is the cognitive layer that lets Syntra explain *what broke*, *why it matters*,
//       and *how it could be fixed* — before any live changes are applied.
//
//   Overview:
//       - HealingSeverity   — informational → critical.
//       - HealingActionKind — update dep, refactor, internalize, etc.
//       - HealingIssue      — a single detected problem.
//       - HealingAction     — a proposed fix with rationale.
//       - HealingReport     — full analysis of a sandbox run.
//       - SelfHealingAdvisor::analyze() — main entrypoint.
//
//   Notes:
//       - Designed to be extended with richer pattern matching over time.
//       - ASCII-safe, dependency-minimal, stable.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use crate::agi_core::{EvolutionPlan};
use crate::agi_core::sandbox::SandboxResult;

use serde::{Serialize, Deserialize};

/// Severity of a detected issue.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealingSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Kind of healing action being proposed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealingActionKind {
    /// No change required; informational only.
    Observe,
    /// Update an external dependency version or feature.
    UpdateDependency,
    /// Refactor internal code to match new APIs or constraints.
    RefactorCode,
    /// Replace an external dependency with an internal lobe.
    InternalizeDependency,
    /// Adjust configuration, flags, or build settings.
    AdjustConfiguration,
    /// Rerun with different parameters or environment.
    RetryWithVariation,
}

/// A single detected issue from a sandbox run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingIssue {
    pub severity: HealingSeverity,
    pub message: String,
    pub source: String, // e.g., "build", "test", "dependency", "compiler"
}

/// A proposed healing action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingAction {
    pub kind: HealingActionKind,
    pub description: String,
    pub rationale: String,
}

/// Full healing report for a sandbox run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingReport {
    pub sandbox_path: String,
    pub build_ok: bool,
    pub test_ok: bool,
    pub issues: Vec<HealingIssue>,
    pub actions: Vec<HealingAction>,
    pub diff_summary: String,
}

#[derive(Debug)]
pub struct SelfHealingAdvisor;

impl SelfHealingAdvisor {
    pub fn new() -> Self {
        Self
    }

    /// Analyze an evolution plan + sandbox result and produce a healing report.
    ///
    /// This is where Syntra "explains" what went wrong and what could be done.
    pub fn analyze(
        &self,
        _plan: &EvolutionPlan,
        result: &SandboxResult,
    ) -> anyhow::Result<HealingReport> {
        let mut issues = Vec::new();
        let mut actions = Vec::new();

        // Analyze build output
        if !result.build_ok {
            issues.extend(self.analyze_build_output(&result.build_output));
        }

        // Analyze test output
        if !result.test_ok {
            issues.extend(self.analyze_test_output(&result.test_output));
        }

        // If everything is green, emit an informational action.
        if result.build_ok && result.test_ok {
            actions.push(HealingAction {
                kind: HealingActionKind::Observe,
                description: "Sandbox build and tests succeeded; plan appears safe.".to_string(),
                rationale: "No errors detected in sandbox execution.".to_string(),
            });
        } else {
            // Propose generic actions based on issues.
            self.propose_actions_from_issues(&issues, &mut actions);
        }

        Ok(HealingReport {
            sandbox_path: result.sandbox_path.clone(),
            build_ok: result.build_ok,
            test_ok: result.test_ok,
            issues,
            actions,
            diff_summary: result.diff_summary.clone(),
        })
    }

    fn analyze_build_output(&self, output: &str) -> Vec<HealingIssue> {
        let mut issues = Vec::new();
        let lower = output.to_lowercase();

        if lower.contains("deprecated") {
            issues.push(HealingIssue {
                severity: HealingSeverity::Warning,
                message: "Build output contains deprecated API usage.".to_string(),
                source: "build".to_string(),
            });
        }

        if lower.contains("error") {
            issues.push(HealingIssue {
                severity: HealingSeverity::Error,
                message: "Build failed with compiler errors.".to_string(),
                source: "build".to_string(),
            });
        }

        if lower.contains("mismatched types") {
            issues.push(HealingIssue {
                severity: HealingSeverity::Error,
                message: "Type mismatch detected in build output.".to_string(),
                source: "build".to_string(),
            });
        }

        if issues.is_empty() && !output.trim().is_empty() {
            issues.push(HealingIssue {
                severity: HealingSeverity::Info,
                message: "Non-empty build output; review recommended.".to_string(),
                source: "build".to_string(),
            });
        }

        issues
    }

    fn analyze_test_output(&self, output: &str) -> Vec<HealingIssue> {
        let mut issues = Vec::new();
        let lower = output.to_lowercase();

        if lower.contains("failed") && lower.contains("test") {
            issues.push(HealingIssue {
                severity: HealingSeverity::Error,
                message: "One or more tests failed in sandbox.".to_string(),
                source: "test".to_string(),
            });
        }

        if lower.contains("panic") {
            issues.push(HealingIssue {
                severity: HealingSeverity::Critical,
                message: "Runtime panic detected during tests.".to_string(),
                source: "test".to_string(),
            });
        }

        if issues.is_empty() && !output.trim().is_empty() {
            issues.push(HealingIssue {
                severity: HealingSeverity::Info,
                message: "Non-empty test output; review recommended.".to_string(),
                source: "test".to_string(),
            });
        }

        issues
    }

    fn propose_actions_from_issues(
        &self,
        issues: &[HealingIssue],
        actions: &mut Vec<HealingAction>,
    ) {
        let mut has_deprecated = false;
        let mut has_build_error = false;
        let mut has_test_error = false;
        let mut has_panic = false;

        for issue in issues {
            let msg = issue.message.to_lowercase();
            if msg.contains("deprecated") {
                has_deprecated = true;
            }
            if issue.source == "build" && issue.severity >= HealingSeverity::Error {
                has_build_error = true;
            }
            if issue.source == "test" && issue.severity >= HealingSeverity::Error {
                has_test_error = true;
            }
            if msg.contains("panic") {
                has_panic = true;
            }
        }

        if has_deprecated {
            actions.push(HealingAction {
                kind: HealingActionKind::UpdateDependency,
                description: "Review and update deprecated APIs or dependency versions.".to_string(),
                rationale: "Build output indicates deprecated usage; updating reduces future breakage risk.".to_string(),
            });
        }

        if has_build_error {
            actions.push(HealingAction {
                kind: HealingActionKind::RefactorCode,
                description: "Refactor code to resolve compiler errors.".to_string(),
                rationale: "Build errors indicate incompatible signatures, types, or APIs.".to_string(),
            });
        }

        if has_test_error {
            actions.push(HealingAction {
                kind: HealingActionKind::AdjustConfiguration,
                description: "Review failing tests and adjust configuration or logic as needed.".to_string(),
                rationale: "Test failures indicate behavioral mismatches or regressions.".to_string(),
            });
        }

        if has_panic {
            actions.push(HealingAction {
                kind: HealingActionKind::RefactorCode,
                description: "Investigate and eliminate runtime panics.".to_string(),
                rationale: "Panics indicate unsafe assumptions or unhandled edge cases.".to_string(),
            });
        }

        if actions.is_empty() && !issues.is_empty() {
            actions.push(HealingAction {
                kind: HealingActionKind::Observe,
                description: "Issues detected; manual review recommended.".to_string(),
                rationale: "No specific automated healing strategy identified yet.".to_string(),
            });
        }
    }
}
