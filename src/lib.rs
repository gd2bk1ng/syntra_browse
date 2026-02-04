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
// ================================================================================================

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Syntra Kernel — Root Library
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
//! - **syntra_lang** — Syntra Language front-end (tokens, lexer, AST, types, errors).
//!   Future: parser, type checker, Syntra-IR, MLIR lowering.
//!
//! ## Utilities & Shared Systems
//! - **utilities** — Diagnostics, logging, tracing, and shared helpers.
//! - **browser** — Browser engine core and state management.
//! - **terminal** — Developer terminal interface.
//!
//! ## Advanced Subsystems
//! - **security**, **knowledge**, **predictive**, **continuity**,
//!   **diagnostics_ext**, **simulation**, **distributed**.

// Core systems
pub mod genesis;
pub mod agi_core;
pub mod conduit;
pub mod cortex;
pub mod renderer;
pub mod utilities;
pub mod terminal;
pub mod browser;
pub mod runtime;

// Syntra Language front-end
pub mod syntra_lang;

// Optional: crate-root self-mod orchestrator
pub mod self_mod_orchestrator;

// Advanced subsystems
pub mod security;
pub mod knowledge;
pub mod predictive;
pub mod continuity;
pub mod diagnostics_ext;
pub mod simulation;
pub mod distributed;

// Optional: re-exports for Syntra Language convenience
pub use syntra_lang::{
    Lexer,
    SyntraError,
    SyntraType,
    Token,
    TokenKind,
    Keyword,
    Module as SyntraModule,
    Item as SyntraItem,
    Function as SyntraFunction,
};
