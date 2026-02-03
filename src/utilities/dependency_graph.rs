// ================================================================================================
//   SYNTRA KERNEL — DEPENDENCY GRAPH
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/dependency_graph.rs
//   Module:      Utilities — Dependency Graph
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides a structural view of Syntra's dependencies (internal modules and external crates).
//       This is used for:
//         • Identifying fragile or high-risk external dependencies.
//         • Finding candidates for internalization (self-hosted replacements).
//         • Understanding coupling between lobes, utilities, and core modules.
//         • Guiding evolution plans that reduce external surface area.
//
//   Notes:
//       - Initial implementation is metadata-oriented and non-invasive.
//       - Designed to be fed by `cargo metadata` or static analysis in future iterations.
//       - MIT & Apache 2.0 dual-licensed.
// ================================================================================================

#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

/// Kind of dependency node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DependencyKind {
    InternalModule,
    InternalCrate,
    ExternalCrate,
}

/// A single node in the dependency graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub name: String,
    pub kind: DependencyKind,
    /// Optional version (for crates).
    pub version: Option<String>,
    /// Optional path (for internal modules/crates).
    pub path: Option<String>,
}

/// A directed edge: `from` depends on `to`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
}

/// Full dependency graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyNode) {
        self.nodes.insert(node.name.clone(), node);
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.edges.push(DependencyEdge {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    /// Returns all direct dependencies of a given node.
    pub fn dependencies_of(&self, name: &str) -> Vec<&DependencyNode> {
        let mut out = Vec::new();
        for edge in &self.edges {
            if edge.from == name {
                if let Some(node) = self.nodes.get(&edge.to) {
                    out.push(node);
                }
            }
        }
        out
    }

    /// Returns all nodes that depend on the given node.
    pub fn dependents_of(&self, name: &str) -> Vec<&DependencyNode> {
        let mut out = Vec::new();
        for edge in &self.edges {
            if edge.to == name {
                if let Some(node) = self.nodes.get(&edge.from) {
                    out.push(node);
                }
            }
        }
        out
    }

    /// Identify external crates that are heavily depended on (hotspots).
    pub fn external_hotspots(&self, threshold: usize) -> Vec<&DependencyNode> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for edge in &self.edges {
            *counts.entry(edge.to.clone()).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        for (name, count) in counts {
            if count >= threshold {
                if let Some(node) = self.nodes.get(&name) {
                    if node.kind == DependencyKind::ExternalCrate {
                        result.push(node);
                    }
                }
            }
        }
        result
    }

    /// Detect simple cycles (high-level; can be refined later).
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        fn dfs(
            graph: &DependencyGraph,
            current: &str,
            stack: &mut Vec<String>,
            visited: &mut HashSet<String>,
            cycles: &mut Vec<Vec<String>>,
        ) {
            if stack.contains(&current.to_string()) {
                // Found a cycle
                if let Some(pos) = stack.iter().position(|n| n == current) {
                    cycles.push(stack[pos..].to_vec());
                }
                return;
            }

            if !visited.insert(current.to_string()) {
                return;
            }

            stack.push(current.to_string());
            for dep in graph.dependencies_of(current) {
                dfs(graph, &dep.name, stack, visited, cycles);
            }
            stack.pop();
        }

        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        for name in self.nodes.keys() {
            dfs(self, name, &mut stack, &mut visited, &mut cycles);
        }

        cycles
    }
}

