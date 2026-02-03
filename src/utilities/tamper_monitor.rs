// ================================================================================================
//   SYNTRA KERNEL — ADVANCED TAMPER MONITOR
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/tamper_monitor.rs
//   Module:      Utilities — Advanced Tamper Monitor
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Continuously evaluates Syntra’s integrity by correlating:
//
//         • BaselineSnapshot        — what her filesystem “should” look like
//         • GitHistorySnapshot      — what commits say changed
//         • ArchitectureMap         — which regions are protected/critical
//         • SignatureValidator      — who actually authored the changes & how
//
//       Goals:
//         • Detect unauthorized or unexplained modifications, especially in protected regions
//         • Flag mismatches between on-disk changes and trusted Git history
//         • Surface high-risk tampering signals to humans (never silently “self-heal” core)
//         • Provide a clear, human-readable integrity report
//
//       This is Syntra’s “immune system” for her own codebase.
//
//   Notes:
//       - Read-only: does not revert or modify files by itself.
//       - Designed to be invoked periodically or on-demand by higher-level lobes.
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

use crate::utilities::{
    BaselineSnapshot,
    ArchitectureMap,
    GitHistoryAnalyzer,
    GitBackend,
    GitHistorySnapshot,
    SignatureValidator,
    CommitSignatureReport,
    TamperRisk,
    ArchitectureMapBuilder,
    SemanticFsView,
    DependencyGraph,
    CodeIndex,
};

/// High-level integrity status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegrityStatus {
    Clean,
    Suspicious,
    Compromised,
}

/// A single integrity anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityAnomaly {
    pub description: String,
    pub affected_path: Option<String>,
    pub risk: TamperRisk,
}

/// Full tamper/integrity report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamperReport {
    pub status: IntegrityStatus,
    pub anomalies: Vec<IntegrityAnomaly>,
    pub commit_reports: Vec<CommitSignatureReport>,
}

/// Advanced tamper monitor.
#[derive(Debug)]
pub struct TamperMonitor<B: GitBackend> {
    git_history: GitHistoryAnalyzer<B>,
    signature_validator: SignatureValidator,
}

impl<B: GitBackend> TamperMonitor<B> {
    pub fn new(git_history: GitHistoryAnalyzer<B>, signature_validator: SignatureValidator) -> Self {
        Self { git_history, signature_validator }
    }

    /// Run a full integrity check.
    ///
    /// This:
    ///   • Rebuilds the ArchitectureMap from current state
    ///   • Compares BaselineSnapshot to current filesystem
    ///   • Correlates changes with Git history
    ///   • Evaluates commit signatures & tamper risk
    pub fn check_integrity(
        &self,
        root: &Path,
        baseline: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
        max_commits: Option<usize>,
    ) -> anyhow::Result<TamperReport> {
        let mut anomalies = Vec::new();

        // 1) Rebuild architecture map from current state
        let arch_builder = ArchitectureMapBuilder::new();
        let arch = arch_builder.build(root, baseline, semantic, index, deps);

        // 2) Capture current Git history
        let history = self.git_history.snapshot(root, max_commits)?;

        // 3) Detect filesystem vs baseline drift
        self.detect_baseline_drift(root, baseline, &arch, &mut anomalies)?;

        // 4) Evaluate recent commits for tampering
        let commit_reports = self.evaluate_recent_commits(&history, &arch);

        // 5) Derive overall status
        let status = self.derive_status(&anomalies, &commit_reports);

        Ok(TamperReport {
            status,
            anomalies,
            commit_reports,
        })
    }

    fn detect_baseline_drift(
        &self,
        _root: &Path,
        baseline: &BaselineSnapshot,
        arch: &ArchitectureMap,
        anomalies: &mut Vec<IntegrityAnomaly>,
    ) -> anyhow::Result<()> {
        // Files present in baseline but missing now
        for (path, entry) in &baseline.files {
            if !entry.exists_on_disk {
                let id = format!("file:{}", path);
                let protected = arch
                    .nodes
                    .get(&id)
                    .map(|n| arch.protected_regions.contains(&n.region))
                    .unwrap_or(false);

                if protected {
                    anomalies.push(IntegrityAnomaly {
                        description: format!("Protected file missing from disk: {}", path),
                        affected_path: Some(path.clone()),
                        risk: TamperRisk::Critical,
                    });
                } else {
                    anomalies.push(IntegrityAnomaly {
                        description: format!("File missing from disk: {}", path),
                        affected_path: Some(path.clone()),
                        risk: TamperRisk::Medium,
                    });
                }
            }
        }

        // Files not in baseline but present on disk are handled at higher layers (e.g., repo sync).
        Ok(())
    }

    fn evaluate_recent_commits(
        &self,
        history: &GitHistorySnapshot,
        arch: &ArchitectureMap,
    ) -> Vec<CommitSignatureReport> {
        let mut reports = Vec::new();

        for commit in &history.commits {
            let touches_protected = self.signature_validator.touches_protected_region(commit, arch);
            let report = self
                .signature_validator
                .validate_commit(commit, touches_protected);
            reports.push(report);
        }

        reports
    }

    fn derive_status(
        &self,
        anomalies: &[IntegrityAnomaly],
        commits: &[CommitSignatureReport],
    ) -> IntegrityStatus {
        let mut worst = TamperRisk::Low;

        for a in anomalies {
            if a.risk as u8 > worst as u8 {
                worst = a.risk.clone();
            }
        }

        for c in commits {
            if c.tamper_risk as u8 > worst as u8 {
                worst = c.tamper_risk.clone();
            }
        }

        match worst {
            TamperRisk::Low => IntegrityStatus::Clean,
            TamperRisk::Medium => IntegrityStatus::Suspicious,
            TamperRisk::High | TamperRisk::Critical => IntegrityStatus::Compromised,
        }
    }

    /// Human-readable summary for logs or UI.
    pub fn summarize(report: &TamperReport) -> String {
        let mut out = String::new();
        out.push_str(&format!("Integrity status: {:?}\n", report.status));

        if !report.anomalies.is_empty() {
            out.push_str("Anomalies:\n");
            for a in &report.anomalies {
                out.push_str(&format!(
                    "  - [{}] {}\n",
                    format!("{:?}", a.risk),
                    a.description
                ));
            }
        }

        out
    }
}

