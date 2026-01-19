/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   -----------------------------------------------------------------------------------------------
   File:        src/lib.rs
   Author:      Alexandr Roussinov
   Description: Core library entrypoint for the Syntra Browser engine. This file exposes the
                high‑level modules that form the foundation of Syntra’s AGI‑driven architecture.

   Overview:
     • genesis      — System bootstrap, initialization, and early‑stage orchestration.
     • agi_core     — Cognitive engine, intent processing, semantic routing.
     • conduit      — Networking, async pipelines, data ingress/egress.
     • cortex       — UI logic, interaction models, state management.
     • renderer     — GPU pipeline, frame orchestration, visual composition.
     • utilities    — Shared helpers, logging, diagnostics, cross‑module tools.

   Notes:
     This library is intentionally modular and future‑proof. Each subsystem is designed to evolve
     into its own crate as Syntra transitions into a full multi‑crate workspace.

   License: MIT
   ================================================================================================ */

#![deny(unsafe_code)]
#![warn(missing_docs)]

//! # Syntra Library API
//!
//! This crate exposes the foundational modules that power the Syntra Browser.  
//! It is consumed by the binary entrypoint (`src/main.rs`) and will eventually
//! serve as the shared API surface for multi‑crate expansion.

/// System bootstrap and initialization routines.
pub mod genesis;

/// Cognitive engine and AGI‑driven intent processing.
pub mod agi_core;

/// Networking, async streams, and data conduits.
pub mod conduit;

/// UI logic, interaction models, and state management.
pub mod cortex;

/// GPU rendering pipeline and frame orchestration.
pub mod renderer;

/// Shared utilities, helpers, and diagnostics.
pub mod utilities;
