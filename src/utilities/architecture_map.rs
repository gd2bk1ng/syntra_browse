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
//       Provides a high-level architectural map of Syntra’s internal structure. This module
//       defines the conceptual regions, lobes, subsystems, and their relationships.
//
//       The ArchitectureMap is used by:
//         • SemanticGraphBuilder
//         • AdvisorySelfModEngine
//         • SelfHealingAdvisor
//         • EvolutionPredictor
//         • RiskAnalyzer
//
//       It acts as Syntra’s “blueprint memory,” enabling her to reason about:
//         • What each subsystem is responsible for
//         • How lobes relate to each other
//         • Which regions are protected or critical
//         • Where evolution is safe vs. high-risk
//
//   Notes:
//       - Pure metadata; no filesystem scanning.
//       - Designed for long-term stability and introspection.
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
use std::collections::HashMap;

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
}

impl ArchitectureMap {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ArchNode) {
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
}

/// Default architecture map for Syntra.
/// This is her “blueprint memory.”
pub fn default_architecture_map() -> ArchitectureMap {
    let mut map = ArchitectureMap::new();

    // Core regions
    map.add_node(ArchNode {
        id: "cortex".into(),
        region: ArchRegion::Cortex,
        description: "High-level orchestration, evolution cycles, self-healing.",
        path_hint: Some("src/agi_core/".into()),
    });

    map.add_node(ArchNode {
        id: "agi_core".into(),
        region: ArchRegion::AgiCore,
        description: "Unified cognition: intent, planning, reasoning, safety.",
        path_hint: Some("src/agi_core/".into()),
    });

    // Utilities
    map.add_node(ArchNode {
        id: "utilities".into(),
        region: ArchRegion::Utility,
        description: "Diagnostics, logging, snapshots, introspection, semantic graph.",
        path_hint: Some("src/utilities/".into()),
    });

    // Sandbox
    map.add_node(ArchNode {
        id: "sandbox".into(),
        region: ArchRegion::Sandbox,
        description: "Safe execution environment for evolution testing.",
        path_hint: Some("src/agi_core/sandbox/".into()),
    });

    // Relations
    map.add_relation("cortex", "agi_core", ArchRelationKind::DependsOn);
    map.add_relation("cortex", "utilities", ArchRelationKind::DependsOn);
    map.add_relation("cortex", "sandbox", ArchRelationKind::Supervises);
    map.add_relation("utilities", "agi_core", ArchRelationKind::CommunicatesWith);

    map
}

