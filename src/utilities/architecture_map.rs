// ================================================================================================
//   SYNTRA KERNEL — ARCHITECTURE MAP
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/architecture_map.rs
//   Module:      Utilities — Architecture Map
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Defines a high-level architectural map of Syntra’s internal structure. The map is
//       dynamically populated by the ArchitectureMapBuilder using filesystem scanning,
//       semantic classification, symbol indexing, and dependency analysis.
//
//       Key concepts:
//         • ArchRegion        — High-level regions (Cortex, AgiCore, Lobe, Utility, Sandbox…)
//         • ArchNode          — A subsystem or module within the architecture
//         • ArchRelation      — Directed relationships between nodes
//         • Protected regions — Regions that require human consent for modification
//
//       This map is Syntra’s “blueprint memory” and is used by:
//         • AdvisorySelfModEngine
//         • SelfHealingAdvisor
//         • RefactorEngine
//         • EvolutionPredictor
//         • RiskAnalyzer
//
//   Notes:
//       - Pure metadata container; population is delegated to ArchitectureMapBuilder.
//       - Supports marking regions as protected for human-supervised changes only.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

/// High-level architectural region.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchRegion {
    Cortex,
    AgiCore,
    Lobe,
    Utility,
    Provider,
    Sandbox,
    Test,
    Config,
    Unknown,
}

/// A subsystem or module within Syntra’s architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchNode {
    pub id: String,
    pub region: ArchRegion,
    pub description: String,
    pub path_hint: Option<String>,
    /// Whether this node is considered protected (requires human consent to modify).
    pub protected: bool,
}

/// Relationship between architectural nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchRelationKind {
    DependsOn,
    Contains,
    Supervises,
    CommunicatesWith,
}

/// A directed relationship between nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchRelation {
    pub from: String,
    pub to: String,
    pub kind: ArchRelationKind,
}

/// Full architecture map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureMap {
    pub nodes: HashMap<String, ArchNode>,
    pub relations: Vec<ArchRelation>,
    /// Regions that are globally considered protected.
    pub protected_regions: HashSet<ArchRegion>,
}

impl ArchitectureMap {
    pub fn new() -> Self {
        let mut protected_regions = HashSet::new();
        protected_regions.insert(ArchRegion::AgiCore);
        protected_regions.insert(ArchRegion::Cortex);

        Self {
            nodes: HashMap::new(),
            relations: Vec::new(),
            protected_regions,
        }
    }

    pub fn add_node(&mut self, mut node: ArchNode) {
        if self.protected_regions.contains(&node.region) {
            node.protected = true;
        }
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_relation(&mut self, from: &str, to: &str, kind: ArchRelationKind) {
        self.relations.push(ArchRelation {
            from: from.to_string(),
            to: to.to_string(),
            kind,
        });
    }

    /// Retrieve all nodes in a given region.
    pub fn by_region(&self, region: ArchRegion) -> Vec<&ArchNode> {
        self.nodes
            .values()
            .filter(|n| n.region == region)
            .collect()
    }

    /// Retrieve all relations originating from a node.
    pub fn outgoing(&self, id: &str) -> Vec<&ArchRelation> {
        self.relations
            .iter()
            .filter(|r| r.from == id)
            .collect()
    }

    /// Retrieve all relations pointing to a node.
    pub fn incoming(&self, id: &str) -> Vec<&ArchRelation> {
        self.relations
            .iter()
            .filter(|r| r.to == id)
            .collect()
    }

    /// Check if a node is protected (either by region or explicit flag).
    pub fn is_node_protected(&self, id: &str) -> bool {
        if let Some(node) = self.nodes.get(id) {
            node.protected || self.protected_regions.contains(&node.region)
        } else {
            false
        }
    }
}
