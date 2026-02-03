// ================================================================================================
//   SYNTRA KERNEL — SELF-MOD ORCHESTRATOR (AGI CORE EVOLUTION ENGINE)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/self_mod_orchestrator.rs
//   Module:      AgiCore — Self-Modification Orchestrator
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       High-level evolution engine that coordinates Syntra’s safe self-modification.
//
//       Responsibilities:
//         • Observe: use utilities (introspection, metrics, risk) to find improvement targets
//         • Plan: build refactor/evolution plans using refactor_engine + change_impact_graph
//         • Govern: route all plans through SelfModGate and safety policy
//         • Propose: surface human-readable proposals for review/approval
//         • Apply: delegate actual changes to self_mod_engine (never directly mutate)
//
//       This is the “conductor” that turns raw capabilities into governed evolution.
//
//   Notes:
//       - Does NOT write to disk or Git directly.
//       - All mutations go through: SelfModGate + self_mod_engine + safety layers.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use std::path::Path;

use crate::utilities::{
    ArchitectureMap,
    ArchitectureMapBuilder,
    BaselineSnapshot,
    ChangeImpactAnalyzer,
    CodeIndex,
    ComplexityAnalyzer,
    DependencyGraph,
    EvolutionPredictor,
    ImpactGraph,
    IntrospectionHub,
    RiskAnalyzer,
    RiskLevel,
    SemanticFsView,
    SelfModGate,
    SelfModKind,
    SelfModRequest,
    SelfModDecision,
    TamperMonitor,
};

use crate::agi_core::self_mod_engine::SelfModEngine;

/// High-level evolution goal.
#[derive(Debug, Clone)]
pub enum EvolutionGoal {
    /// Reduce complexity / improve maintainability.
    ReduceComplexity,
    /// Improve performance in hotspots.
    ImprovePerformance,
    /// Harden safety / security-sensitive regions.
    HardenSafety,
    /// Address specific file/module.
    TargetPath(String),
}

/// A proposed evolution plan at orchestrator level.
#[derive(Debug, Clone)]
pub struct EvolutionProposal {
    pub goal: EvolutionGoal,
    pub target_paths: Vec<String>,
    pub risk_level: RiskLevel,
    pub human_summary: String,
}

/// Orchestrator configuration.
#[derive(Debug, Clone)]
pub struct SelfModOrchestratorConfig {
    pub max_targets: usize,
}

/// Self-modification orchestrator — AGI core evolution engine.
#[derive(Debug)]
pub struct SelfModOrchestrator<B: crate::utilities::GitBackend> {
    config: SelfModOrchestratorConfig,
    introspection: IntrospectionHub,
    complexity: ComplexityAnalyzer,
    risk: RiskAnalyzer,
    impact: ChangeImpactAnalyzer,
    evolution_predictor: EvolutionPredictor,
    arch_builder: ArchitectureMapBuilder,
    tamper_monitor: TamperMonitor<B>,
    self_mod_gate: SelfModGate<B>,
    self_mod_engine: SelfModEngine,
}

impl<B: crate::utilities::GitBackend> SelfModOrchestrator<B> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: SelfModOrchestratorConfig,
        introspection: IntrospectionHub,
        complexity: ComplexityAnalyzer,
        risk: RiskAnalyzer,
        impact: ChangeImpactAnalyzer,
        evolution_predictor: EvolutionPredictor,
        arch_builder: ArchitectureMapBuilder,
        tamper_monitor: TamperMonitor<B>,
        self_mod_gate: SelfModGate<B>,
        self_mod_engine: SelfModEngine,
    ) -> Self {
        Self {
            config,
            introspection,
            complexity,
            risk,
            impact,
            evolution_predictor,
            arch_builder,
            tamper_monitor,
            self_mod_gate,
            self_mod_engine,
        }
    }

    /// Main entry point: given a high-level goal, propose an evolution plan.
    pub fn propose_evolution(
        &self,
        root: &Path,
        baseline: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
        goal: EvolutionGoal,
    ) -> anyhow::Result<EvolutionProposal> {
        // 1) Build architecture map from current state.
        let arch: ArchitectureMap = self
            .arch_builder
            .build(root, baseline, semantic, index, deps);

        // 2) Run introspection snapshot (metrics, hotspots, etc.).
        let snapshot = self.introspection.snapshot(root, baseline, semantic, index, deps)?;

        // 3) Use complexity + risk analyzers to find candidate targets.
        let complexity_report = self.complexity.analyze(&snapshot)?;
        let risk_report = self.risk.assess(&snapshot)?;

        let mut candidates: Vec<(String, RiskLevel)> = Vec::new();

        for file in &complexity_report.files {
            if let Some(risk) = risk_report.file_risks.get(&file.path) {
                candidates.push((file.path.clone(), risk.level));
            }
        }

        // Sort by risk descending and truncate.
        candidates.sort_by_key(|(_, level)| match level {
            RiskLevel::Low => 0,
            RiskLevel::Medium => 1,
            RiskLevel::High => 2,
            RiskLevel::Critical => 3,
        });
        candidates.reverse();
        candidates.truncate(self.config.max_targets);

        let target_paths: Vec<String> = match goal {
            EvolutionGoal::TargetPath(ref p) => vec![p.clone()],
            _ => candidates.iter().map(|(p, _)| p.clone()).collect(),
        };

        let highest_risk = candidates
            .first()
            .map(|(_, level)| *level)
            .unwrap_or(RiskLevel::Low);

        let human_summary = format!(
            "Proposed evolution for goal {:?} targeting {} path(s), highest risk: {:?}.",
            goal,
            target_paths.len(),
            highest_risk
        );

        Ok(EvolutionProposal {
            goal,
            target_paths,
            risk_level: highest_risk,
            human_summary,
        })
    }

    /// Take a proposal, route it through SelfModGate, and if allowed, delegate to self_mod_engine.
    ///
    /// This does NOT auto-apply changes; it respects the decision kind and expects the caller
    /// to handle human/admin approval where required.
    pub fn evaluate_and_execute(
        &self,
        root: &Path,
        baseline: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
        arch: &ArchitectureMap,
        proposal: &EvolutionProposal,
        author: &str,
        github_username: Option<String>,
    ) -> anyhow::Result<(SelfModDecision, Option<String>)> {
        // 1) Build self-mod request.
        let kind = match proposal.goal {
            EvolutionGoal::ReduceComplexity => SelfModKind::Refactor,
            EvolutionGoal::ImprovePerformance => SelfModKind::FeatureOrFix,
            EvolutionGoal::HardenSafety => SelfModKind::FeatureOrFix,
            EvolutionGoal::TargetPath(_) => SelfModKind::FeatureOrFix,
        };

        let request = SelfModRequest {
            author: author.to_string(),
            github_username,
            affected_paths: proposal.target_paths.clone(),
            kind,
            reason: proposal.human_summary.clone(),
        };

        // 2) Ask the gate.
        let decision = self.self_mod_gate.evaluate_request(
            root,
            baseline,
            semantic,
            index,
            deps,
            arch,
            &request,
        )?;

        // 3) If denied, stop here.
        use crate::utilities::SelfModDecisionKind::*;
        match decision.kind {
            SelfModDecisionKind::Denied => {
                return Ok((decision, None));
            }
            SelfModDecisionKind::RequiresHumanApproval
            | SelfModDecisionKind::RequiresAdminApproval => {
                // Caller is expected to present decision.reasons to a human and re-invoke
                // self_mod_engine only after explicit approval.
                return Ok((decision, None));
            }
            SelfModDecisionKind::Allowed => {
                // 4) Delegate to self_mod_engine to actually plan/apply changes.
                let plan_id = self.self_mod_engine.plan_and_maybe_apply(
                    root,
                    baseline,
                    semantic,
                    index,
                    deps,
                    &proposal.target_paths,
                    &proposal.human_summary,
                )?;
                Ok((decision, Some(plan_id)))
            }
        }
    }
}

