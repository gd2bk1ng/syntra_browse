/* ================================================================================================
   SYNTRA BROWSER — AXIOM FIVE
   ------------------------------------------------------------------------------------------------
   File:        src/agi_core/ecosystem.rs
   Module:      AGI Core — Ecosystem Model
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Structural model of Syntra's ecosystem. Tracks major lobes (code, docs, terminals)
                and their presence/health.
   ================================================================================================ */

#![allow(dead_code)]

/// Represents a structural lobe in Syntra's ecosystem.
///
/// Lobes correspond to major filesystem or logical components such as source code,
/// documentation, or terminal interfaces. Each lobe tracks its presence status.
#[derive(Debug, Clone)]
pub struct EcosystemLobe {
    /// Name of the lobe (e.g., "src/agi_core", "docs").
    pub name: String,
    /// Filesystem path or identifier for the lobe.
    pub path: String,
    /// Whether the lobe currently exists or is accessible.
    pub present: bool,
}

/// High-level model of Syntra's ecosystem health.
///
/// Aggregates multiple lobes and provides methods to query the state of the ecosystem,
/// such as which lobes are missing or present.
#[derive(Debug, Clone, Default)]
pub struct EcosystemModel {
    /// Collection of ecosystem lobes representing the system structure.
    pub lobes: Vec<EcosystemLobe>,
}

impl EcosystemModel {
    /// Returns a list of lobes that are currently missing (not present).
    pub fn missing_lobes(&self) -> Vec<&EcosystemLobe> {
        self.lobes.iter().filter(|l| !l.present).collect()
    }

    /// Returns a list of lobes that are currently present.
    pub fn present_lobes(&self) -> Vec<&EcosystemLobe> {
        self.lobes.iter().filter(|l| l.present).collect()
    }
}
