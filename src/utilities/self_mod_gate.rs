// ================================================================================================
//   SYNTRA KERNEL — SELF-MOD GATE (UNIFIED PERMISSION GUARD)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/self_mod_gate.rs
//   Module:      Utilities — Self-Modification Permission Gate
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Centralizes all decisions about whether Syntra is allowed to modify herself.
//
//       This gate pulls together:
//         • IntegrityDaemon        — is Syntra currently locked down?
//         • TamperMonitor         — recent integrity/tamper signals
//         • PublishPolicy         — governance & open-source rules
//         • GitHubUserVerifier    — contributor identity & trust
//         • SignatureValidator    — commit provenance & tamper risk
//         • ArchitectureMap       — protected regions (AgiCore, Cortex)
//
//       Threats addressed:
//         • Malicious code contributions
//         • Supply-chain / dependency abuse (via policy hooks)
//         • Unauthorized local modifications
//         • Poisoned / disguised “features”
//         • Identity spoofing / unverified contributors
//         • Social engineering of self-modification
//
//       All self-modification flows MUST go through this gate.
//
//   Notes:
//       - This module does not perform modifications itself.
//       - It only answers: “Is this self-mod allowed, and under what conditions?”
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
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::utilities::{
    ArchitectureMap,
    PublishPolicy,
    PublishEvaluation,
    GitHubUserVerifier,
    GitHubUserVerification,
    SignatureValidator,
    TamperMonitor,
    IntegrityDaemon,
    IntegrityStatus,
    SelfModMode,
    BaselineSnapshot,
    SemanticFsView,
    CodeIndex,
    DependencyGraph,
    GitBackend,
};

/// High-level classification of a self-modification request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelfModKind {
    /// Refactor, cleanup, or non-behavioral change.
    Refactor,
    /// Behavioral change in non-protected regions.
    FeatureOrFix,
    /// Any change that touches protected regions (AgiCore, Cortex).
    ProtectedCoreChange,
}

/// Description of a proposed self-modification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModRequest {
    /// Human or GitHub username initiating the change.
    pub author: String,
    /// Optional GitHub handle (if different from author).
    pub github_username: Option<String>,
    /// Files or modules affected.
    pub affected_paths: Vec<String>,
    /// High-level kind of change.
    pub kind: SelfModKind,
    /// Human-provided reason/intent.
    pub reason: String,
}

/// Decision outcome for a self-mod request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelfModDecisionKind {
    /// Allowed immediately (rare; typically for low-risk refactors).
    Allowed,
    /// Allowed only with explicit human approval.
    RequiresHumanApproval,
    /// Allowed only with admin approval.
    RequiresAdminApproval,
    /// Denied outright.
    Denied,
}

/// Full decision with context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModDecision {
    pub kind: SelfModDecisionKind,
    pub reasons: Vec<String>,
}

/// Self-modification gate — unified guard for all self-mod flows.
#[derive(Debug)]
pub struct SelfModGate<B: GitBackend> {
    pub publish_policy: PublishPolicy,
    pub github_verifier: GitHubUserVerifier,
    pub signature_validator: SignatureValidator,
    pub tamper_monitor: TamperMonitor<B>,
    pub integrity_daemon: IntegrityDaemon<B>,
}

impl<B: GitBackend> SelfModGate<B> {
    pub fn new(
        publish_policy: PublishPolicy,
        github_verifier: GitHubUserVerifier,
        signature_validator: SignatureValidator,
        tamper_monitor: TamperMonitor<B>,
        integrity_daemon: IntegrityDaemon<B>,
    ) -> Self {
        Self {
            publish_policy,
            github_verifier,
            signature_validator,
            tamper_monitor,
            integrity_daemon,
        }
    }

    /// Evaluate whether a self-mod request is allowed, and under what conditions.
    pub fn evaluate_request(
        &self,
        root: &Path,
        baseline: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
        arch: &ArchitectureMap,
        request: &SelfModRequest,
    ) -> anyhow::Result<SelfModDecision> {
        let mut reasons = Vec::new();

        // 1) Check global integrity / lockdown state.
        if !self.integrity_daemon.self_mod_allowed() {
            reasons.push("IntegrityDaemon: self-modification currently locked down due to compromised state.".into());
            return Ok(SelfModDecision {
                kind: SelfModDecisionKind::RequiresAdminApproval,
                reasons,
            });
        }

        // 2) Run a fresh tamper check (defensive).
        let report = self.tamper_monitor.check_integrity(
            root,
            baseline,
            semantic,
            index,
            deps,
            Some(50),
        )?;

        match report.status {
            IntegrityStatus::Compromised => {
                reasons.push("TamperMonitor: integrity compromised; no self-mod allowed until human override.".into());
                return Ok(SelfModDecision {
                    kind: SelfModDecisionKind::RequiresAdminApproval,
                    reasons,
                });
            }
            IntegrityStatus::Suspicious => {
                reasons.push("TamperMonitor: suspicious activity detected; self-mod requires human approval.".into());
            }
            IntegrityStatus::Clean => {}
        }

        // 3) Verify contributor identity (GitHub).
        let github_user = request
            .github_username
            .as_deref()
            .unwrap_or(&request.author);

        let verification = self.github_verifier.verify_user(github_user)?;
        if !self.github_verifier.is_contributor_allowed(&verification) {
            reasons.push(format!(
                "GitHubUserVerifier: contributor '{}' is not allowed to submit code (trust level: {:?}).",
                verification.username, verification.trust_level
            ));
            return Ok(SelfModDecision {
                kind: SelfModDecisionKind::Denied,
                reasons,
            });
        } else {
            reasons.push(format!(
                "GitHubUserVerifier: contributor '{}' accepted with trust level {:?}.",
                verification.username, verification.trust_level
            ));
        }

        // 4) Determine if protected regions are involved.
        let mut touches_protected = false;
        for path in &request.affected_paths {
            let id = format!("file:{}", path);
            if let Some(node) = arch.nodes.get(&id) {
                if arch.protected_regions.contains(&node.region) {
                    touches_protected = true;
                    reasons.push(format!(
                        "Request touches protected region via '{}'.",
                        path
                    ));
                }
            }
        }

        // 5) Map to publish policy scope.
        let dummy_sync_plan = crate::utilities::SyncPlan {
            pending_changes: request
                .affected_paths
                .iter()
                .map(|p| crate::utilities::PendingChange {
                    path: p.clone(),
                    kind: crate::utilities::ChangeKind::Modified,
                    is_remote: false,
                })
                .collect(),
            actions: Vec::new(),
            requires_human_approval: false,
        };

        let publish_eval = self.publish_policy.evaluate(&dummy_sync_plan, arch);
        reasons.push(format!(
            "PublishPolicy: decision={:?}, scope={:?}.",
            publish_eval.decision, publish_eval.scope
        ));

        // 6) Derive final decision.
        let decision_kind = self.derive_decision_kind(
            &request,
            touches_protected,
            &verification,
            &publish_eval,
            &report,
            &mut reasons,
        );

        Ok(SelfModDecision {
            kind: decision_kind,
            reasons,
        })
    }

    fn derive_decision_kind(
        &self,
        request: &SelfModRequest,
        touches_protected: bool,
        verification: &GitHubUserVerification,
        publish_eval: &PublishEvaluation,
        tamper_report: &crate::utilities::TamperReport,
        reasons: &mut Vec<String>,
    ) -> SelfModDecisionKind {
        // Hard deny if publish policy denies.
        use crate::utilities::PublishDecision;
        if let PublishDecision::Denied = publish_eval.decision {
            reasons.push("PublishPolicy: denied; self-mod not allowed.".into());
            return SelfModDecisionKind::Denied;
        }

        // If integrity is suspicious, escalate to human approval at minimum.
        if tamper_report.status == IntegrityStatus::Suspicious {
            reasons.push("TamperMonitor: suspicious state; escalating to human approval.".into());
            return SelfModDecisionKind::RequiresHumanApproval;
        }

        // Protected core changes are always gated.
        if touches_protected || matches!(request.kind, SelfModKind::ProtectedCoreChange) {
            if matches!(verification.trust_level, crate::utilities::TrustLevel::Admin) {
                reasons.push("Protected core change by admin; requires explicit human/admin approval.".into());
                return SelfModDecisionKind::RequiresAdminApproval;
            } else {
                reasons.push("Protected core change by non-admin; not allowed without admin.".into());
                return SelfModDecisionKind::RequiresAdminApproval;
            }
        }

        // Non-protected changes:
        match request.kind {
            SelfModKind::Refactor => {
                // Low-risk refactors by trusted users can be allowed or require human approval.
                match verification.trust_level {
                    crate::utilities::TrustLevel::Trusted | crate::utilities::TrustLevel::Admin => {
                        reasons.push("Refactor by trusted user; allowed with optional human review.".into());
                        SelfModDecisionKind::Allowed
                    }
                    _ => {
                        reasons.push("Refactor by non-trusted user; requires human approval.".into());
                        SelfModDecisionKind::RequiresHumanApproval
                    }
                }
            }
            SelfModKind::FeatureOrFix => {
                // Behavioral changes always require at least human approval.
                reasons.push("Behavioral change; requires human approval.".into());
                SelfModDecisionKind::RequiresHumanApproval
            }
            SelfModKind::ProtectedCoreChange => {
                // Already handled above, but keep a safe default.
                SelfModDecisionKind::RequiresAdminApproval
            }
        }
    }

    /// Human-readable explanation.
    pub fn explain(decision: &SelfModDecision) -> String {
        let mut out = String::new();
        out.push_str(&format!("Self-mod decision: {:?}\n", decision.kind));
        if !decision.reasons.is_empty() {
            out.push_str("Reasons:\n");
            for r in &decision.reasons {
                out.push_str(&format!("  - {}\n", r));
            }
        }
        out
    }
}

