// ================================================================================================
//   SYNTRA KERNEL — ARCHITECTURE MAP BUILDER
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/architecture_map_builder.rs
//   Module:      Utilities — Architecture Map Builder
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Dynamically constructs an ArchitectureMap from Syntra’s current state by integrating:
//
//         • BaselineSnapshot   — filesystem structure
//         • SemanticFsView     — semantic roles (AgiCore, Lobe, Utility, Sandbox…)
//         • CodeIndex          — symbol-level information
//         • DependencyGraph    — module/crate dependencies
//
//       The resulting map is fully dynamic, but regions like Cortex and AgiCore are marked as
//       protected. Any modifications to protected regions must be performed only with explicit
//       human consent and under human supervision.
//
//   Notes:
//       - Non-mutating; pure builder.
//       - Intended to be called by IntrospectionHub or higher-level lobes.
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
    ArchitectureMap,
    ArchNode,
    ArchRegion,
    ArchRelationKind,
};

use std::path::Path;

/// Builder for constructing a dynamic ArchitectureMap.
#[derive(Debug)]
pub struct ArchitectureMapBuilder;

impl ArchitectureMapBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(
        &self,
        root: &Path,
        snapshot: &BaselineSnapshot,
        semantic: &SemanticFsView,
        index: &CodeIndex,
        deps: &DependencyGraph,
    ) -> ArchitectureMap {
        let mut map = ArchitectureMap::new();

        // 1) Create nodes from semantic filesystem roles.
        for file in &semantic.files {
            let region = match file.role {
                SemanticRole::AgiCore => ArchRegion::AgiCore,
                SemanticRole::Utility => ArchRegion::Utility,
                SemanticRole::Sandbox => ArchRegion::Sandbox,
                SemanticRole::Test => ArchRegion::Test,
                SemanticRole::Config => ArchRegion::Config,
                SemanticRole::Lobe => ArchRegion::Lobe,
                _ => ArchRegion::Unknown,
            };

            let id = format!("file:{}", file.path);

            map.add_node(ArchNode {
                id,
                region,
                description: format!("File in region {:?}: {}", region, file.path),
                path_hint: Some(file.path.clone()),
                protected: false, // will be overridden by ArchitectureMap::add_node if region protected
            });
        }

        // 2) Add higher-level synthetic nodes for major regions if present.
        self.ensure_region_node(&mut map, ArchRegion::AgiCore, "agi_core", "AGI Core region");
        self.ensure_region_node(&mut map, ArchRegion::Utility, "utilities", "Utilities region");
        self.ensure_region_node(&mut map, ArchRegion::Sandbox, "sandbox", "Sandbox region");
        self.ensure_region_node(&mut map, ArchRegion::Test, "tests", "Tests region");

        // 3) Link files to their region nodes.
        for node in map.nodes.values() {
            if let Some(path) = &node.path_hint {
                if path.starts_with("src/agi_core") {
                    map.add_relation("agi_core", &node.id, ArchRelationKind::Contains);
                } else if path.starts_with("src/utilities") {
                    map.add_relation("utilities", &node.id, ArchRelationKind::Contains);
                } else if path.starts_with("tests") {
                    map.add_relation("tests", &node.id, ArchRelationKind::Contains);
                } else if path.starts_with("sandbox") {
                    map.add_relation("sandbox", &node.id, ArchRelationKind::Contains);
                }
            }
        }

        // 4) Use dependency graph to add high-level relations.
        for edge in &deps.edges {
            let from_id = format!("module:{}", edge.from);
            let to_id = format!("module:{}", edge.to);

            if !map.nodes.contains_key(&from_id) {
                map.add_node(ArchNode {
                    id: from_id.clone(),
                    region: ArchRegion::Unknown,
                    description: format!("Module {}", edge.from),
                    path_hint: None,
                    protected: false,
                });
            }

            if !map.nodes.contains_key(&to_id) {
                map.add_node(ArchNode {
                    id: to_id.clone(),
                    region: ArchRegion::Unknown,
                    description: format!("Module {}", edge.to),
                    path_hint: None,
                    protected: false,
                });
            }

            map.add_relation(&from_id, &to_id, ArchRelationKind::DependsOn);
        }

        // 5) Optionally, use CodeIndex to add synthetic nodes for major symbols (future expansion).

        map
    }

    fn ensure_region_node(
        &self,
        map: &mut ArchitectureMap,
        region: ArchRegion,
        id: &str,
        description: &str,
    ) {
        if !map.nodes.contains_key(id) {
            map.add_node(ArchNode {
                id: id.to_string(),
                region,
                description: description.to_string(),
                path_hint: None,
                protected: false,
            });
        }
    }
}

