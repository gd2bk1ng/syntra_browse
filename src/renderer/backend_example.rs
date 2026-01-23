/* ================================================================================================
   SYNTRA BROWSER — AXIOM THREE
   ------------------------------------------------------------------------------------------------
   File:        src/renderer/backend_example.rs
   Module:      Example GPU Backend
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: A simple example implementation of the RenderBackend trait that logs submissions.

   Notes:
     This backend does not perform actual GPU rendering but serves as a reference.
   ================================================================================================ */

use crate::renderer::{RenderBackend, graph::{RenderGraph, RenderNode}};

/// Example GPU backend that logs submitted render graphs.
pub struct ExampleBackend;

impl ExampleBackend {
    /// Creates a new ExampleBackend instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderBackend for ExampleBackend {
    fn submit(&mut self, graph: &RenderGraph) {
        println!("ExampleBackend received RenderGraph submission:");
        for node in &graph.nodes {
            println!(" - Node: {}", node.name);
        }
        // Here real GPU submission code would go.
    }
}
