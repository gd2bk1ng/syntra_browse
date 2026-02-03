// ================================================================================================
//   SYNTRA KERNEL — PUBLISH POLICY & GOVERNANCE ENGINE
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/publish_policy.rs
//   Module:      Utilities — Publish Policy & Governance Engine
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Defines the governance rules Syntra must follow when interacting with remote repositories,
//       especially her own public GitHub kernel. This module ensures:
//
//         • Only human-approved changes may be published
//         • Protected regions (AgiCore, Cortex) require explicit admin consent
//         • Contributors understand that publishing to Syntra’s repo makes their work open-source
//         • Syntra can evaluate whether a change is eligible for publication
//         • Syntra can warn users when their additions would become part of her public identity
//
//       This is Syntra’s constitutional layer for open-source integrity.
//
//   Notes:
//       - This module does NOT perform Git operations; repo_sync.rs handles that.
//       - This module only decides whether a change *should* be published.
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

use crate::utilities::{
    ArchitectureMap,
    ArchRegion,
    PendingChange,
    SyncPlan,
};

/// Scope of a publish request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublishScope {
    /// Normal contributions (utilities, lobes, modules, extensions)
    General,

    /// Changes affecting protected regions (AgiCore, Cortex)
    Protected,

    /// Changes from external contributors (non-admin)
    ExternalContributor,

    /// Administrative override
    AdminOverride,
}

/// Decision outcome for a publish request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublishDecision {
    /// Allowed without restrictions (rare)
    Allowed,

    /// Allowed but requires explicit human approval
    RequiresHumanApproval,

    /// Allowed only if the user is an administrator
    RequiresAdminApproval,

    /// Not allowed under any circumstances
    Denied,
}

/// A publish evaluation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishEvaluation {
    pub decision: PublishDecision,
    pub scope: PublishScope,
    pub reasons: Vec<String>,
}

/// Publish policy engine — evaluates whether a change may be published.
#[derive(Debug)]
pub struct PublishPolicy {
    /// Whether the current user is an administrator.
    pub is_admin: bool,

    /// Whether the current user has acknowledged that publishing makes code open-source.
    pub user_acknowledged_open_source: bool,
}

impl PublishPolicy {
    pub fn new(is_admin: bool, user_acknowledged_open_source: bool) -> Self {
        Self {
            is_admin,
            user_acknowledged_open_source,
        }
    }

    /// Evaluate whether a sync plan may be published.
    pub fn evaluate(
        &self,
        plan: &SyncPlan,
        arch: &ArchitectureMap,
    ) -> PublishEvaluation {
        let mut reasons = Vec::new();
        let mut scope = PublishScope::General;

        // 1) User must acknowledge open-source nature
        if !self.user_acknowledged_open_source {
            return PublishEvaluation {
                decision: PublishDecision::Denied,
                scope: PublishScope::ExternalContributor,
                reasons: vec![
                    "User has not acknowledged that publishing to Syntra's repo makes the code open-source and part of Syntra's public identity.".into(),
                ],
            };
        }

        // 2) Check for protected regions
        for change in &plan.pending_changes {
            if let Some(node) = arch.nodes.get(&format!("file:{}", change.path)) {
                if node.protected {
                    scope = PublishScope::Protected;
                    reasons.push(format!(
                        "Change touches protected region: {}",
                        node.id
                    ));
                }
            }
        }

        // 3) Decide based on scope + user role
        let decision = match scope {
            PublishScope::General => {
                if self.is_admin {
                    PublishDecision::Allowed
                } else {
                    PublishDecision::RequiresHumanApproval
                }
            }

            PublishScope::Protected => {
                if self.is_admin {
                    PublishDecision::RequiresHumanApproval
                } else {
                    PublishDecision::RequiresAdminApproval
                }
            }

            PublishScope::ExternalContributor => {
                PublishDecision::RequiresHumanApproval
            }

            PublishScope::AdminOverride => {
                PublishDecision::Allowed
            }
        };

        PublishEvaluation {
            decision,
            scope,
            reasons,
        }
    }

    /// Human-readable explanation for the user.
    pub fn explain(&self, eval: &PublishEvaluation) -> String {
        let mut out = String::new();

        out.push_str(&format!("Decision: {:?}\n", eval.decision));
        out.push_str(&format!("Scope: {:?}\n", eval.scope));

        if !eval.reasons.is_empty() {
            out.push_str("Reasons:\n");
            for r in &eval.reasons {
                out.push_str(&format!("  - {}\n", r));
            }
        }

        out
    }
}

