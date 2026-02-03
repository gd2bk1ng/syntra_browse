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
//       semantic filesystem classification, code indexing, and unified introspection.
//
//       These tools form the backbone of Axiom Zero and Axiom One — enabling self-analysis,
//       observability, introspection, and safe self-modification.
//
//   Overview:
//       - logging            — Human-readable & JSON-style structured logging.
//       - diagnostics        — Diagnostic event bus for internal health signals.
//       - ecosystem          — Filesystem introspection utilities for self-awareness.
//       - tracing            — Lightweight tracing hooks for internal instrumentation.
//       - evolution_log      — Records Syntra’s self-modification events.
//       - baseline_snapshot  — Syntra’s filesystem self-image for evolution & safety.
//       - fs_utils           — Safe reads/writes, atomic updates, sandbox copying.
//       - path_utils         — Cross-platform path normalization & matching.
//       - dependency_graph   — Structural view of crates/modules/dependencies.
//       - semantic_fs        — Semantic roles over the filesystem (lobes, core, utilities).
//       - code_index         — Symbol-level index for refactors and analysis.
//       - introspection      — Unified self-awareness/introspection hub.
//       - semantic_graph     — High-level semantic graph of Syntra’s architecture.
//       - refactor_engine    — Refactor planning and (future) application engine.
//       - pattern_detector   — Structural pattern and smell detector.
//       - risk_analyzer      — Risk scoring for modules, dependencies, and plans.
//       - evolution_predictor— Impact estimation for evolution and refactor plans.
//
//   Notes:
//       - Dependency-minimal and ASCII-safe for long-term stability.
//       - Provides the shared API surface for all higher-level lobes.
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
pub mod refactor_engine;
pub mod pattern_detector;
pub mod risk_analyzer;
pub mod evolution_predictor;

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

// Refactor Engine
pub use refactor_engine::{
    RefactorEngine,
    RefactorPlan,
    RefactorOperation,
    RefactorKind,
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

