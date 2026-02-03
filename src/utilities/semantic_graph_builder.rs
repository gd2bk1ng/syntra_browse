// ================================================================================================
//   SYNTRA KERNEL — SEMANTIC GRAPH BUILDER
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/semantic_graph_builder.rs
//   Module:      Utilities — Semantic Graph Builder
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Constructs Syntra’s high-level SemanticGraph by integrating:
//
//         • BaselineSnapshot      — raw filesystem structure
//         • SemanticFsView        — semantic roles (core, lobes, utilities, tests…)
//         • CodeIndex             — symbol-level definitions
//         • DependencyGraph       — module/crate dependency structure
//
//       This builder is the “connective tissue” that transforms raw introspection data into a
//       unified semantic graph Syntra can reason about. It is used by:
//
//         • AdvisorySelfModEngine
//         • SelfHealingAdvisor
//         • RefactorEngine
//         • EvolutionPredictor
//
//       The resulting graph is a living map of Syntra’s architecture.
//
//   Notes:
//       - Non-mutating; pure builder.
//       - Designed for incremental enrichment as Syntra evolves.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual-licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use crate::utilities::{
    BaselineSnapshot,
    SemanticFsView,
    SemanticRole,
    CodeIndex,
    DependencyGraph,
    SemanticGraph,
    SemanticNode,
    SemanticNodeKind,
    SemanticEdge,
    SemanticEdgeKind,
};

use std::path::Path;

/// Builder for constructing a SemanticGraph from introspection components.
#[derive(Debug)]
pub struct SemanticGraphBuilder;

impl SemanticGraphBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Build a full semantic graph from all introspection layers.
    pub fn build(
        &self,
        root: &Path,
        snapshot: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
    ) -> SemanticGraph {
        let mut graph = SemanticGraph::new();

        // 1) Add file nodes
        for file in &semantic.files {
            graph.add_node(SemanticNode {
                id: format!("file:{}", file.path),
                kind: SemanticNodeKind::File,
                label: file.path.clone(),
                path: Some(file.path.clone()),
            });

            // Add region/role edges
            let region_id = format!("region:{:?}", file.role);
            if !graph.nodes.contains_key(&region_id) {
                graph.add_node(SemanticNode {
                    id: region_id.clone(),
                    kind: SemanticNodeKind::Region,
                    label: format!("{:?}", file.role),
                    path: None,
                });
            }

            graph.add_edge(
                &region_id,
                &format!("file:{}", file.path),
                SemanticEdgeKind::Contains,
            );
        }

        // 2) Add symbol nodes
        for sym in index.all() {
            let id = format!("symbol:{}:{}", sym.kind as u32, sym.name);

            graph.add_node(SemanticNode {
                id: id.clone(),
                kind: match sym.kind {
                    crate::utilities::SymbolKind::Struct => SemanticNodeKind::Struct,
                    crate::utilities::SymbolKind::Enum => SemanticNodeKind::Struct,
                    crate::utilities::SymbolKind::Trait => SemanticNodeKind::Trait,
                    crate::utilities::SymbolKind::Function => SemanticNodeKind::Function,
                    _ => SemanticNodeKind::Module,
                },
                label: sym.name.clone(),
                path: Some(sym.file.clone()),
            });

            // Link symbol → file
            graph.add_edge(
                &format!("file:{}", sym.file),
                &id,
                SemanticEdgeKind::Contains,
            );
        }

        // 3) Add dependency edges
        for edge in &deps.edges {
            let from = format!("module:{}", edge.from);
            let to = format!("module:{}", edge.to);

            // Ensure nodes exist
            if !graph.nodes.contains_key(&from) {
                graph.add_node(SemanticNode {
                    id: from.clone(),
                    kind: SemanticNodeKind::Module,
                    label: edge.from.clone(),
                    path: None,
                });
            }
            if !graph.nodes.contains_key(&to) {
                graph.add_node(SemanticNode {
                    id: to.clone(),
                    kind: SemanticNodeKind::Module,
                    label: edge.to.clone(),
                    path: None,
                });
            }

            graph.add_edge(&from, &to, SemanticEdgeKind::DependsOn);
        }

        graph
    }
}

