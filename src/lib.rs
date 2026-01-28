// ================================================================================================
//   SYNTRA KERNEL — ROOT LIBRARY ENTRYPOINT
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/lib.rs
//   Module:      Syntra Kernel — Library Root
//   Description: Primary public API surface for the Syntra Kernel. This module exposes the core
//                subsystems that define Syntra’s cognitive architecture, runtime orchestration,
//                compiler components, safety systems, predictive engines, and rendering pipeline.
//
//   Design Philosophy:
//     - Strict modularity with long-term maintainability.
//     - Zero unsafe code; memory-safe by design.
//     - Clear subsystem boundaries with explicit public interfaces.
//     - Architecture intended for multi-crate workspace evolution.
//     - Emphasis on observability, safety, and cognitive clarity.
//     - Future-proof: ASCII-safe, dependency-light, and workspace-ready.
//
//   Notes:
//     - All Rust files in Syntra Kernel follow the same banner format.
//     - This crate acts as the stable API surface for internal binaries and external tools.
//     - New advanced modules added: security, knowledge, predictive, continuity,
//       diagnostics_ext, simulation, distributed.
// ================================================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Syntra Kernel — Root Library
//!
//! The Syntra Kernel is a cognitive-first runtime and compiler environment designed around
//! AGI-aligned principles, modular subsystems, and long-term architectural clarity.
//!
//! This crate exposes the major subsystems that form the backbone of the Syntra ecosystem:
//!
//! ## Core Cognitive & Runtime Systems
//! - **genesis** — System bootstrap, initialization, and orchestration.
//! - **agi_core** — Cognitive engine, intent semantics, and reasoning primitives.
//! - **runtime** — Execution environment for cognitive processes and Syntra programs.
//! - **conduit** — Inter-module communication and message routing.
//! - **cortex** — UI logic, interaction models, and cognitive state management.
//! - **renderer** — GPU pipeline, frame orchestration, and visual composition.
//!
//! ## Compiler Pipeline
//! - **tokens**, **lexer**, **parser**, **ast**, **type_checker**, **errors**, **types** —
//!   Syntra Reference Compiler pipeline.
//!
//! ## Utilities & Shared Systems
//! - **utilities** — Diagnostics, logging, tracing, and shared helpers.
//! - **dataset** — Dataset management and structured storage.
//! - **browser** — Browser engine core and state management.
//! - **terminal** — Developer terminal interface.
//!
//! ## Newly Added Advanced Subsystems
//!
//! These modules elevate Syntra Kernel into a modern AGI-grade cognitive runtime:
//!
//! - **security** — Capability-based safety, sandboxing, and intent-governed permissions.
//! - **knowledge** — Semantic memory, embeddings, vector search, and knowledge graph.
//! - **predictive** — Forecasting engine, temporal reasoning, and pattern modeling.
//! - **continuity** — Long-term state persistence, episodic memory, and session stitching.
//! - **diagnostics_ext** — Advanced profiling, telemetry, and performance analytics.
//! - **simulation** — Sandboxed world simulation and agent-based modeling.
//! - **distributed** — Multi-node runtime, federated cognition, distributed intent resolution.
//!
//! ## Stability
//!
//! This crate is the stable public API surface for the Syntra Kernel. Internal binaries,
//! external tools, and future crates depend on this module layout remaining predictable.

/// System bootstrap and initialization routines.
pub mod genesis;

/// Cognitive engine and AGI-driven intent processing.
pub mod agi_core;

/// Inter-module communication and message routing.
pub mod conduit;

/// UI logic, interaction models, and cognitive state management.
pub mod cortex;

/// GPU rendering pipeline and frame orchestration.
pub mod renderer;

/// Shared utilities, diagnostics, logging, and tracing.
pub mod utilities;

/// Terminal interface and command handling.
pub mod terminal;

/// Token definitions for the Syntra language.
pub mod tokens;

/// Lexical scanner for Syntra language.
pub mod lexer;

/// Abstract Syntax Tree (AST) structures.
pub mod ast;

/// Type definitions and system types.
pub mod types;

/// Error handling and diagnostics.
pub mod errors;

/// Parser implementation for Syntra language.
pub mod parser;

/// Type checker for Syntra language.
pub mod type_checker;

/// Dataset management and structured storage.
pub mod dataset;

/// Browser engine core and state management.
pub mod browser;

/// Runtime environment and execution engine.
pub mod runtime;

// ================================================================================================
// New Advanced Subsystems
// ================================================================================================

/// Security, sandboxing, and capability-based permission systems.
pub mod security;

/// Semantic memory, embeddings, vector search, and knowledge graph.
pub mod knowledge;

/// Forecasting engine, temporal reasoning, and predictive modeling.
pub mod predictive;

/// Long-term state persistence, episodic memory, and continuity engine.
pub mod continuity;

/// Advanced diagnostics, profiling, telemetry, and performance analytics.
pub mod diagnostics_ext;

/// Sandboxed world simulation and agent-based modeling.
pub mod simulation;

/// Distributed runtime, federated cognition, and multi-node orchestration.
pub mod distributed;
