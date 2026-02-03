// ================================================================================================
//   SYNTRA KERNEL — UTILITIES (HELPERS & SYSTEM TOOLS)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/mod.rs
//   Module:      Utilities (Helpers & System Tools)
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Aggregates Syntra's lightweight and advanced utility subsystems, including structured
//       logging, diagnostics, filesystem introspection, tracing hooks, baseline snapshots,
//       evolution logging, filesystem helpers, path normalization, dependency graph analysis,
//       semantic filesystem classification, code indexing, semantic graph, architecture mapping,
//       complexity and metrics analysis, change impact modeling, refactor rules/engine, risk
//       analysis, evolution prediction, and Git-aware churn/replication utilities.
//
//       These tools form the backbone of Axiom Zero and Axiom One — enabling self-analysis,
//       observability, introspection, safe self-modification, and supervised interaction with
//       external Git repositories (including Syntra’s own public kernel).
//
//   Overview:
//       - logging                 — Human-readable & JSON-style structured logging.
//       - diagnostics             — Diagnostic event bus for internal health signals.
//       - ecosystem               — Filesystem introspection utilities for self-awareness.
//       - tracing                 — Lightweight tracing hooks for internal instrumentation.
//       - evolution_log           — Records Syntra’s self-modification events.
//       - baseline_snapshot       — Syntra’s filesystem self-image for evolution & safety.
//       - fs_utils                — Safe reads/writes, atomic updates, sandbox copying.
//       - path_utils              — Cross-platform path normalization & matching.
//       - dependency_graph        — Structural view of crates/modules/dependencies.
//       - semantic_fs             — Semantic roles over the filesystem (lobes, core, utilities).
//       - code_index              — Symbol-level index for refactors and analysis.
//       - introspection           — Unified self-awareness/introspection hub.
//       - semantic_graph          — High-level semantic graph of Syntra’s architecture.
//       - semantic_graph_builder  — Builder that fuses indices into a semantic graph.
//       - architecture_map        — Dynamic architectural blueprint with protected regions.
//       - architecture_map_builder— Builds ArchitectureMap from snapshot + semantics + deps.
//       - complexity_analyzer     — Structural complexity metrics.
//       - code_metrics            — Quantitative metrics (LOC, fan-in/out, hotspots).
//       - change_impact_graph     — Ripple-effect modeling for changes.
//       - refactor_engine         — Refactor planning and (future) application engine.
//       - refactor_rules          — Constitutional constraints for refactors.
//       - pattern_detector        — Structural pattern and smell detector.
//       - risk_analyzer           — Risk scoring for modules, dependencies, and plans.
//       - evolution_predictor     — Impact estimation for evolution and refactor plans.
//       - git_history             — Git history & churn analysis (real commit-based metrics).
//       - git_backend_git2        — Concrete Git backend using git2 (read-only).
//       - repo_sync               — Repo sync, detection of new/changed files, supervised pushes.
//       - publish_policy          — Rules for what may be published to Syntra’s public repo.
//       - remote_backend_github   — GitHub remote backend (API + remote diffing).
//       - github_user_verification— GitHub identity & trust engine.
//       - github_signature_validation
//                                — Commit signature validation & tamper risk.
//       - tamper_monitor          — Advanced tamper detection & integrity reporting.
//       - integrity_daemon        — Periodic watchdog that can lock self-modification.
//       - self_mod_gate           — Unified permission gate for all self-mod flows.
//
//   Notes:
//       - Dependency-minimal and ASCII-safe for long-term stability.
//       - Git operations must always be gated by explicit human consent and per-author policy.
//       - Anything published to the Syntra kernel repo is treated as native, open-source Syntra.
//       - MIT & Apache 2.0 dual-licensed.
//       - Designed for future expansion as Syntra evolves.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

// ================================================================================================
// Submodules — every file in src/utilities/
// ================================================================================================

pub mod ecosystem;
pub mod logging;
pub mod diagnostics;
pub mod tracing;
pub mod evolution_log;
pub mod baseline_snapshot;
pub mod fs_utils;
pub mod path_utils;

pub mod dependency_graph;
pub mod semantic_fs;
pub mod code_index;
pub mod introspection;

pub mod semantic_graph;
pub mod semantic_graph_builder;

pub mod architecture_map;
pub mod architecture_map_builder;

pub mod complexity_analyzer;
pub mod code_metrics;

pub mod change_impact_graph;

pub mod refactor_engine;
pub mod refactor_rules;
pub mod pattern_detector;
pub mod risk_analyzer;
pub mod evolution_predictor;

// Git-aware utilities (with strict human-gated policies)
pub mod git_history;
pub mod git_backend_git2;
pub mod repo_sync;
pub mod publish_policy;
pub mod remote_backend_github;
pub mod github_user_verification;
pub mod github_signature_validation;
pub mod tamper_monitor;
pub mod integrity_daemon;
pub mod self_mod_gate;

// ================================================================================================
// Re-exports — Common utilities exposed for convenience
// ================================================================================================

// Logging
pub use logging::{
    error,
    info,
    warn,
    LogLevel,
    log_human,
    log_json,
    timestamp,
};

// Diagnostics
pub use diagnostics::{
    DiagnosticBus,
    DiagnosticEvent,
    DiagnosticKind,
};

// Tracing
pub use tracing::{
    trace_event,
    trace_enter,
    trace_exit,
};

// Evolution Log
pub use evolution_log::{
    EvolutionRecord,
    write_evolution_record,
    compute_file_hash,
};

// Baseline Snapshot
pub use baseline_snapshot::{
    BaselineSnapshot,
    SnapshotDiff,
    FileEntry,
};

// Filesystem Utilities
pub use fs_utils::{
    read_text_file,
    write_text_file_atomic,
    ensure_dir,
    copy_tree,
    remove_tree,
    is_path_allowed,
};

// Path Utilities
pub use path_utils::{
    normalize_path,
    join_paths,
    is_within,
    strip_prefix_safe,
    canonicalize_lossy,
    glob_match,
};

// Dependency Graph
pub use dependency_graph::{
    DependencyGraph,
    DependencyNode,
    DependencyKind,
    DependencyEdge,
};

// Semantic Filesystem
pub use semantic_fs::{
    SemanticFsView,
    SemanticFile,
    SemanticRole,
};

// Code Index
pub use code_index::{
    CodeIndex,
    CodeSymbol,
    SymbolKind,
    CodeIndexer,
};

// Introspection Hub
pub use introspection::{
    IntrospectionHub,
    IntrospectionSnapshot,
};

// Semantic Graph
pub use semantic_graph::{
    SemanticGraph,
    SemanticNode,
    SemanticNodeKind,
    SemanticEdge,
    SemanticEdgeKind,
};

// Semantic Graph Builder
pub use semantic_graph_builder::SemanticGraphBuilder;

// Architecture Map
pub use architecture_map::{
    ArchitectureMap,
    ArchNode,
    ArchRegion,
    ArchRelation,
    ArchRelationKind,
};

// Architecture Map Builder
pub use architecture_map_builder::ArchitectureMapBuilder;

// Complexity Analyzer
pub use complexity_analyzer::{
    ComplexityAnalyzer,
    ComplexityReport,
    FileComplexity,
    FunctionComplexity,
};

// Code Metrics
pub use code_metrics::{
    CodeMetricsEngine,
    MetricsReport,
    FileMetrics,
};

// Change Impact Graph
pub use change_impact_graph::{
    ChangeImpactAnalyzer,
    ImpactGraph,
    ImpactNode,
    ImpactEdge,
    ImpactScore,
};

// Refactor Engine
pub use refactor_engine::{
    RefactorEngine,
    RefactorPlan,
    RefactorOperation,
    RefactorKind,
};

// Refactor Rules
pub use refactor_rules::{
    RefactorRuleEngine,
    RefactorRule,
    RuleViolation,
    RuleSeverity,
    default_refactor_rules,
};

// Pattern Detector
pub use pattern_detector::{
    PatternDetector,
    DetectedPattern,
    PatternKind,
    PatternSeverity,
};

// Risk Analyzer
pub use risk_analyzer::{
    RiskAnalyzer,
    RiskAssessment,
    RiskLevel,
};

// Evolution Predictor
pub use evolution_predictor::{
    EvolutionPredictor,
    EvolutionImpact,
};

// Git History (churn, commits, authorship)
pub use git_history::{
    GitHistoryAnalyzer,
    CommitSummary,
    FileChurn,
};

// Git backend (git2-based, read-only)
pub use git_backend_git2::GitBackendGit2;

// Repo Sync (local <-> remote, supervised)
pub use repo_sync::{
    RepoSyncEngine,
    PendingChange,
    SyncPlan,
    ChangeKind,
    SyncAction,
    SyncActionKind,
    RemoteRepoBackend,
    NoopRemoteRepoBackend,
};

// Publish Policy (what may be published to Syntra’s public repo)
pub use publish_policy::{
    PublishPolicy,
    PublishDecision,
    PublishScope,
    PublishEvaluation,
};

// GitHub remote backend
pub use remote_backend_github::{
    GitHubRemoteBackend,
    GitHubRemoteConfig,
    ContributorVerification,
};

// GitHub user verification & trust
pub use github_user_verification::{
    GitHubUserVerifier,
    GitHubVerificationConfig,
    GitHubUserVerification,
    TrustLevel,
};

// GitHub signature validation & tamper risk
pub use github_signature_validation::{
    SignatureValidator,
    SignatureValidationConfig,
    CommitSignatureReport,
    SignatureStatus,
    TamperRisk,
};

// Tamper monitor
pub use tamper_monitor::{
    TamperMonitor,
    TamperReport,
    IntegrityStatus,
    IntegrityAnomaly,
};

// Integrity daemon (watchdog)
pub use integrity_daemon::{
    IntegrityDaemon,
    SelfModMode,
};

// Self-modification gate
pub use self_mod_gate::{
    SelfModGate,
    SelfModRequest,
    SelfModDecision,
    SelfModDecisionKind,
    SelfModKind,
};
