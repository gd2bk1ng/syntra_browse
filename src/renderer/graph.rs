/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/renderer/graph.rs
   Module:      Render Graph Skeleton
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Minimal RenderGraph abstraction for future GPU-accelerated rendering in Syntra.

   Notes:
     - Axiom Three defines nodes and passes conceptually.
   ================================================================================================ */

/// Represents a GPU rendering pipeline graph composed of render nodes.
#[derive(Debug, Clone, Default)]
pub struct RenderGraph {
    /// Collection of render nodes that form the graph.
    pub nodes: Vec<RenderNode>,
}

impl RenderGraph {
    /// Creates a new empty RenderGraph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    /// Adds a render node to the graph.
    ///
    /// # Parameters
    ///
    /// - `node`: The `RenderNode` to add.
    pub fn add_node(&mut self, node: RenderNode) {
        self.nodes.push(node);
        println!("Added node '{}' to RenderGraph.", node.name);
    }
}

/// Represents a single node or pass in the render graph.
///
/// Each node may correspond to a rendering pass or stage.
#[derive(Debug, Clone)]
pub struct RenderNode {
    /// Name or identifier of the render node.
    pub name: String,
}

impl RenderNode {
    /// Creates a new RenderNode with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
