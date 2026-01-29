// ================================================================================================
//   SYNTRA KERNEL — AXIOM SIX (ECOSYSTEM & EVOLUTION MODEL)
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/ecosystem_model.rs
//   Module:      Ecosystem Model
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Represents the Syntra Kernel ecosystem: lobes, modules, and their relationships.
//       Used by the self-mod engine, safety subsystem, and routing to reason about structure.
// ================================================================================================

#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;

/// A single lobe/module in the ecosystem.
#[derive(Debug, Clone)]
pub struct LobeDescriptor {
    /// Logical name of the lobe (e.g., "reasoner", "planner").
    pub name: String,
    /// Files that belong to this lobe.
    pub files: Vec<PathBuf>,
    /// Optional description of the lobe's role.
    pub description: Option<String>,
}

/// High-level model of the Syntra ecosystem.
#[derive(Debug, Clone, Default)]
pub struct EcosystemModel {
    /// Map from lobe name to descriptor.
    pub lobes: HashMap<String, LobeDescriptor>,
}

impl EcosystemModel {
    pub fn new() -> Self {
        Self {
            lobes: HashMap::new(),
        }
    }

    /// Register a lobe in the ecosystem.
    pub fn register_lobe(&mut self, lobe: LobeDescriptor) {
        self.lobes.insert(lobe.name.clone(), lobe);
    }

    /// Get a lobe by name.
    pub fn get_lobe(&self, name: &str) -> Option<&LobeDescriptor> {
        self.lobes.get(name)
    }

    /// Check if a lobe exists.
    pub fn has_lobe(&self, name: &str) -> bool {
        self.lobes.contains_key(name)
    }

    /// List all lobe names.
    pub fn lobe_names(&self) -> Vec<String> {
        self.lobes.keys().cloned().collect()
    }
}
