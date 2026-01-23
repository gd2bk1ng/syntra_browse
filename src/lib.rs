/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO & THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/lib.rs
   Module:      Syntra Library Root
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Core library entrypoint for the Syntra Browser engine and Syntra Reference Compiler.
                This crate exposes foundational modules that form the backbone of Syntra's AGI-driven
                architecture, compiler components, runtime, and rendering pipeline.

   Design Philosophy:
     - Modular and future-proof architecture aimed at multi-crate workspace evolution.
     - ASCII-safe and dependency-light root crate for long-term stability.
     - Emphasis on observability, safety, and composability.
     - Stable public API surface for internal binaries and external tools.
     - Potential future subsystems: memory, simulation, distributed runtime.

   License: MIT
   ================================================================================================ */

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Syntra Browser Library
//!
//! This crate provides the core API surface for the Syntra Browser project,
//! including modules for AGI-driven intent browsing, rendering, compiler tooling,
//! runtime management, and more.
//!
//! ## Overview of Modules
//!
//! - **genesis**: System bootstrap, initialization, and orchestration.
//! - **agi_core**: Cognitive engine with intent semantics and reasoning primitives.
//! - **conduit**: Inter-module communication, message routing, and data conduits.
//! - **cortex**: UI logic, interaction models, and state management.
//! - **renderer**: GPU pipeline, frame orchestration, and visual composition.
//! - **utilities**: Shared helpers, diagnostics, logging, and cross-module tools.
//! - **terminal, tokens, lexer, ast, types, errors, parser, type_checker, dataset, browser, runtime**:
//!   Compiler and runtime components for Syntra Reference Compiler and Browser.
//!
//! ## Example
//!
//! ```rust
//! // Example: Initialize the system and start the main event loop.
//! use syntra::genesis::initialize_system;
//! use syntra::cortex::ui::start_ui;
//!
//! fn main() {
//!     initialize_system();
//!     start_ui();
//! }
//! ```
//!
//! ## Notes
//!
//! The root crate is designed to be lightweight and dependency-minimal to ensure
//! stability and ease of maintenance as the project grows.
//!
//! Future iterations will likely split this crate into multiple crates for better
//! modularity and independent evolution.

/// System bootstrap and initialization routines.
///
/// This module handles the early-stage orchestration, configuration loading,
/// and overall system startup procedures.
///
/// # Example
///
/// ```rust
/// use syntra::genesis::initialize_system;
///
/// fn main() {
///     initialize_system();
///     println!("System initialized successfully.");
/// }
/// ```
pub mod genesis;

/// Cognitive engine and AGI-driven intent processing.
///
/// Contains core reasoning primitives, intent semantics, and AI-driven
/// decision-making logic.
///
/// # Example
///
/// ```rust
/// use syntra::agi_core::IntentProcessor;
///
/// let mut processor = IntentProcessor::new();
/// let result = processor.process_intent("browse for sustainable products");
/// println!("Intent processed: {:?}", result);
/// ```
pub mod agi_core;

/// Inter-module communication, message routing, and data conduits.
///
/// Provides channels and protocols for message passing between subsystems,
/// ensuring decoupled and efficient communication.
///
/// # Example
///
/// ```rust
/// use syntra::conduit::{MessageBus, Message};
///
/// let bus = MessageBus::new();
/// bus.send(Message::new("UpdateUI", "Refresh screen"));
/// ```
pub mod conduit;

/// UI logic, interaction models, and state management.
///
/// Manages user interface components, event handling, and application state.
///
/// # Example
///
/// ```rust
/// use syntra::cortex::ui::{start_ui, UIState};
///
/// fn main() {
///     start_ui();
///     let state = UIState::default();
///     println!("UI started with state: {:?}", state);
/// }
/// ```
pub mod cortex;

/// GPU rendering pipeline and frame orchestration.
///
/// Manages rendering tasks including frame composition, GPU resource management,
/// and visual output.
///
/// # Example
///
/// ```rust
/// use syntra::renderer::Renderer;
///
/// let mut renderer = Renderer::new();
/// renderer.render_frame();
/// ```
pub mod renderer;

/// Shared utilities, helpers, and diagnostics.
///
/// Contains logging utilities, error reporting helpers, and common tools shared
/// across modules.
///
/// # Example
///
/// ```rust
/// use syntra::utilities::logger::log_info;
///
/// log_info("Syntra Browser started.");
/// ```
pub mod utilities;

/// Terminal interface and command handling.
///
/// Provides terminal input/output and command parsing for developer interaction.
pub mod terminal;

/// Token definitions and lexical analysis.
///
/// Defines language tokens and lexical scanning logic.
pub mod tokens;

/// Lexer implementation for Syntra language.
///
/// Parses raw input into tokens.
pub mod lexer;

/// Abstract Syntax Tree (AST) structures.
///
/// Defines syntax tree nodes representing parsed code.
pub mod ast;

/// Type definitions and system types.
///
/// Contains type system definitions and utilities.
pub mod types;

/// Error handling and diagnostics.
///
/// Centralized error types and diagnostic messages.
pub mod errors;

/// Parser implementation.
///
/// Parses tokens into AST nodes.
pub mod parser;

/// Type checker for Syntra language.
///
/// Validates type correctness in AST.
pub mod type_checker;

/// Dataset management and storage.
///
/// Manages datasets used by the browser and runtime.
pub mod dataset;

/// Browser engine core.
///
/// Contains core browser logic and state management.
pub mod browser;

/// Runtime environment and execution.
///
/// Manages runtime execution of Syntra code and processes.
pub mod runtime;
