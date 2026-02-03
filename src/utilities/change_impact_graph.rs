// ================================================================================================
//   SYNTRA KERNEL — CHANGE IMPACT GRAPH
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/change_impact_graph.rs
//   Module:      Utilities — Change Impact Graph
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Models the ripple effects of changes across Syntra’s architecture. This graph is used to
//       understand how modifying a file, symbol, or module will propagate through dependent
//       components.
//
//       Inputs:
//         • DependencyGraph
//         • SemanticGraph
//         • ArchitectureMap
//         • CodeIndex
//
//       Outputs:
//         • ImpactGraph — nodes + edges representing change propagation
//         • ImpactScore — heuristic severity of ripple effects
//
//       Used by:
//         • EvolutionPredictor
//         • RefactorEngine
//         • SelfHealingAdvisor
//         • RiskAnalyzer
//
//   Notes:
//       - Non-mutating; pure analysis.
//       - Designed for future AST-level precision.
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

use crate::utilities::{
    DependencyGraph,
    SemanticGraph,
    ArchitectureMap,
};

/// A node in the impact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactNode {
    pub id: String,
    pub label: String,
}

/// A directed impact edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactEdge {
    pub from: String,
    pub to: String,
    pub weight: f32, // heuristic impact weight
}

/// Full change impact graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactGraph {
    pub nodes: HashMap<String, ImpactNode>,
    pub edges: Vec<ImpactEdge>,
}

/// High-level impact score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactScore {
    pub subject: String,
    pub estimated_ripple_size: usize,
    pub severity: String,
}

/// Change impact analyzer.
#[derive(Debug)]
pub struct ChangeImpactAnalyzer;

impl ChangeImpactAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Build an impact graph for a given module or file.
    pub fn build_impact_graph(
        &self,
        subject: &str,
        deps: &DependencyGraph,
        semantic: &SemanticGraph,
        arch: &ArchitectureMap,
    ) -> ImpactGraph {
        let mut graph = ImpactGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        };

        // Add subject node
        graph.nodes.insert(
            subject.to_string(),
            ImpactNode {
                id: subject.to_string(),
                label: subject.to_string(),
            },
        );

        // 1) Dependency-based propagation
        for edge in &deps.edges {
            if edge.from == subject {
                let id = format!("module:{}", edge.to);
                graph.nodes.insert(
                    id.clone(),
                    ImpactNode {
                        id: id.clone(),
                        label: edge.to.clone(),
                    },
                );
                graph.edges.push(ImpactEdge {
                    from: subject.to_string(),
                    to: id,
                    weight: 1.0,
                });
            }
        }

        // 2) Semantic graph propagation
        for edge in &semantic.edges {
            if edge.from == subject {
                graph.nodes.insert(
                    edge.to.clone(),
                    ImpactNode {
                        id: edge.to.clone(),
                        label: edge.to.clone(),
                    },
                );
                graph.edges.push(ImpactEdge {
                    from: subject.to_string(),
                    to: edge.to.clone(),
                    weight: 0.5,
                });
            }
        }

        // 3) Architecture-level propagation
        for rel in &arch.relations {
            if rel.from == subject {
                graph.nodes.insert(
                    rel.to.clone(),
                    ImpactNode {
                        id: rel.to.clone(),
                        label: rel.to.clone(),
                    },
                );
                graph.edges.push(ImpactEdge {
                    from: subject.to_string(),
                    to: rel.to.clone(),
                    weight: 0.8,
                });
            }
        }

        graph
    }

    /// Compute a high-level impact score.
    pub fn score(&self, graph: &ImpactGraph, subject: &str) -> ImpactScore {
        let ripple = graph.nodes.len().saturating_sub(1);

        let severity = if ripple > 50 {
            "high"
        } else if ripple > 10 {
            "medium"
        } else {
            "low"
        };

        ImpactScore {
            subject: subject.to_string(),
            estimated_ripple_size: ripple,
            severity: severity.to_string(),
        }
    }
}

