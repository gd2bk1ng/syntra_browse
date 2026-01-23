/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/lib.rs
   Module:      Syntra Library Root
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Core library entrypoint for the Syntra Browser engine. This file exposes the
                high-level modules that form the foundation of Syntra's AGI-driven architecture.
                All subsystems are routed through this crate, making it the central API surface
                for both internal binaries and future multi-crate expansion.

   Overview:
     • genesis      - System bootstrap, initialization, and early-stage orchestration.
     • agi_core     - Cognitive engine, intent semantics, reasoning primitives.
     • conduit      - Inter-module communication, message routing, data ingress/egress.
     • cortex       - Interaction models, UI logic, state management.
     • renderer     - GPU pipeline, frame orchestration, visual composition.
     • utilities    - Shared helpers, diagnostics, logging, cross-module tools.

   Notes:
     - This library is intentionally modular and future-proof. Each subsystem is designed to
       evolve into its own crate as Syntra transitions into a full multi-crate workspace.
     - All modules exposed here form the stable public API for Syntra's internal binaries.
     - The root crate should remain ASCII-safe and dependency-light for long-term stability.
     - Additional subsystems (memory, simulation, distributed runtime) may be added over time.

   License: MIT
   ================================================================================================ */

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![deny(missing_docs)]

//! # Syntra Browser Library
//!
//! This crate provides the core API for the Syntra Browser project.
//! It includes modules for AGI-driven intent browsing, rendering, and more.
//!
//! # Examples
//!
//! ```rust
//! // Example usage here
//! ```

//! # Syntra Library API
//!
//! This crate exposes the foundational modules that power the Syntra Browser.
//! It is consumed by the binary entrypoint (`src/main.rs`) and will eventually
//! serve as the shared API surface for multi-crate expansion.

/// System bootstrap and initialization routines.
pub mod genesis;

/// Cognitive engine and AGI-driven intent processing.
pub mod agi_core;

/// Inter-module communication, message routing, and data conduits.
pub mod conduit;

/// UI logic, interaction models, and state management.
pub mod cortex;

/// GPU rendering pipeline and frame orchestration.
pub mod renderer;

/// Shared utilities, helpers, and diagnostics.
pub mod utilities;
